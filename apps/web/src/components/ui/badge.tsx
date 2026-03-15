import type { PropsWithChildren } from "react";

import { cn } from "@/utils/cn";

type BadgeProps = PropsWithChildren<{
  tone?: "neutral" | "accent" | "success" | "danger";
  className?: string;
}>;

const toneMap: Record<NonNullable<BadgeProps["tone"]>, string> = {
  neutral: "bg-panel-soft text-text-secondary",
  accent: "bg-accent-soft text-accent",
  success: "bg-success/14 text-success",
  danger: "bg-danger/14 text-danger"
};

export function Badge({ children, className, tone = "neutral" }: BadgeProps) {
  return (
    <span
      className={cn(
        "inline-flex h-7 items-center rounded-full px-3 text-xs font-semibold",
        toneMap[tone],
        className,
      )}
    >
      {children}
    </span>
  );
}
