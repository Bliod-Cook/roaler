import type { PropsWithChildren, ReactNode } from "react";

import { cn } from "@/utils/cn";

type FieldProps = PropsWithChildren<{
  label: string;
  hint?: string;
  error?: string | null;
  action?: ReactNode;
  htmlFor?: string;
  className?: string;
}>;

export function Field({ action, children, className, error, hint, htmlFor, label }: FieldProps) {
  return (
    <div className={cn("space-y-2", className)}>
      <div className="flex items-center justify-between gap-3">
        {htmlFor ? (
          <label className="text-sm font-semibold text-text" htmlFor={htmlFor}>
            {label}
          </label>
        ) : (
          <span className="text-sm font-semibold text-text">{label}</span>
        )}
        {action}
      </div>
      {children}
      {error ? <span className="block text-sm text-danger">{error}</span> : null}
      {!error && hint ? <span className="block text-sm text-text-tertiary">{hint}</span> : null}
    </div>
  );
}
