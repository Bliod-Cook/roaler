import type { ReactElement, SVGProps } from "react";

import { cn } from "@/utils/cn";

export type IconName =
  | "arrow-left"
  | "bookmark"
  | "check"
  | "chevron-right"
  | "collections"
  | "entry"
  | "inbox"
  | "logout"
  | "plus"
  | "search"
  | "settings"
  | "sparkles"
  | "sources"
  | "star"
  | "translate"
  | "user";

const iconPaths: Record<IconName, ReactElement> = {
  "arrow-left": <path d="m10 6-6 6 6 6m-6-6h16" />,
  bookmark: <path d="M7 5.5h10a1 1 0 0 1 1 1V20l-6-3.6L6 20V6.5a1 1 0 0 1 1-1Z" />,
  check: <path d="m5.5 12.5 4 4 9-9" />,
  "chevron-right": <path d="m10 6 6 6-6 6" />,
  collections: (
    <>
      <path d="M6 7.5A2.5 2.5 0 0 1 8.5 5H18a1 1 0 0 1 1 1v9.5A2.5 2.5 0 0 1 16.5 18H7a1 1 0 0 1-1-1V7.5Z" />
      <path d="M5 8H4a1 1 0 0 0-1 1v8.5A2.5 2.5 0 0 0 5.5 20H14" />
    </>
  ),
  entry: (
    <>
      <path d="M7 5.5h10a1 1 0 0 1 1 1v11a1 1 0 0 1-1 1H7a1 1 0 0 1-1-1v-11a1 1 0 0 1 1-1Z" />
      <path d="M9 9.5h6M9 13h6M9 16.5h4" />
    </>
  ),
  inbox: (
    <>
      <path d="M5 8.5A2.5 2.5 0 0 1 7.5 6h9A2.5 2.5 0 0 1 19 8.5v7A2.5 2.5 0 0 1 16.5 18h-9A2.5 2.5 0 0 1 5 15.5v-7Z" />
      <path d="M6 13h3l1.2 2h3.6l1.2-2H18" />
    </>
  ),
  logout: (
    <>
      <path d="M10 6H7.5A2.5 2.5 0 0 0 5 8.5v7A2.5 2.5 0 0 0 7.5 18H10" />
      <path d="m13 8 4 4-4 4M17 12H9" />
    </>
  ),
  plus: (
    <>
      <path d="M12 5v14" />
      <path d="M5 12h14" />
    </>
  ),
  search: <path d="m17 17-3.8-3.8M15 10.5a4.5 4.5 0 1 1-9 0 4.5 4.5 0 0 1 9 0Z" />,
  settings: (
    <>
      <path d="m12 3 1.7 2.7 3.1.7-.7 3.1L18 12l-1.9 2.5.7 3.1-3.1.7L12 21l-1.7-2.7-3.1-.7.7-3.1L6 12l1.9-2.5-.7-3.1 3.1-.7L12 3Z" />
      <path d="M14.8 12a2.8 2.8 0 1 1-5.6 0 2.8 2.8 0 0 1 5.6 0Z" />
    </>
  ),
  sparkles: (
    <>
      <path d="m12 4 1.2 3.3L16.5 8.5l-3.3 1.2L12 13l-1.2-3.3L7.5 8.5l3.3-1.2L12 4Z" />
      <path d="m18.5 14 0.7 1.8 1.8 0.7-1.8 0.7-0.7 1.8-.7-1.8-1.8-.7 1.8-.7.7-1.8Z" />
      <path d="m6 15 0.8 2 2 0.8-2 0.8-0.8 2-.8-2-2-.8 2-.8.8-2Z" />
    </>
  ),
  sources: (
    <>
      <path d="M6.5 16.5a7 7 0 0 1 0-9.9" />
      <path d="M10 13a2.5 2.5 0 1 1 0-3.5" />
      <path d="M17.5 7.5a7 7 0 0 1 0 9.9" />
      <path d="M14 11a2.5 2.5 0 1 1 0 3.5" />
    </>
  ),
  star: <path d="m12 4 2.3 4.7 5.2.8-3.8 3.7.9 5.3L12 16l-4.6 2.5.9-5.3L4.5 9.5l5.2-.8L12 4Z" />,
  translate: (
    <>
      <path d="M4 7.5h9" />
      <path d="M8.5 5v2.5A10 10 0 0 1 5 15" />
      <path d="M6.5 11a8.5 8.5 0 0 0 3 2.2" />
      <path d="m15 8 4 11" />
      <path d="m13.5 15 6-1" />
    </>
  ),
  user: (
    <>
      <path d="M12 12a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7Z" />
      <path d="M5.5 19.5a6.5 6.5 0 0 1 13 0" />
    </>
  )
};

type IconProps = SVGProps<SVGSVGElement> & {
  name: IconName;
};

export function Icon({ className, name, ...props }: IconProps) {
  return (
    <svg
      aria-hidden="true"
      className={cn("size-4 shrink-0", className)}
      fill="none"
      stroke="currentColor"
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth={1.8}
      viewBox="0 0 24 24"
      {...props}
    >
      {iconPaths[name]}
    </svg>
  );
}
