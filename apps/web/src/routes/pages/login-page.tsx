import { useNavigate } from "@tanstack/react-router";

import { useAuthActions } from "@/api/hooks";
import { AuthShell } from "@/features/auth/auth-shell";
import { LoginForm } from "@/features/auth/login-form";

export function LoginPage() {
  const navigate = useNavigate();
  const actions = useAuthActions();
  return (
    <AuthShell
      accentLabel="Session Access"
      description="输入管理员邮箱和密码，回到统一时间线与阅读工作台。"
      eyebrow="Login"
      title="Return to your reading desk."
    >
      <LoginForm
        busy={actions.login.isPending}
        error={actions.login.error?.message ?? null}
        onSubmit={async (payload) => {
          await actions.login.mutateAsync(payload);
          await navigate({ to: "/" });
        }}
      />
    </AuthShell>
  );
}
