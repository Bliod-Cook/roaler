import { useNavigate } from "@tanstack/react-router";

import { useAuthActions } from "@/api/hooks";
import { AuthShell } from "@/features/auth/auth-shell";
import { SetupForm } from "@/features/auth/setup-form";

export function SetupPage() {
  const navigate = useNavigate();
  const actions = useAuthActions();
  return (
    <AuthShell
      accentLabel="Bootstrap"
      description="首次部署时只创建一个管理员账号，后续所有操作都通过这个会话进入。"
      eyebrow="Setup"
      title="Create the first and only admin."
    >
      <SetupForm
        busy={actions.bootstrap.isPending}
        error={actions.bootstrap.error?.message ?? null}
        onSubmit={async (payload) => {
          await actions.bootstrap.mutateAsync(payload);
          await navigate({ to: "/" });
        }}
      />
    </AuthShell>
  );
}
