import { useCollections, useCreateSource, useSession, useSources } from "@/api/hooks";
import { SubviewShell } from "@/features/layout/subview-shell";
import { SourceList } from "@/features/sources/source-list";

export function SourcesPage() {
  const session = useSession(false);
  const isAuthenticated = Boolean(session.data?.user);
  const sources = useSources(isAuthenticated);
  const collections = useCollections(isAuthenticated);
  const createSource = useCreateSource();
  return (
    <SubviewShell
      description="首版保留标准 feed 与 RSSHub route 两种导入方式，并保持与时间线同步。"
      eyebrow="Sources"
      title="Bring RSS and RSSHub into the desk"
    >
      <SourceList
        busy={createSource.isPending}
        collections={collections.data}
        items={sources.data}
        onCreate={async (payload) => {
          await createSource.mutateAsync(payload);
        }}
      />
    </SubviewShell>
  );
}
