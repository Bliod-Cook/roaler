import type { SelectHTMLAttributes } from "react";

import { cn } from "@/utils/cn";

export function Select(props: SelectHTMLAttributes<HTMLSelectElement>) {
  return (
    <select
      {...props}
      className={cn(
        "h-11 w-full rounded-2xl border border-line bg-panel-soft px-4 text-sm text-text outline-none transition-colors focus:border-accent focus:bg-panel disabled:cursor-not-allowed disabled:opacity-60",
        props.className,
      )}
    />
  );
}
