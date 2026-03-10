import { useCollections, useCreateCollection, useSession } from "@/api/hooks";
import { SectionHeader } from "@/components/shell/section-header";
import { CollectionGrid } from "@/features/collections/collection-grid";

export function CollectionsPage() {
  const session = useSession(false);
  const isAuthenticated = Boolean(session.data?.user);
  const collections = useCollections(isAuthenticated);
  const createCollection = useCreateCollection();
  return (
    <div className="space-y-6">
      <SectionHeader
        eyebrow="Collections"
        title="Curate feed groups"
        description="用合集把多个来源组织成稳定阅读视角。"
      />
      <CollectionGrid
        busy={createCollection.isPending}
        items={collections.data}
        onCreate={async (payload) => {
          await createCollection.mutateAsync(payload);
        }}
      />
    </div>
  );
}
