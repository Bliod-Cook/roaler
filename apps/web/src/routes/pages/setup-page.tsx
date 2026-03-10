import { useNavigate } from "@tanstack/react-router";

import { useAuthActions } from "@/api/hooks";
import { SetupForm } from "@/features/auth/setup-form";

export function SetupPage() {
  const navigate = useNavigate();
  const actions = useAuthActions();
  return (
    <div className="flex min-h-screen items-center px-4 py-12">
      <SetupForm
        busy={actions.bootstrap.isPending}
        error={actions.bootstrap.error?.message ?? null}
        onSubmit={async (payload) => {
          await actions.bootstrap.mutateAsync(payload);
          await navigate({ to: "/" });
        }}
      />
    </div>
  );
}

