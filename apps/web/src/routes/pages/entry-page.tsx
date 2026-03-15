import { useParams } from "@tanstack/react-router";

import { ReaderWorkspace } from "@/features/entries/reader-workspace";

export function EntryPage() {
  const { entryId } = useParams({ from: "/entries/$entryId" });
  return <ReaderWorkspace entryId={entryId} />;
}
