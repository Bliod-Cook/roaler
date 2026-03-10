create extension if not exists pg_trgm;

do $$
begin
  create type source_kind as enum ('rss', 'atom', 'jsonfeed', 'rsshub');
exception
  when duplicate_object then null;
end $$;

do $$
begin
  create type sync_status as enum ('running', 'success', 'failed');
exception
  when duplicate_object then null;
end $$;

do $$
begin
  create type content_status as enum ('pending', 'ready', 'failed');
exception
  when duplicate_object then null;
end $$;

do $$
begin
  create type ai_task_type as enum ('entry_summary', 'entry_translation', 'entry_topic_tags', 'collection_digest');
exception
  when duplicate_object then null;
end $$;

do $$
begin
  create type ai_job_status as enum ('pending', 'running', 'success', 'failed');
exception
  when duplicate_object then null;
end $$;

create table if not exists admin_users (
  id uuid primary key,
  email text not null unique,
  display_name text not null,
  password_hash text not null,
  created_at timestamptz not null default now()
);

create table if not exists admin_sessions (
  id uuid primary key,
  user_id uuid not null references admin_users(id) on delete cascade,
  token_hash text not null unique,
  expires_at timestamptz not null,
  created_at timestamptz not null default now()
);

create table if not exists admin_settings (
  key text primary key,
  value_json jsonb not null,
  updated_at timestamptz not null default now()
);

create table if not exists collections (
  id uuid primary key,
  name text not null,
  slug text not null unique,
  accent_color text not null,
  last_digest text,
  last_digest_at timestamptz,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table if not exists feed_sources (
  id uuid primary key,
  kind source_kind not null,
  title text not null,
  description text,
  site_url text,
  feed_url text not null unique,
  rsshub_base_url text,
  rsshub_route text,
  hub_url text,
  etag text,
  last_modified text,
  last_synced_at timestamptz,
  last_error text,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table if not exists subscriptions (
  id uuid primary key,
  source_id uuid not null unique references feed_sources(id) on delete cascade,
  refresh_interval_minutes integer not null,
  next_sync_at timestamptz,
  paused boolean not null default false,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table if not exists subscription_collections (
  subscription_id uuid not null references subscriptions(id) on delete cascade,
  collection_id uuid not null references collections(id) on delete cascade,
  primary key (subscription_id, collection_id)
);

create table if not exists entries (
  id uuid primary key,
  source_id uuid not null references feed_sources(id) on delete cascade,
  external_id text,
  guid text,
  dedupe_key text not null unique,
  title text not null,
  summary text,
  url text not null,
  author text,
  published_at timestamptz not null,
  media_json jsonb not null default '[]'::jsonb,
  raw_payload jsonb not null default '{}'::jsonb,
  content_hash text not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table if not exists entry_contents (
  entry_id uuid primary key references entries(id) on delete cascade,
  content_html text,
  text_content text,
  ai_summary text,
  ai_translation text,
  ai_tags jsonb not null default '[]'::jsonb,
  content_status content_status not null default 'pending',
  content_error text,
  search_document text not null default '',
  updated_at timestamptz not null default now()
);

create table if not exists entry_states (
  entry_id uuid primary key references entries(id) on delete cascade,
  is_read boolean not null default false,
  is_starred boolean not null default false,
  is_saved boolean not null default false,
  updated_at timestamptz not null default now()
);

create table if not exists sync_runs (
  id uuid primary key,
  source_id uuid not null references feed_sources(id) on delete cascade,
  trigger_kind text not null,
  status sync_status not null,
  fetched_count integer not null,
  inserted_count integer not null,
  started_at timestamptz not null default now(),
  finished_at timestamptz,
  failed_reason text
);

create table if not exists webhook_subscriptions (
  id uuid primary key,
  source_id uuid not null unique references feed_sources(id) on delete cascade,
  hub_url text not null,
  topic_url text not null,
  secret text not null,
  verified_at timestamptz,
  expires_at timestamptz,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table if not exists ai_jobs (
  id uuid primary key,
  entry_id uuid references entries(id) on delete cascade,
  collection_id uuid references collections(id) on delete cascade,
  task_type ai_task_type not null,
  status ai_job_status not null,
  model text not null,
  prompt_version text not null,
  attempts integer not null default 0,
  cost_usd double precision,
  error_message text,
  output_json jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index if not exists idx_entries_source_published on entries(source_id, published_at desc);
create index if not exists idx_entries_published on entries(published_at desc);
create index if not exists idx_entry_contents_search on entry_contents using gin (to_tsvector('simple', search_document));
create index if not exists idx_entries_title_trgm on entries using gin (title gin_trgm_ops);
create index if not exists idx_subscriptions_next_sync on subscriptions(next_sync_at);
