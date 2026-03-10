import type { InputHTMLAttributes } from "react";

import { cn } from "@/utils/cn";

export function Input(props: InputHTMLAttributes<HTMLInputElement>) {
  return (
    <input
      {...props}
      className={cn(
        "w-full rounded-2xl border border-borderWarm bg-white/80 px-4 py-3 text-sm text-ink outline-none placeholder:text-ink/50 focus:border-accent",
        props.className,
      )}
    />
  );
}

