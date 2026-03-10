import { Link, useRouterState } from "@tanstack/react-router";

import type { SessionUser } from "@/api/types";
import { NAV_ITEMS } from "@/features/layout/nav-items";
import { cn } from "@/utils/cn";

export function MobileNav({ user }: { user: SessionUser | null }) {
  const pathname = useRouterState({ select: (state) => state.location.pathname });

  return (
    <section className="editorial-surface mb-6 rounded-[1.6rem] p-4 lg:hidden">
      <div className="flex items-start justify-between gap-4">
        <div className="space-y-2">
          <p className="eyebrow">Roaler</p>
          <p className="font-display text-2xl leading-tight">Open feeds, one calm desk.</p>
        </div>
        <div className="text-right text-sm text-ink/70">
          <p className="font-medium text-ink">{user?.display_name ?? "未登录"}</p>
          <p>{user?.email ?? "请先初始化或登录"}</p>
        </div>
      </div>
      <nav className="mt-4 flex gap-2 overflow-x-auto pb-1">
        {NAV_ITEMS.map((item) => (
          <Link
            key={item.to}
            to={item.to}
            className={cn(
              "whitespace-nowrap rounded-full px-4 py-2 text-sm",
              pathname === item.to
                ? "bg-accent text-white"
                : "bg-white/70 text-ink hover:bg-white",
            )}
          >
            {item.label}
          </Link>
        ))}
      </nav>
    </section>
  );
}
