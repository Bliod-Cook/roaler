import { useCollections, useCreateCollection, useSession } from "@/api/hooks";
import { CollectionGrid } from "@/features/collections/collection-grid";
import { SubviewShell } from "@/features/layout/subview-shell";

export function CollectionsPage() {
  const session = useSession(false);
  const isAuthenticated = Boolean(session.data?.user);
  const collections = useCollections(isAuthenticated);
  const createCollection = useCreateCollection();
  return (
    <SubviewShell
      description="用合集把多个来源整理成稳定阅读视角，后续也能承接 digest 和批量处理。"
      eyebrow="Collections"
      title="Group subscriptions into reusable views"
    >
      <CollectionGrid
        busy={createCollection.isPending}
        items={collections.data}
        onCreate={async (payload) => {
          await createCollection.mutateAsync(payload);
        }}
      />
    </SubviewShell>
  );
}
