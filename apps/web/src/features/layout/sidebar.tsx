import { Link, useNavigate, useRouterState } from "@tanstack/react-router";

import type { SessionUser } from "@/api/types";
import { Button } from "@/components/ui/button";
import { Icon } from "@/components/ui/icons";
import { NAV_ITEMS } from "@/features/layout/nav-items";
import { useAuthActions } from "@/api/hooks";
import { cn } from "@/utils/cn";

export function Sidebar({ user }: { user: SessionUser | null }) {
  const pathname = useRouterState({ select: (state) => state.location.pathname });
  const navigate = useNavigate();
  const actions = useAuthActions();

  async function handleLogout() {
    await actions.logout.mutateAsync();
    await navigate({ to: "/login" });
  }

  return (
    <aside className="app-panel sticky top-4 hidden h-[calc(100vh-2rem)] w-[288px] shrink-0 flex-col justify-between rounded-panel p-4 lg:flex">
      <div className="space-y-5">
        <div className="space-y-4 rounded-card border border-line/70 bg-panel-soft/70 p-4">
          <div className="flex items-center gap-3">
            <div className="flex size-11 items-center justify-center rounded-2xl bg-accent-soft text-accent">
              <Icon className="size-5" name="sparkles" />
            </div>
            <div>
              <p className="font-display text-xl text-text">Roaler</p>
              <p className="text-sm text-text-secondary">Desktop reading workspace</p>
            </div>
          </div>
          <p className="text-sm leading-7 text-text-secondary">
            单管理员自部署信息流桌面，把来源、时间线、全文阅读和 AI 辅助放在同一层。
          </p>
        </div>
        <div className="grid grid-cols-2 gap-2">
          <Link className="block" to="/sources">
            <Button className="w-full justify-start" size="sm" variant="secondary">
              <Icon name="plus" />
              Add source
            </Button>
          </Link>
          <Link className="block" to="/collections">
            <Button className="w-full justify-start" size="sm" variant="ghost">
              <Icon name="collections" />
              New group
            </Button>
          </Link>
        </div>
        <nav className="space-y-2">
          {NAV_ITEMS.map((item) => (
            <Link
              key={item.to}
              to={item.to}
              className={cn(
                "flex items-center justify-between rounded-card border px-4 py-3 transition-colors",
                pathname === item.to
                  ? "border-accent/25 bg-accent-soft text-accent"
                  : "border-line/60 bg-panel-soft/40 text-text-secondary hover:bg-panel-soft",
              )}
            >
              <div className="flex items-center gap-3">
                <div
                  className={cn(
                    "flex size-10 items-center justify-center rounded-2xl border",
                    pathname === item.to
                      ? "border-accent/20 bg-panel text-accent"
                      : "border-line/60 bg-panel text-text-secondary",
                  )}
                >
                  <Icon name={item.icon} />
                </div>
                <div>
                  <p className="text-sm font-semibold text-text">{item.label}</p>
                  <p className="text-xs text-text-tertiary">{item.description}</p>
                </div>
              </div>
              <Icon className="size-4" name="chevron-right" />
            </Link>
          ))}
        </nav>
      </div>
      <div className="space-y-3">
        <div className="rounded-card border border-line/70 bg-panel-soft/70 p-4">
          <div className="flex items-center gap-3">
            <div className="flex size-11 items-center justify-center rounded-2xl bg-panel text-text">
              <Icon name="user" />
            </div>
            <div className="min-w-0">
              <p className="truncate text-sm font-semibold text-text">
                {user?.display_name ?? "未登录"}
              </p>
              <p className="truncate text-sm text-text-tertiary">
                {user?.email ?? "请先初始化或登录"}
              </p>
            </div>
          </div>
        </div>
        <Button className="w-full justify-start" onClick={handleLogout} variant="ghost">
          <Icon name="logout" />
          Sign out
        </Button>
      </div>
    </aside>
  );
}
