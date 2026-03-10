import { useMemo } from "react";

import { useEntries, useSession } from "@/api/hooks";
import { SectionHeader } from "@/components/shell/section-header";
import { EntryList } from "@/features/entries/entry-list";

export function InboxPage() {
  const params = useMemo(() => new URLSearchParams(), []);
  const session = useSession(false);
  const entries = useEntries(params, Boolean(session.data?.user));
  return (
    <div className="space-y-6">
      <SectionHeader
        eyebrow="Inbox"
        title="Your reading timeline"
        description="聚合所有订阅源的最新条目，先保证可读、可筛、可跳转到正文详情。"
      />
      <EntryList
        entries={entries.data}
        title="Timeline"
        description="默认按发布时间倒序展示。"
      />
    </div>
  );
}
