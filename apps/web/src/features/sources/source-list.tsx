import { useState } from "react";

import type { CollectionWithCount, SourceView } from "@/api/types";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";

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
    <div className="grid gap-6 xl:grid-cols-[1.5fr_1fr]">
      <Card className="space-y-4">
        <p className="eyebrow">Sources</p>
        {(items ?? []).map((item) => (
          <div key={item.id} className="rounded-[1.4rem] border border-borderWarm bg-white/70 p-4">
            <div className="flex items-start justify-between gap-4">
              <div>
                <h2 className="font-display text-2xl">{item.title}</h2>
                <p className="text-sm text-ink/60">{item.feed_url}</p>
              </div>
              <span className="rounded-full bg-accentSoft px-3 py-1 text-xs">
                {item.kind}
              </span>
            </div>
            {item.last_error ? <p className="mt-3 text-sm text-red-600">{item.last_error}</p> : null}
          </div>
        ))}
      </Card>
      <Card>
        <p className="eyebrow">Add source</p>
        <form className="mt-4 space-y-4" onSubmit={handleSubmit}>
          <div className="grid grid-cols-2 gap-2 rounded-[1.4rem] bg-white/70 p-1">
            <button className={kind === "rss" ? "rounded-full bg-accent px-3 py-2 text-white" : "rounded-full px-3 py-2"} onClick={() => setKind("rss")} type="button">RSS</button>
            <button className={kind === "rsshub" ? "rounded-full bg-accent px-3 py-2 text-white" : "rounded-full px-3 py-2"} onClick={() => setKind("rsshub")} type="button">RSSHub</button>
          </div>
          {kind === "rss" ? (
            <Input placeholder="https://example.com/feed.xml" value={feedUrl} onChange={(event) => setFeedUrl(event.target.value)} />
          ) : (
            <Input placeholder="/github/issue/openai/openai" value={rsshubRoute} onChange={(event) => setRsshubRoute(event.target.value)} />
          )}
          <select
            className="w-full rounded-2xl border border-borderWarm bg-white/80 px-4 py-3 text-sm"
            onChange={(event) => setCollectionId(event.target.value)}
            value={collectionId}
          >
            <option value="">No collection</option>
            {(collections ?? []).map((collection) => (
              <option key={collection.id} value={collection.id}>
                {collection.name}
              </option>
            ))}
          </select>
          <Button className="w-full" disabled={busy} type="submit">
            {busy ? "Adding…" : "Add source"}
          </Button>
        </form>
      </Card>
    </div>
  );
}

