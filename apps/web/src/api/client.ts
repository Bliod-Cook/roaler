import type {
  AIJob,
  AuthResponse,
  BootstrapStatus,
  CollectionWithCount,
  EntryDetail,
  EntrySummary,
  SearchResponse,
  SourceView,
  SystemSettings,
} from "@/api/types";

const API_BASE = "/api";

type ApiOptions = RequestInit & {
  bodyJson?: unknown;
};

export async function apiFetch<T>(path: string, options: ApiOptions = {}) {
  const response = await fetch(`${API_BASE}${path}`, {
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      ...(options.headers ?? {}),
    },
    ...options,
    body: options.bodyJson ? JSON.stringify(options.bodyJson) : options.body,
  });
  if (!response.ok) {
    const errorBody = await response.json().catch(() => ({ message: response.statusText }));
    throw new Error(errorBody.message ?? response.statusText);
  }
  if (response.status === 204) {
    return undefined as T;
  }
  return (await response.json()) as T;
}

export const getBootstrapStatus = () => apiFetch<BootstrapStatus>("/auth/bootstrap-status");
export const getSession = () => apiFetch<AuthResponse>("/auth/me");
export const bootstrapAdmin = (body: {
  email: string;
  password: string;
  display_name: string;
}) => apiFetch<AuthResponse>("/auth/bootstrap", { method: "POST", bodyJson: body });
export const loginAdmin = (body: { email: string; password: string }) =>
  apiFetch<AuthResponse>("/auth/login", { method: "POST", bodyJson: body });
export const logoutAdmin = () => apiFetch("/auth/logout", { method: "POST" });

export const listEntries = (params: URLSearchParams) =>
  apiFetch<EntrySummary[]>(`/entries?${params.toString()}`);
export const getEntryDetail = (entryId: string) => apiFetch<EntryDetail>(`/entries/${entryId}`);
export const updateEntryState = (
  entryId: string,
  body: { read?: boolean; starred?: boolean; saved?: boolean },
) =>
  apiFetch(`/entries/${entryId}/state`, {
    method: "PATCH",
    bodyJson: {
      is_read: body.read,
      is_starred: body.starred,
      is_saved: body.saved,
    },
  });

export const listCollections = () => apiFetch<CollectionWithCount[]>("/collections");
export const createCollection = (body: {
  name: string;
  accent_color: string;
}) => apiFetch("/collections", { method: "POST", bodyJson: body });

export const listSources = () => apiFetch<SourceView[]>("/sources");
export const createSource = (body: {
  kind: string;
  feed_url?: string;
  site_url?: string;
  title?: string;
  rsshub_base_url?: string;
  rsshub_route?: string;
  collection_ids: string[];
  refresh_interval_minutes?: number;
}) => apiFetch("/sources", { method: "POST", bodyJson: body });

export const searchEntries = async (query: string) => {
  const response = await apiFetch<SearchResponse>(`/search?query=${encodeURIComponent(query)}`);
  return response.items;
};

export const getSettings = () => apiFetch<SystemSettings>("/system/settings");
export const saveSettings = (body: SystemSettings) =>
  apiFetch<SystemSettings>("/system/settings", { method: "PUT", bodyJson: body });

export const queueEntryTask = (entryId: string, taskType: string) => {
  if (taskType === "entry_summary") {
    return apiFetch<AIJob>(`/ai/entries/${entryId}/summary`, { method: "POST" });
  }
  if (taskType === "entry_translation") {
    return apiFetch<AIJob>(`/ai/entries/${entryId}/translation`, {
      method: "POST",
      bodyJson: { target_language: "中文" },
    });
  }
  return apiFetch<AIJob>(`/ai/entries/${entryId}/tags`, { method: "POST" });
};

