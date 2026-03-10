import {
  Link,
  Navigate,
  Outlet,
  createRootRoute,
  createRoute,
  createRouter,
  useRouterState,
} from "@tanstack/react-router";

import { useBootstrapStatus, useSession } from "@/api/hooks";
import { AppShell } from "@/components/shell/app-shell";
import { Button } from "@/components/ui/button";
import { CollectionsPage } from "@/routes/pages/collections-page";
import { EntryPage } from "@/routes/pages/entry-page";
import { InboxPage } from "@/routes/pages/inbox-page";
import { LoginPage } from "@/routes/pages/login-page";
import { SearchPage } from "@/routes/pages/search-page";
import { SettingsPage } from "@/routes/pages/settings-page";
import { SetupPage } from "@/routes/pages/setup-page";
import { SourcesPage } from "@/routes/pages/sources-page";

function RootLayout() {
  const pathname = useRouterState({ select: (state) => state.location.pathname });
  const bootstrap = useBootstrapStatus();
  const shouldCheckSession = bootstrap.data?.bootstrapped ?? false;
  const session = useSession(shouldCheckSession);
  const isAuthRoute = pathname === "/setup" || pathname === "/login";

  if (bootstrap.isPending || (shouldCheckSession && session.isPending)) {
    return <div className="flex min-h-screen items-center justify-center">Loading…</div>;
  }

  const initialized = bootstrap.data?.bootstrapped ?? false;
  const isAuthenticated = Boolean(session.data?.user);

  if (!initialized && pathname !== "/setup") {
    return <Navigate to="/setup" />;
  }
  if (initialized && isAuthenticated && isAuthRoute) {
    return <Navigate to="/" />;
  }
  if (initialized && !isAuthenticated && pathname !== "/login") {
    return <Navigate to="/login" />;
  }

  if (isAuthRoute) {
    return <Outlet />;
  }
  return (
    <AppShell>
      <Outlet />
    </AppShell>
  );
}

function NotFoundPage() {
  return (
    <div className="flex min-h-screen items-center justify-center px-4">
      <div className="editorial-surface max-w-lg rounded-[2rem] p-8 text-center">
        <p className="eyebrow">404</p>
        <h1 className="mt-3 font-display text-4xl">Page not found</h1>
        <p className="mt-4 text-sm leading-7 text-ink/70">这个前端骨架只铺了首版核心视图。</p>
        <Link to="/">
          <Button className="mt-6">Back to inbox</Button>
        </Link>
      </div>
    </div>
  );
}

const rootRoute = createRootRoute({
  component: RootLayout,
  notFoundComponent: NotFoundPage,
});

const setupRoute = createRoute({ getParentRoute: () => rootRoute, path: "setup", component: SetupPage });
const loginRoute = createRoute({ getParentRoute: () => rootRoute, path: "login", component: LoginPage });
const inboxRoute = createRoute({ getParentRoute: () => rootRoute, path: "/", component: InboxPage });
const collectionsRoute = createRoute({ getParentRoute: () => rootRoute, path: "collections", component: CollectionsPage });
const sourcesRoute = createRoute({ getParentRoute: () => rootRoute, path: "sources", component: SourcesPage });
const searchRoute = createRoute({ getParentRoute: () => rootRoute, path: "search", component: SearchPage });
const settingsRoute = createRoute({ getParentRoute: () => rootRoute, path: "settings", component: SettingsPage });
const entryRoute = createRoute({ getParentRoute: () => rootRoute, path: "entries/$entryId", component: EntryPage });

const routeTree = rootRoute.addChildren([
  setupRoute,
  loginRoute,
  inboxRoute,
  collectionsRoute,
  sourcesRoute,
  searchRoute,
  settingsRoute,
  entryRoute,
]);

export const router = createRouter({
  routeTree,
  defaultPreload: "intent",
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
