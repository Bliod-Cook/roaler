import { useParams } from "@tanstack/react-router";

import { useEntryActions, useEntryDetail, useSession } from "@/api/hooks";
import { SectionHeader } from "@/components/shell/section-header";
import { EntryDetailPanel } from "@/features/entries/entry-detail-panel";

export function EntryPage() {
  const { entryId } = useParams({ from: "/entries/$entryId" });
  const session = useSession(false);
  const isAuthenticated = Boolean(session.data?.user);
  const detail = useEntryDetail(entryId, isAuthenticated);
  const actions = useEntryActions(entryId);
  return (
    <div className="space-y-6">
      <SectionHeader
        eyebrow="Entry"
        title="Read the full item"
        description="正文、摘要、翻译和标签在同一个视图里集中呈现。"
      />
      <EntryDetailPanel
        busy={actions.queueTask.isPending || actions.updateState.isPending}
        detail={detail.data}
        onQueueTask={(taskType) => {
          actions.queueTask.mutate(taskType);
        }}
        onStateChange={(field, value) => {
          actions.updateState.mutate({ [field]: value });
        }}
      />
    </div>
  );
}
