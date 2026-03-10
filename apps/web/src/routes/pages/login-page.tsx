import { useNavigate } from "@tanstack/react-router";

import { useAuthActions } from "@/api/hooks";
import { LoginForm } from "@/features/auth/login-form";

export function LoginPage() {
  const navigate = useNavigate();
  const actions = useAuthActions();
  return (
    <div className="flex min-h-screen items-center px-4 py-12">
      <LoginForm
        busy={actions.login.isPending}
        error={actions.login.error?.message ?? null}
        onSubmit={async (payload) => {
          await actions.login.mutateAsync(payload);
          await navigate({ to: "/" });
        }}
      />
    </div>
  );
}

