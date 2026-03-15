import type { EntrySummary } from "@/api/types";
import { Icon } from "@/components/ui/icons";
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
      <div className="rounded-card border border-line/70 bg-panel-soft/65 p-4">
        <div className="flex items-center gap-3 rounded-2xl border border-line bg-panel px-4">
          <Icon className="size-4 text-text-tertiary" name="search" />
          <Input
            className="border-0 bg-transparent px-0 focus:bg-transparent"
            placeholder="Search titles, summaries, extracted text…"
            value={query}
            onChange={(event) => onQueryChange(event.target.value)}
          />
        </div>
      </div>
      <EntryList entries={items} title="Results" description="PostgreSQL 全文检索结果会展示在这里。" />
    </div>
  );
}
