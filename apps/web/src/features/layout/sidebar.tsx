import { Link, useRouterState } from "@tanstack/react-router";

import type { SessionUser } from "@/api/types";
import { Button } from "@/components/ui/button";
import { NAV_ITEMS } from "@/features/layout/nav-items";
import { cn } from "@/utils/cn";

export function Sidebar({ user }: { user: SessionUser | null }) {
  const pathname = useRouterState({ select: (state) => state.location.pathname });
  return (
    <aside className="editorial-surface sticky top-6 hidden h-[calc(100vh-3rem)] w-72 shrink-0 flex-col justify-between rounded-[2rem] p-6 lg:flex">
      <div className="space-y-8">
        <div className="space-y-3">
          <p className="eyebrow">Roaler</p>
          <h1 className="font-display text-3xl leading-tight">
            Open feeds,
            <br />
            one calm desk.
          </h1>
          <p className="text-sm leading-7 text-ink/70">
            单管理员自部署信息流工作台，优先把订阅、搜索和阅读状态拉齐。
          </p>
        </div>
        <nav className="space-y-2">
          {NAV_ITEMS.map((item) => (
            <Link
              key={item.to}
              to={item.to}
              className={cn(
                "flex items-center justify-between rounded-2xl px-4 py-3 text-sm",
                pathname === item.to
                  ? "bg-accent text-white"
                  : "bg-white/50 text-ink hover:bg-white/80",
              )}
            >
              <span>{item.label}</span>
              <span className="text-xs opacity-70">↗</span>
            </Link>
          ))}
        </nav>
      </div>
      <div className="space-y-3">
        <div className="rounded-2xl bg-white/70 p-4">
          <p className="text-xs uppercase tracking-[0.2em] text-ink/45">Admin</p>
          <p className="mt-2 font-medium">{user?.display_name ?? "未登录"}</p>
          <p className="text-sm text-ink/60">{user?.email ?? "请先初始化或登录"}</p>
        </div>
        <Button className="w-full" variant="secondary" disabled>
          Single-tenant mode
        </Button>
      </div>
    </aside>
  );
}
