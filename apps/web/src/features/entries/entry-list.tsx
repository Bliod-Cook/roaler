import { Link } from "@tanstack/react-router";

import type { EntrySummary } from "@/api/types";
import { Card } from "@/components/ui/card";

type EntryListProps = {
  entries: EntrySummary[] | undefined;
  title: string;
  description: string;
};

export function EntryList({ entries, title, description }: EntryListProps) {
  return (
    <Card className="space-y-4">
      <div>
        <p className="eyebrow">{title}</p>
        <p className="mt-2 text-sm text-ink/70">{description}</p>
      </div>
      <div className="space-y-3">
        {(entries ?? []).map((entry) => (
          <Link
            key={entry.id}
            className="block rounded-[1.4rem] border border-borderWarm bg-white/70 p-4 transition hover:bg-white"
            to="/entries/$entryId"
            params={{ entryId: entry.id }}
          >
            <div className="flex items-start justify-between gap-4">
              <div className="space-y-2">
                <p className="text-xs uppercase tracking-[0.16em] text-ink/45">{entry.source_title}</p>
                <h2 className="font-display text-2xl">{entry.title}</h2>
                <p className="line-clamp-3 text-sm leading-7 text-ink/70">
                  {entry.ai_summary ?? entry.summary ?? "No summary yet."}
                </p>
              </div>
              <div className="rounded-full bg-accentSoft px-3 py-1 text-xs">
                {entry.is_read ? "Read" : "Unread"}
              </div>
            </div>
          </Link>
        ))}
        {!entries?.length ? (
          <p className="rounded-[1.4rem] border border-dashed border-borderWarm p-6 text-sm text-ink/55">
            暂无内容。先去 Sources 页面添加 RSS 或 RSSHub 源。
          </p>
        ) : null}
      </div>
    </Card>
  );
}

