import type { PropsWithChildren } from "react";

import { useSession } from "@/api/hooks";
import { MobileNav } from "@/features/layout/mobile-nav";
import { Sidebar } from "@/features/layout/sidebar";

export function AppShell({ children }: PropsWithChildren) {
  const session = useSession(false);
  const user = session.data?.user ?? null;

  return (
    <div className="mx-auto min-h-screen max-w-[1500px] px-4 py-6 text-ink lg:px-8">
      <MobileNav user={user} />
      <div className="flex gap-6">
        <Sidebar user={user} />
        <main className="min-w-0 flex-1">{children}</main>
      </div>
    </div>
  );
}
