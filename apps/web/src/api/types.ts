export type BootstrapStatus = {
  bootstrapped: boolean;
};

export type SessionUser = {
  id: string;
  email: string;
  display_name: string;
  created_at: string;
};

export type AuthResponse = {
  user: SessionUser;
};

export type SourceKind = "rss" | "atom" | "jsonfeed" | "rsshub";

export type SourceView = {
  id: string;
  kind: SourceKind;
  title: string;
  site_url: string | null;
  feed_url: string;
  hub_url: string | null;
  last_error: string | null;
  last_synced_at: string | null;
  refresh_interval_minutes: number;
  next_sync_at: string;
  paused: boolean;
  collection_ids: string[];
};

export type CollectionWithCount = {
  id: string;
  name: string;
  slug: string;
  accent_color: string;
  last_digest_at: string | null;
  source_count: number;
  unread_count: number;
};

export type EntrySummary = {
  id: string;
  source_id: string;
  source_title: string;
  title: string;
  summary: string | null;
  url: string | null;
  published_at: string | null;
  is_read: boolean;
  is_starred: boolean;
  is_saved: boolean;
  ai_summary: string | null;
  media_json: unknown;
};

export type EntryDetail = EntrySummary & {
  raw_payload: unknown;
  author_name: string | null;
  html_content: string | null;
  text_content: string | null;
  ai_translation: string | null;
  ai_tags: unknown;
  content_status: "pending" | "ready" | "failed";
  content_error: string | null;
};

export type SystemSettings = {
  public_base_url: string | null;
  default_rsshub_base_url: string;
  ai: {
    base_url: string;
    api_key: string;
    model: string;
    timeout_seconds: number;
  } | null;
};

export type AIJob = {
  id: string;
  entry_id: string | null;
  collection_id: string | null;
  task_type: "entry_summary" | "entry_translation" | "entry_topic_tags" | "collection_digest";
  status: "pending" | "running" | "succeeded" | "failed";
  model: string;
  prompt_version: string;
  attempts: number;
  cost_usd: number | null;
  error_message: string | null;
  output_json: unknown;
  created_at: string;
  updated_at: string;
};

export type SearchResponse = {
  items: EntrySummary[];
};

