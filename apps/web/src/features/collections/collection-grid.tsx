import { useState } from "react";

import type { CollectionWithCount } from "@/api/types";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
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
    <div className="grid gap-6 xl:grid-cols-[1.3fr_0.9fr]">
      <Card className="space-y-4">
        <p className="eyebrow">Folders</p>
        {(items ?? []).map((item) => (
          <div key={item.id} className="rounded-[1.4rem] border border-borderWarm bg-white/70 p-4">
            <div className="flex items-center justify-between gap-4">
              <div>
                <h2 className="font-display text-2xl">{item.name}</h2>
                <p className="text-sm text-ink/60">{item.slug}</p>
              </div>
              <span className="rounded-full bg-accentSoft px-3 py-1 text-xs">
                {item.source_count} sources
              </span>
            </div>
          </div>
        ))}
      </Card>
      <Card>
        <p className="eyebrow">New collection</p>
        <form className="mt-4 space-y-4" onSubmit={handleSubmit}>
          <Input placeholder="Name" value={name} onChange={(event) => setName(event.target.value)} />
          <Button className="w-full" disabled={busy} type="submit">
            {busy ? "Creating…" : "Create"}
          </Button>
        </form>
      </Card>
    </div>
  );
}

