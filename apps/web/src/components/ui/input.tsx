import type { InputHTMLAttributes } from "react";

import { cn } from "@/utils/cn";

export function Input(props: InputHTMLAttributes<HTMLInputElement>) {
  return (
    <input
      {...props}
      className={cn(
        "h-11 w-full rounded-2xl border border-line bg-panel-soft px-4 text-sm text-text outline-none transition-colors placeholder:text-text-tertiary focus:border-accent focus:bg-panel disabled:cursor-not-allowed disabled:opacity-60",
        props.className,
      )}
    />
  );
}
