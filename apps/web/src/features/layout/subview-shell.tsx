import type { PropsWithChildren, ReactNode } from "react";

import { Card } from "@/components/ui/card";

type SubviewShellProps = PropsWithChildren<{
  eyebrow: string;
  title: string;
  description: string;
  actions?: ReactNode;
}>;

export function SubviewShell({
  actions,
  children,
  description,
  eyebrow,
  title,
}: SubviewShellProps) {
  return (
    <Card className="overflow-hidden rounded-panel p-0 lg:h-[calc(100vh-2rem)]">
      <header className="subview-header-blur border-b panel-divider px-6 py-5">
        <div className="flex flex-wrap items-start justify-between gap-4">
          <div className="space-y-3">
            <p className="eyebrow">{eyebrow}</p>
            <h1 className="font-display text-3xl leading-tight text-text">{title}</h1>
            <p className="max-w-2xl text-sm leading-7 text-text-secondary">{description}</p>
          </div>
          {actions ? <div className="flex items-center gap-3">{actions}</div> : null}
        </div>
      </header>
      <div className="thin-scrollbar h-full overflow-y-auto px-6 py-6">{children}</div>
    </Card>
  );
}
