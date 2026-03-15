import { useState } from "react";

import type { CollectionWithCount } from "@/api/types";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import { Icon } from "@/components/ui/icons";
import { Input } from "@/components/ui/input";

type CollectionGridProps = {
  items: CollectionWithCount[] | undefined;
  busy: boolean;
  onCreate: (payload: { name: string; accent_color: string }) => Promise<void>;
};

export function CollectionGrid({ items, busy, onCreate }: CollectionGridProps) {
  const [name, setName] = useState("");

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    await onCreate({ name, accent_color: "#c97741" });
    setName("");
  }

  return (
    <div className="grid gap-6 xl:grid-cols-[1.25fr_0.75fr]">
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
                    <Icon name="collections" />
                  </div>
                  <div>
                    <h2 className="text-base font-semibold text-text">{item.name}</h2>
                    <p className="text-sm text-text-tertiary">{item.slug}</p>
                  </div>
                </div>
                <p className="text-sm leading-7 text-text-secondary">
                  用同一合集整理多个来源，后续可用于摘要批处理和阅读视角切换。
                </p>
              </div>
              <Badge>{item.source_count} sources</Badge>
            </div>
            <div className="mt-4 flex flex-wrap gap-2">
              <Badge tone="accent">{item.unread_count} unread</Badge>
              {item.last_digest_at ? <Badge>Digest ready</Badge> : <Badge>No digest yet</Badge>}
            </div>
          </article>
        ))}
        {!items?.length ? (
          <div className="flex min-h-[260px] flex-col items-center justify-center gap-4 rounded-card border border-dashed border-line px-6 text-center">
            <div className="flex size-12 items-center justify-center rounded-2xl bg-accent-soft text-accent">
              <Icon name="collections" />
            </div>
            <div className="space-y-2">
              <p className="text-base font-semibold text-text">No collections yet</p>
              <p className="max-w-sm text-sm leading-7 text-text-secondary">
                先创建一个阅读分组，把同主题或同来源策略的订阅放到一起。
              </p>
            </div>
          </div>
        ) : null}
      </section>
      <aside className="rounded-card border border-line/70 bg-panel-soft/65 p-5">
        <p className="eyebrow">Create</p>
        <h3 className="mt-3 text-xl font-semibold text-text">Add a collection</h3>
        <p className="mt-2 text-sm leading-7 text-text-secondary">
          新建一个阅读分组，为来源管理和后续 AI digest 做准备。
        </p>
        <form className="mt-6 space-y-5" onSubmit={handleSubmit}>
          <Field hint="展示名称会同步到导航和来源管理。" htmlFor="collection-name" label="Collection name">
            <Input
              id="collection-name"
              placeholder="Design Watchlist"
              value={name}
              onChange={(event) => setName(event.target.value)}
            />
          </Field>
          <Button className="w-full" disabled={busy} type="submit">
            <Icon name="plus" />
            {busy ? "Creating…" : "Create collection"}
          </Button>
        </form>
      </aside>
    </div>
  );
}
