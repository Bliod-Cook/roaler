import { useMemo } from "react";

import { useEntries, useEntryActions, useEntryDetail, useSession } from "@/api/hooks";
import { EntryDetailPanel } from "@/features/entries/entry-detail-panel";
import { TimelinePane } from "@/features/entries/timeline-pane";

export function ReaderWorkspace({ entryId }: { entryId?: string }) {
  const params = useMemo(() => new URLSearchParams(), []);
  const session = useSession(false);
  const isAuthenticated = Boolean(session.data?.user);
  const entries = useEntries(params, isAuthenticated);
  const detail = useEntryDetail(entryId ?? "", isAuthenticated && Boolean(entryId));
  const actions = useEntryActions(entryId ?? "");

  return (
    <div className="grid gap-4 xl:grid-cols-[minmax(360px,0.78fr)_minmax(460px,1.22fr)]">
      <TimelinePane activeEntryId={entryId} entries={entries.data} />
      <EntryDetailPanel
        busy={actions.queueTask.isPending || actions.updateState.isPending}
        detail={detail.data}
        hasSelection={Boolean(entryId)}
        onQueueTask={(taskType) => {
          if (!entryId) {
            return;
          }
          actions.queueTask.mutate(taskType);
        }}
        onStateChange={(field, value) => {
          if (!entryId) {
            return;
          }
          actions.updateState.mutate({ [field]: value });
        }}
      />
    </div>
  );
}
