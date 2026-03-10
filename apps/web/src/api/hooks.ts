import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import {
  bootstrapAdmin,
  createCollection,
  createSource,
  getBootstrapStatus,
  getEntryDetail,
  getSession,
  getSettings,
  listCollections,
  listEntries,
  listSources,
  loginAdmin,
  logoutAdmin,
  queueEntryTask,
  saveSettings,
  searchEntries,
  updateEntryState,
} from "@/api/client";

const entryKeys = {
  all: ["entries"] as const,
  detail: (entryId: string) => ["entries", entryId] as const,
};

export const useBootstrapStatus = () =>
  useQuery({ queryKey: ["bootstrap-status"], queryFn: getBootstrapStatus });

export const useSession = (enabled = true) =>
  useQuery({ queryKey: ["session"], queryFn: getSession, retry: false, enabled });

export const useEntries = (params: URLSearchParams, enabled = true) =>
  useQuery({
    queryKey: [...entryKeys.all, params.toString()],
    queryFn: () => listEntries(params),
    enabled,
  });

export const useEntryDetail = (entryId: string, enabled = true) =>
  useQuery({
    queryKey: entryKeys.detail(entryId),
    queryFn: () => getEntryDetail(entryId),
    enabled: enabled && Boolean(entryId),
  });

export const useCollections = (enabled = true) =>
  useQuery({ queryKey: ["collections"], queryFn: listCollections, enabled });

export const useSources = (enabled = true) =>
  useQuery({ queryKey: ["sources"], queryFn: listSources, enabled });

export const useSettings = (enabled = true) =>
  useQuery({ queryKey: ["settings"], queryFn: getSettings, enabled });

export const useSearch = (query: string, enabled = true) =>
  useQuery({
    queryKey: ["search", query],
    queryFn: () => searchEntries(query),
    enabled: enabled && query.trim().length > 0,
  });

export function useAuthActions() {
  const queryClient = useQueryClient();
  return {
    bootstrap: useMutation({
      mutationFn: bootstrapAdmin,
      onSuccess: (session) => {
        queryClient.setQueryData(["bootstrap-status"], { bootstrapped: true });
        queryClient.setQueryData(["session"], session);
      },
    }),
    login: useMutation({
      mutationFn: loginAdmin,
      onSuccess: (session) => {
        queryClient.setQueryData(["session"], session);
      },
    }),
    logout: useMutation({
      mutationFn: logoutAdmin,
      onSuccess: () => {
        queryClient.removeQueries({ queryKey: ["session"] });
      },
    }),
  };
}

export function useCreateCollection() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: createCollection,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["collections"] }),
  });
}

export function useCreateSource() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: createSource,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["sources"] });
      queryClient.invalidateQueries({ queryKey: entryKeys.all });
    },
  });
}

export function useSaveSettings() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: saveSettings,
    onSuccess: (settings) => queryClient.setQueryData(["settings"], settings),
  });
}

export function useEntryActions(entryId: string) {
  const queryClient = useQueryClient();
  return {
    updateState: useMutation({
      mutationFn: (body: { read?: boolean; starred?: boolean; saved?: boolean }) =>
        updateEntryState(entryId, body),
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: entryKeys.all });
        queryClient.invalidateQueries({ queryKey: entryKeys.detail(entryId) });
      },
    }),
    queueTask: useMutation({
      mutationFn: (taskType: string) => queueEntryTask(entryId, taskType),
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: entryKeys.detail(entryId) });
      },
    }),
  };
}
