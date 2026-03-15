import type { PropsWithChildren } from "react";

import { useSession } from "@/api/hooks";
import { MobileNav } from "@/features/layout/mobile-nav";
import { Sidebar } from "@/features/layout/sidebar";

export function AppShell({ children }: PropsWithChildren) {
  const session = useSession(false);
  const user = session.data?.user ?? null;

  return (
    <div className="app-shell-grid mx-auto max-w-[1680px] px-3 py-3 text-text sm:px-4 lg:px-5">
      <MobileNav user={user} />
      <div className="flex gap-4">
        <Sidebar user={user} />
        <main className="min-w-0 flex-1">{children}</main>
      </div>
    </div>
  );
}
