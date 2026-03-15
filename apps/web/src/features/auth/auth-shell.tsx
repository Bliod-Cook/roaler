import type { PropsWithChildren } from "react";

import { Badge } from "@/components/ui/badge";
import { Icon } from "@/components/ui/icons";

type AuthShellProps = PropsWithChildren<{
  eyebrow: string;
  title: string;
  description: string;
  accentLabel: string;
}>;

export function AuthShell({
  accentLabel,
  children,
  description,
  eyebrow,
  title,
}: AuthShellProps) {
  return (
    <div className="mx-auto grid min-h-screen max-w-[1360px] gap-6 px-4 py-6 lg:px-6 xl:grid-cols-[1.05fr_0.95fr] xl:px-8">
      <section className="app-panel hidden rounded-panel p-10 xl:flex xl:flex-col xl:justify-between">
        <div className="space-y-6">
          <Badge className="w-fit" tone="accent">
            <Icon className="size-3.5" name="sparkles" />
            {accentLabel}
          </Badge>
          <div className="space-y-4">
            <p className="eyebrow">Roaler</p>
            <h1 className="max-w-xl font-display text-5xl leading-[1.05] text-text">
              Open feeds,
              <br />
              one focused desk.
            </h1>
            <p className="max-w-xl text-base leading-8 text-text-secondary">
              自部署阅读工作台，保持来源、时间线、全文阅读和 AI 辅助在同一张桌面上。
            </p>
          </div>
        </div>
        <div className="grid gap-4 lg:grid-cols-3">
          <AuthMetric label="Timeline" value="Unified feed flow" />
          <AuthMetric label="Ownership" value="Single-admin control" />
          <AuthMetric label="Stack" value="RSS, search, AI" />
        </div>
      </section>
      <section className="app-panel flex min-h-[calc(100vh-3rem)] flex-col justify-between rounded-panel p-6 sm:p-8 xl:min-h-0 xl:p-10">
        <div className="space-y-4">
          <Badge className="w-fit xl:hidden" tone="accent">
            <Icon className="size-3.5" name="sparkles" />
            {accentLabel}
          </Badge>
          <div className="space-y-3">
            <p className="eyebrow">{eyebrow}</p>
            <h2 className="max-w-lg font-display text-4xl leading-tight text-text">{title}</h2>
            <p className="max-w-lg text-sm leading-7 text-text-secondary">{description}</p>
          </div>
        </div>
        <div className="mt-8">{children}</div>
      </section>
    </div>
  );
}

function AuthMetric({ label, value }: { label: string; value: string }) {
  return (
    <div className="surface-muted rounded-card border border-line/70 p-4">
      <p className="text-xs font-semibold uppercase tracking-[0.16em] text-text-tertiary">
        {label}
      </p>
      <p className="mt-3 text-sm font-semibold leading-6 text-text">{value}</p>
    </div>
  );
}
