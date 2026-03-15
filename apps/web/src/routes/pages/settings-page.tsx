import { useSaveSettings, useSession, useSettings } from "@/api/hooks";
import { SubviewShell } from "@/features/layout/subview-shell";
import { SettingsForm } from "@/features/settings/settings-form";

export function SettingsPage() {
  const session = useSession(false);
  const settings = useSettings(Boolean(session.data?.user));
  const saveSettings = useSaveSettings();
  return (
    <SubviewShell
      description="集中配置公开地址、RSSHub 默认值和 OpenAI 兼容 AI provider。"
      eyebrow="Settings"
      title="Runtime configuration"
    >
      <SettingsForm
        busy={saveSettings.isPending}
        initialValue={settings.data}
        onSubmit={async (payload) => {
          await saveSettings.mutateAsync(payload);
        }}
      />
    </SubviewShell>
  );
}
