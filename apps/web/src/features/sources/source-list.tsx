import { useState } from "react";

import type { CollectionWithCount, SourceView } from "@/api/types";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import { Icon } from "@/components/ui/icons";
import { Input } from "@/components/ui/input";
import { SegmentedControl } from "@/components/ui/segmented-control";
import { Select } from "@/components/ui/select";

type SourceListProps = {
  items: SourceView[] | undefined;
  collections: CollectionWithCount[] | undefined;
  busy: boolean;
  onCreate: (payload: {
    kind: string;
    feed_url?: string;
    rsshub_route?: string;
    collection_ids: string[];
    refresh_interval_minutes: number;
  }) => Promise<void>;
};

export function SourceList({ items, collections, busy, onCreate }: SourceListProps) {
  const [kind, setKind] = useState<"rss" | "rsshub">("rss");
  const [feedUrl, setFeedUrl] = useState("");
  const [rsshubRoute, setRsshubRoute] = useState("");
  const [collectionId, setCollectionId] = useState("");

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    await onCreate({
      kind,
      feed_url: kind === "rss" ? feedUrl : undefined,
      rsshub_route: kind === "rsshub" ? rsshubRoute : undefined,
      collection_ids: collectionId ? [collectionId] : [],
      refresh_interval_minutes: 30,
    });
    setFeedUrl("");
    setRsshubRoute("");
  }

  return (
    <div className="grid gap-6 xl:grid-cols-[1.2fr_0.8fr]">
      <section className="space-y-3">
        {(items ?? []).map((item) => (
          <article
            key={item.id}
            className="rounded-card border border-line/70 bg-panel-soft/65 p-5"
          >
            <div className="flex items-start justify-between gap-4">
              <div className="space-y-2">
                <div className="flex items-center gap-2">
                  <div className="flex size-10 items-center justify-center rounded-2xl bg-accent-soft text-accent">
                    <Icon name="sources" />
                  </div>
                  <div>
                    <h2 className="text-base font-semibold text-text">{item.title}</h2>
                    <p className="text-sm text-text-tertiary">{item.feed_url}</p>
                  </div>
                </div>
                <p className="text-sm leading-7 text-text-secondary">
                  {item.site_url ?? item.hub_url ?? "No site url available"}
                </p>
              </div>
              <Badge tone="accent">{item.kind}</Badge>
            </div>
            <div className="mt-4 flex flex-wrap gap-2">
              <Badge tone={item.paused ? "danger" : "success"}>
                {item.paused ? "Paused" : "Active"}
              </Badge>
              <Badge>{item.refresh_interval_minutes} min cadence</Badge>
            </div>
            {item.last_error ? (
              <div className="mt-4 rounded-card border border-danger/25 bg-danger/10 p-3 text-sm text-danger">
                {item.last_error}
              </div>
            ) : null}
          </article>
        ))}
        {!items?.length ? (
          <div className="flex min-h-[260px] flex-col items-center justify-center gap-4 rounded-card border border-dashed border-line px-6 text-center">
            <div className="flex size-12 items-center justify-center rounded-2xl bg-accent-soft text-accent">
              <Icon name="sources" />
            </div>
            <div className="space-y-2">
              <p className="text-base font-semibold text-text">No sources connected</p>
              <p className="max-w-sm text-sm leading-7 text-text-secondary">
                添加 RSS 或 RSSHub 路由后，中间时间线会自动开始聚合内容。
              </p>
            </div>
          </div>
        ) : null}
      </section>
      <aside className="rounded-card border border-line/70 bg-panel-soft/65 p-5">
        <p className="eyebrow">Composer</p>
        <h3 className="mt-3 text-xl font-semibold text-text">Add a source</h3>
        <form className="mt-6 space-y-5" onSubmit={handleSubmit}>
          <Field hint="首版保留 RSS 与 RSSHub 两种导入入口。" label="Kind">
            <SegmentedControl
              items={[
                { label: "RSS / Atom", value: "rss" },
                { label: "RSSHub", value: "rsshub" },
              ]}
              onChange={(value) => setKind(value)}
              value={kind}
            />
          </Field>
          <Field
            hint={kind === "rss" ? "输入完整 feed URL。" : "输入 RSSHub route，不含基础域名。"}
            htmlFor={kind === "rss" ? "source-feed-url" : "source-rsshub-route"}
            label={kind === "rss" ? "Feed URL" : "RSSHub route"}
          >
            {kind === "rss" ? (
              <Input
                id="source-feed-url"
                placeholder="https://example.com/feed.xml"
                value={feedUrl}
                onChange={(event) => setFeedUrl(event.target.value)}
              />
            ) : (
              <Input
                id="source-rsshub-route"
                placeholder="/github/issue/openai/openai"
                value={rsshubRoute}
                onChange={(event) => setRsshubRoute(event.target.value)}
              />
            )}
          </Field>
          <Field hint="可选，把来源归到某个阅读分组。" htmlFor="source-collection" label="Collection">
            <Select
              id="source-collection"
              onChange={(event) => setCollectionId(event.target.value)}
              value={collectionId}
            >
              <option value="">No collection</option>
              {(collections ?? []).map((collection) => (
                <option key={collection.id} value={collection.id}>
                  {collection.name}
                </option>
              ))}
            </Select>
          </Field>
          <Button className="w-full" disabled={busy} type="submit">
            <Icon name="plus" />
            {busy ? "Adding…" : "Add source"}
          </Button>
        </form>
      </aside>
    </div>
  );
}
