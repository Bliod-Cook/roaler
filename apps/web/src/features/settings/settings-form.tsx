import { useEffect, useState } from "react";

import type { SystemSettings } from "@/api/types";
import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import { Icon } from "@/components/ui/icons";
import { Input } from "@/components/ui/input";

type SettingsFormProps = {
  initialValue: SystemSettings | undefined;
  busy: boolean;
  onSubmit: (payload: SystemSettings) => Promise<void>;
};

export function SettingsForm({ initialValue, busy, onSubmit }: SettingsFormProps) {
  const [value, setValue] = useState<SystemSettings>({
    public_base_url: null,
    default_rsshub_base_url: "",
    ai: null,
  });

  useEffect(() => {
    if (initialValue) {
      setValue(initialValue);
    }
  }, [initialValue]);

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    await onSubmit(value);
  }

  return (
    <form className="grid gap-6 xl:grid-cols-[1fr_1fr]" onSubmit={handleSubmit}>
      <section className="rounded-card border border-line/70 bg-panel-soft/65 p-5">
        <p className="eyebrow">System</p>
        <div className="mt-4 space-y-5">
          <Field hint="用于 webhook 和外部回调的公开访问地址。" htmlFor="settings-public-base-url" label="Public base URL">
            <Input
              id="settings-public-base-url"
              placeholder="https://feeds.example.com"
              value={value.public_base_url ?? ""}
              onChange={(event) =>
                setValue({ ...value, public_base_url: event.target.value || null })
              }
            />
          </Field>
          <Field hint="为 RSSHub route 提供默认基础域名。" htmlFor="settings-default-rsshub-base-url" label="Default RSSHub base URL">
            <Input
              id="settings-default-rsshub-base-url"
              placeholder="https://rsshub.app"
              value={value.default_rsshub_base_url}
              onChange={(event) =>
                setValue({ ...value, default_rsshub_base_url: event.target.value })
              }
            />
          </Field>
        </div>
      </section>
      <section className="rounded-card border border-line/70 bg-panel-soft/65 p-5">
        <p className="eyebrow">AI</p>
        <div className="mt-4 space-y-5">
          <Field hint="兼容 OpenAI 的推理服务地址。" htmlFor="settings-ai-base-url" label="AI base URL">
            <Input
              id="settings-ai-base-url"
              placeholder="https://api.openai.com/v1"
              value={value.ai?.base_url ?? ""}
              onChange={(event) =>
                setValue({
                  ...value,
                  ai: {
                    base_url: event.target.value,
                    api_key: value.ai?.api_key ?? "",
                    model: value.ai?.model ?? "",
                    timeout_seconds: value.ai?.timeout_seconds ?? 60,
                  },
                })
              }
            />
          </Field>
          <Field hint="不会写入源码，只存到运行时配置。" htmlFor="settings-ai-api-key" label="AI API key">
            <Input
              id="settings-ai-api-key"
              placeholder="sk-..."
              value={value.ai?.api_key ?? ""}
              onChange={(event) =>
                setValue({
                  ...value,
                  ai: {
                    base_url: value.ai?.base_url ?? "",
                    api_key: event.target.value,
                    model: value.ai?.model ?? "",
                    timeout_seconds: value.ai?.timeout_seconds ?? 60,
                  },
                })
              }
            />
          </Field>
          <Field hint="摘要、翻译和标签任务默认使用的模型。" htmlFor="settings-ai-model" label="AI model">
            <Input
              id="settings-ai-model"
              placeholder="gpt-4.1-mini"
              value={value.ai?.model ?? ""}
              onChange={(event) =>
                setValue({
                  ...value,
                  ai: {
                    base_url: value.ai?.base_url ?? "",
                    api_key: value.ai?.api_key ?? "",
                    model: event.target.value,
                    timeout_seconds: value.ai?.timeout_seconds ?? 60,
                  },
                })
              }
            />
          </Field>
        </div>
      </section>
      <div className="xl:col-span-2">
        <Button className="w-full xl:w-auto" disabled={busy} type="submit">
          <Icon name="check" />
          {busy ? "Saving…" : "Save settings"}
        </Button>
      </div>
    </form>
  );
}
