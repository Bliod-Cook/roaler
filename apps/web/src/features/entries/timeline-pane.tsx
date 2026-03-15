import { Link } from "@tanstack/react-router";

import type { EntrySummary } from "@/api/types";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";
import { Icon } from "@/components/ui/icons";
import { cn } from "@/utils/cn";

type TimelinePaneProps = {
  activeEntryId?: string;
  entries: EntrySummary[] | undefined;
};

export function TimelinePane({ activeEntryId, entries }: TimelinePaneProps) {
  return (
    <Card className="flex min-h-[520px] flex-col rounded-panel p-0 lg:h-[calc(100vh-2rem)]">
      <div className="border-b panel-divider px-5 py-5">
        <div className="flex items-center justify-between gap-3">
          <div>
            <p className="eyebrow">Timeline</p>
            <h2 className="mt-3 font-display text-2xl text-text">Latest feed flow</h2>
          </div>
          <Badge>{entries?.length ?? 0} items</Badge>
        </div>
        <p className="mt-3 text-sm leading-7 text-text-secondary">
          优先展示最新条目，保持来源、摘要和阅读状态在一列内完成决策。
        </p>
      </div>
      <div className="thin-scrollbar flex-1 space-y-2 overflow-y-auto p-3">
        {(entries ?? []).map((entry) => {
          const active = entry.id === activeEntryId;
          return (
            <Link
              key={entry.id}
              className={cn(
                "block rounded-card border p-4 transition-colors",
                active
                  ? "border-accent/20 bg-accent-soft"
                  : "border-line/60 bg-panel-soft/50 hover:bg-panel-soft",
              )}
              params={{ entryId: entry.id }}
              to="/entries/$entryId"
            >
              <div className="flex items-start justify-between gap-3">
                <div className="min-w-0 space-y-2">
                  <div className="flex items-center gap-2">
                    {!entry.is_read ? (
                      <span className="size-2 rounded-full bg-accent" />
                    ) : (
                      <span className="size-2 rounded-full bg-text-tertiary/30" />
                    )}
                    <p className="truncate text-xs font-semibold uppercase tracking-[0.14em] text-text-tertiary">
                      {entry.source_title}
                    </p>
                  </div>
                  <h3 className="line-clamp-2 text-base font-semibold leading-6 text-text">
                    {entry.title}
                  </h3>
                  <p className="line-clamp-3 text-sm leading-6 text-text-secondary">
                    {entry.ai_summary ?? entry.summary ?? "No summary yet."}
                  </p>
                </div>
                <Icon
                  className={cn("mt-1 size-4 shrink-0", active ? "text-accent" : "text-text-tertiary")}
                  name="chevron-right"
                />
              </div>
              <div className="mt-4 flex flex-wrap items-center gap-2">
                <Badge tone={entry.is_read ? "neutral" : "success"}>
                  {entry.is_read ? "Read" : "Unread"}
                </Badge>
                {entry.is_starred ? <Badge tone="accent">Starred</Badge> : null}
                {entry.is_saved ? <Badge>Saved</Badge> : null}
              </div>
            </Link>
          );
        })}
        {!entries?.length ? (
          <div className="flex h-full min-h-[280px] flex-col items-center justify-center gap-4 rounded-card border border-dashed border-line px-6 text-center">
            <div className="flex size-12 items-center justify-center rounded-2xl bg-accent-soft text-accent">
              <Icon name="inbox" />
            </div>
            <div className="space-y-2">
              <p className="text-base font-semibold text-text">Timeline is empty</p>
              <p className="max-w-xs text-sm leading-7 text-text-secondary">
                先去 Sources 页面添加 RSS 或 RSSHub 源，时间线和详情栏就会自动填充。
              </p>
            </div>
          </div>
        ) : null}
      </div>
    </Card>
  );
}
