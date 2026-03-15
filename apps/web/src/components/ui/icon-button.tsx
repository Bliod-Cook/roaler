import type { ButtonHTMLAttributes } from "react";

import { cn } from "@/utils/cn";

type IconButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  active?: boolean;
};

export function IconButton({ active = false, className, ...props }: IconButtonProps) {
  return (
    <button
      {...props}
      className={cn(
        "inline-flex size-10 items-center justify-center rounded-2xl border text-text-secondary transition-colors duration-200",
        active
          ? "border-accent/30 bg-accent-soft text-accent"
          : "border-line bg-panel-soft hover:bg-panel",
        "focus-visible:outline focus-visible:outline-2 focus-visible:outline-accent focus-visible:outline-offset-2 focus-visible:outline-offset-app disabled:cursor-not-allowed disabled:opacity-60",
        className,
      )}
    />
  );
}
