import { Link, useRouterState } from "@tanstack/react-router";

import type { SessionUser } from "@/api/types";
import { Icon } from "@/components/ui/icons";
import { NAV_ITEMS } from "@/features/layout/nav-items";
import { cn } from "@/utils/cn";

export function MobileNav({ user }: { user: SessionUser | null }) {
  const pathname = useRouterState({ select: (state) => state.location.pathname });

  return (
    <section className="app-panel mb-4 rounded-panel p-4 lg:hidden">
      <div className="flex items-start justify-between gap-4">
        <div className="space-y-2">
          <div className="flex items-center gap-3">
            <div className="flex size-10 items-center justify-center rounded-2xl bg-accent-soft text-accent">
              <Icon name="sparkles" />
            </div>
            <div>
              <p className="font-display text-2xl leading-tight text-text">Roaler</p>
              <p className="text-sm text-text-secondary">Reading workspace</p>
            </div>
          </div>
        </div>
        <div className="text-right text-sm text-text-secondary">
          <p className="font-medium text-text">{user?.display_name ?? "未登录"}</p>
          <p>{user?.email ?? "请先初始化或登录"}</p>
        </div>
      </div>
      <nav className="thin-scrollbar mt-4 flex gap-2 overflow-x-auto pb-1">
        {NAV_ITEMS.map((item) => (
          <Link
            key={item.to}
            to={item.to}
            className={cn(
              "inline-flex items-center gap-2 whitespace-nowrap rounded-full border px-4 py-2 text-sm",
              pathname === item.to
                ? "border-accent/20 bg-accent-soft text-accent"
                : "border-line bg-panel-soft text-text-secondary hover:bg-panel",
            )}
          >
            <Icon className="size-3.5" name={item.icon} />
            {item.label}
          </Link>
        ))}
      </nav>
    </section>
  );
}
