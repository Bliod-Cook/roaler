import { useState } from "react";

import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import { Icon } from "@/components/ui/icons";
import { Input } from "@/components/ui/input";

type SetupFormProps = {
  busy: boolean;
  error: string | null;
  onSubmit: (payload: {
    email: string;
    password: string;
    display_name: string;
  }) => Promise<void>;
};

export function SetupForm({ busy, error, onSubmit }: SetupFormProps) {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [displayName, setDisplayName] = useState("");

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    await onSubmit({ email, password, display_name: displayName });
  }

  return (
    <form className="space-y-5" onSubmit={handleSubmit}>
      <div className="grid gap-5">
        <Field hint="展示在侧边栏和设置页的名字。" htmlFor="setup-display-name" label="Display name">
          <Input
            autoComplete="name"
            id="setup-display-name"
            placeholder="Roaler Admin"
            value={displayName}
            onChange={(event) => setDisplayName(event.target.value)}
          />
        </Field>
        <Field hint="后续登录将使用这个邮箱。" htmlFor="setup-email" label="Email">
          <Input
            autoComplete="email"
            id="setup-email"
            placeholder="admin@example.com"
            type="email"
            value={email}
            onChange={(event) => setEmail(event.target.value)}
          />
        </Field>
        <Field hint="建议使用密码管理器生成高强度密码。" htmlFor="setup-password" label="Password">
          <Input
            autoComplete="new-password"
            id="setup-password"
            placeholder="Create a strong password"
            type="password"
            value={password}
            onChange={(event) => setPassword(event.target.value)}
          />
        </Field>
      </div>
      {error ? (
        <div className="rounded-card border border-danger/25 bg-danger/10 p-3 text-sm text-danger">
          {error}
        </div>
      ) : null}
      <Button className="w-full" disabled={busy} size="lg" type="submit">
        <Icon name="plus" />
        {busy ? "Creating…" : "Create admin"}
      </Button>
    </form>
  );
}
