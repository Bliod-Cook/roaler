import type { ButtonHTMLAttributes, PropsWithChildren } from "react";

import { cn } from "@/utils/cn";

type ButtonProps = PropsWithChildren<ButtonHTMLAttributes<HTMLButtonElement>> & {
  variant?: "primary" | "secondary" | "ghost";
};

const variantMap: Record<NonNullable<ButtonProps["variant"]>, string> = {
  primary:
    "bg-accent text-white hover:opacity-90 focus-visible:outline-accent",
  secondary:
    "bg-accentSoft text-ink border border-borderWarm hover:bg-[#f4ddc7]",
  ghost: "bg-transparent text-ink hover:bg-white/60 border border-transparent",
};

export function Button({
  children,
  className,
  variant = "primary",
  ...props
}: ButtonProps) {
  return (
    <button
      className={cn(
        "inline-flex items-center justify-center rounded-full px-4 py-2 text-sm font-medium transition focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2",
        variantMap[variant],
        className,
      )}
      {...props}
    >
      {children}
    </button>
  );
}

