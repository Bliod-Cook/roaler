import { Link } from "@tanstack/react-router";

import type { EntrySummary } from "@/api/types";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";
import { Icon } from "@/components/ui/icons";

type EntryListProps = {
  entries: EntrySummary[] | undefined;
  title: string;
  description: string;
};

export function EntryList({ entries, title, description }: EntryListProps) {
  return (
    <Card className="space-y-4 rounded-panel p-5">
      <div className="space-y-2">
        <p className="eyebrow">{title}</p>
        <p className="text-sm leading-7 text-text-secondary">{description}</p>
      </div>
      <div className="space-y-3">
        {(entries ?? []).map((entry) => (
          <Link
            key={entry.id}
            className="block rounded-card border border-line/70 bg-panel-soft/65 p-4 transition-colors hover:bg-panel-soft"
            to="/entries/$entryId"
            params={{ entryId: entry.id }}
          >
            <div className="flex items-start justify-between gap-4">
              <div className="space-y-2">
                <p className="text-xs font-semibold uppercase tracking-[0.16em] text-text-tertiary">
                  {entry.source_title}
                </p>
                <h2 className="text-base font-semibold leading-6 text-text">{entry.title}</h2>
                <p className="line-clamp-3 text-sm leading-6 text-text-secondary">
                  {entry.ai_summary ?? entry.summary ?? "No summary yet."}
                </p>
              </div>
              <Icon className="mt-1 size-4 shrink-0 text-text-tertiary" name="chevron-right" />
            </div>
            <div className="mt-4 flex flex-wrap gap-2">
              <Badge tone={entry.is_read ? "neutral" : "success"}>
                {entry.is_read ? "Read" : "Unread"}
              </Badge>
              {entry.is_starred ? <Badge tone="accent">Starred</Badge> : null}
            </div>
          </Link>
        ))}
        {!entries?.length ? (
          <p className="rounded-card border border-dashed border-line p-6 text-sm leading-7 text-text-secondary">
            暂无内容。先去 Sources 页面添加 RSS 或 RSSHub 源。
          </p>
        ) : null}
      </div>
    </Card>
  );
}
