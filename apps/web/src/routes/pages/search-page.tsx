import { useState } from "react";

import { SectionHeader } from "@/components/shell/section-header";
import { SearchPanel } from "@/features/search/search-panel";
import { useSearch, useSession } from "@/api/hooks";

export function SearchPage() {
  const [query, setQuery] = useState("");
  const session = useSession(false);
  const search = useSearch(query, Boolean(session.data?.user));
  return (
    <div className="space-y-6">
      <SectionHeader
        eyebrow="Search"
        title="Search the whole archive"
        description="按标题、摘要、抽取正文和 AI 摘要检索。"
      />
      <SearchPanel items={search.data} onQueryChange={setQuery} query={query} />
    </div>
  );
}
