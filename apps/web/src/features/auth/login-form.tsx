import { useState } from "react";

import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import { Icon } from "@/components/ui/icons";
import { Input } from "@/components/ui/input";

type LoginFormProps = {
  busy: boolean;
  error: string | null;
  onSubmit: (payload: { email: string; password: string }) => Promise<void>;
};

export function LoginForm({ busy, error, onSubmit }: LoginFormProps) {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    await onSubmit({ email, password });
  }

  return (
    <form className="space-y-5" onSubmit={handleSubmit}>
      <div className="grid gap-5">
        <Field hint="使用初始化阶段创建的管理员邮箱。" htmlFor="login-email" label="Email">
          <Input
            autoComplete="email"
            id="login-email"
            placeholder="admin@example.com"
            type="email"
            value={email}
            onChange={(event) => setEmail(event.target.value)}
          />
        </Field>
        <Field hint="当前仅支持单管理员会话登录。" htmlFor="login-password" label="Password">
          <Input
            autoComplete="current-password"
            id="login-password"
            placeholder="••••••••"
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
        <Icon name="check" />
        {busy ? "Signing in…" : "Sign in"}
      </Button>
    </form>
  );
}
