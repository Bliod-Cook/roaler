import type { ButtonHTMLAttributes, PropsWithChildren } from "react";

import { cn } from "@/utils/cn";

type ButtonProps = PropsWithChildren<ButtonHTMLAttributes<HTMLButtonElement>> & {
  variant?: "primary" | "secondary" | "ghost" | "danger";
  size?: "sm" | "md" | "lg";
};

const variantMap: Record<NonNullable<ButtonProps["variant"]>, string> = {
  primary:
    "border border-accent/80 bg-accent text-white shadow-[0_14px_32px_rgba(50,97,216,0.28)] hover:bg-accent/92 focus-visible:outline-accent",
  secondary:
    "border border-line bg-panel-soft text-text hover:bg-panel-strong focus-visible:outline-accent",
  ghost: "border border-transparent bg-transparent text-text-secondary hover:bg-panel-soft focus-visible:outline-accent",
  danger:
    "border border-danger/35 bg-danger/12 text-danger hover:bg-danger/18 focus-visible:outline-danger",
};

const sizeMap: Record<NonNullable<ButtonProps["size"]>, string> = {
  sm: "h-9 rounded-xl px-3.5 text-sm",
  md: "h-11 rounded-2xl px-4 text-sm",
  lg: "h-12 rounded-2xl px-5 text-sm font-semibold",
};

export function Button({
  children,
  className,
  variant = "primary",
  size = "md",
  ...props
}: ButtonProps) {
  return (
    <button
      className={cn(
        "inline-flex items-center justify-center gap-2 font-medium transition-colors duration-200 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-offset-app disabled:cursor-not-allowed disabled:opacity-60",
        variantMap[variant],
        sizeMap[size],
        className,
      )}
      {...props}
    >
      {children}
    </button>
  );
}
