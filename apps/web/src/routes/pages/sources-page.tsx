import { useCollections, useCreateSource, useSession, useSources } from "@/api/hooks";
import { SectionHeader } from "@/components/shell/section-header";
import { SourceList } from "@/features/sources/source-list";

export function SourcesPage() {
  const session = useSession(false);
  const isAuthenticated = Boolean(session.data?.user);
  const sources = useSources(isAuthenticated);
  const collections = useCollections(isAuthenticated);
  const createSource = useCreateSource();
  return (
    <div className="space-y-6">
      <SectionHeader
        eyebrow="Sources"
        title="Bring RSS and RSSHub in"
        description="首版优先支持标准 feed 与 RSSHub route 两种输入方式。"
      />
      <SourceList
        busy={createSource.isPending}
        collections={collections.data}
        items={sources.data}
        onCreate={async (payload) => {
          await createSource.mutateAsync(payload);
        }}
      />
    </div>
  );
}
