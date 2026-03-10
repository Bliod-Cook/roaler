import type { PropsWithChildren } from "react";

import { cn } from "@/utils/cn";

export function Card({
  children,
  className,
}: PropsWithChildren<{ className?: string }>) {
  return (
    <section className={cn("editorial-surface rounded-[2rem] p-6", className)}>
      {children}
    </section>
  );
}

