import type { IconName } from "@/components/ui/icons";

export const NAV_ITEMS: ReadonlyArray<{
  to: "/" | "/collections" | "/sources" | "/search" | "/settings";
  label: string;
  description: string;
  icon: IconName;
}> = [
  { to: "/", label: "Inbox", description: "Unified timeline", icon: "inbox" },
  { to: "/collections", label: "Collections", description: "Group subscriptions", icon: "collections" },
  { to: "/sources", label: "Sources", description: "RSS and RSSHub", icon: "sources" },
  { to: "/search", label: "Search", description: "Query the archive", icon: "search" },
  { to: "/settings", label: "Settings", description: "Runtime control", icon: "settings" },
] as const;
