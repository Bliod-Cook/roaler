import { useEffect, useState } from "react";

import type { SystemSettings } from "@/api/types";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
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
    <Card>
      <p className="eyebrow">System</p>
      <form className="mt-4 space-y-4" onSubmit={handleSubmit}>
        <Input
          placeholder="Public base URL"
          value={value.public_base_url ?? ""}
          onChange={(event) =>
            setValue({ ...value, public_base_url: event.target.value || null })
          }
        />
        <Input
          placeholder="Default RSSHub base URL"
          value={value.default_rsshub_base_url}
          onChange={(event) =>
            setValue({ ...value, default_rsshub_base_url: event.target.value })
          }
        />
        <Input
          placeholder="AI base URL"
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
        <Input
          placeholder="AI API key"
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
        <Input
          placeholder="AI model"
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
        <Button className="w-full" disabled={busy} type="submit">
          {busy ? "Saving…" : "Save settings"}
        </Button>
      </form>
    </Card>
  );
}

