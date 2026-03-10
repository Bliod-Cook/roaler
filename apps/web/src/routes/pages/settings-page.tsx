import { useSaveSettings, useSession, useSettings } from "@/api/hooks";
import { SectionHeader } from "@/components/shell/section-header";
import { SettingsForm } from "@/features/settings/settings-form";

export function SettingsPage() {
  const session = useSession(false);
  const settings = useSettings(Boolean(session.data?.user));
  const saveSettings = useSaveSettings();
  return (
    <div className="space-y-6">
      <SectionHeader
        eyebrow="Settings"
        title="Runtime configuration"
        description="这里集中放公开地址、默认 RSSHub 与 AI provider 配置。"
      />
      <SettingsForm
        busy={saveSettings.isPending}
        initialValue={settings.data}
        onSubmit={async (payload) => {
          await saveSettings.mutateAsync(payload);
        }}
      />
    </div>
  );
}
