alter table admin_settings
  add column if not exists updated_at timestamptz not null default now();

alter table collections
  add column if not exists last_digest text,
  add column if not exists last_digest_at timestamptz;

alter table subscriptions
  add column if not exists paused boolean not null default false;
