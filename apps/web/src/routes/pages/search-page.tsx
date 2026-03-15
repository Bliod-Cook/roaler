import { useState } from "react";

import { SubviewShell } from "@/features/layout/subview-shell";
import { SearchPanel } from "@/features/search/search-panel";
import { useSearch, useSession } from "@/api/hooks";

export function SearchPage() {
  const [query, setQuery] = useState("");
  const session = useSession(false);
  const search = useSearch(query, Boolean(session.data?.user));
  return (
    <SubviewShell
      description="按标题、摘要、抽取正文和 AI 摘要检索整个归档，结果直接回跳阅读详情。"
      eyebrow="Search"
      title="Search the whole archive"
    >
      <SearchPanel items={search.data} onQueryChange={setQuery} query={query} />
    </SubviewShell>
  );
}
