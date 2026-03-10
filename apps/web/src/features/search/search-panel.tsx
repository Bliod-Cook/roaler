import type { EntrySummary } from "@/api/types";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { EntryList } from "@/features/entries/entry-list";

type SearchPanelProps = {
  query: string;
  items: EntrySummary[] | undefined;
  onQueryChange: (value: string) => void;
};

export function SearchPanel({ query, items, onQueryChange }: SearchPanelProps) {
  return (
    <div className="space-y-6">
      <Card>
        <p className="eyebrow">Search</p>
        <div className="mt-4">
          <Input placeholder="Search titles, summaries, extracted text…" value={query} onChange={(event) => onQueryChange(event.target.value)} />
        </div>
      </Card>
      <EntryList entries={items} title="Results" description="PostgreSQL 全文检索结果会展示在这里。" />
    </div>
  );
}

