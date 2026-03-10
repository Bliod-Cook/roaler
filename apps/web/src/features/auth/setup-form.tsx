import { useState } from "react";

import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
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
    <Card className="mx-auto max-w-xl">
      <p className="eyebrow">Bootstrap</p>
      <h1 className="mt-3 font-display text-4xl">Create the only admin.</h1>
      <p className="mt-2 text-sm leading-7 text-ink/70">
        首次启动后只创建一个管理员账号，后续全部通过会话登录。
      </p>
      <form className="mt-8 space-y-4" onSubmit={handleSubmit}>
        <Input
          autoComplete="name"
          placeholder="Display name"
          value={displayName}
          onChange={(event) => setDisplayName(event.target.value)}
        />
        <Input
          autoComplete="email"
          placeholder="Email"
          type="email"
          value={email}
          onChange={(event) => setEmail(event.target.value)}
        />
        <Input
          autoComplete="new-password"
          placeholder="Password"
          type="password"
          value={password}
          onChange={(event) => setPassword(event.target.value)}
        />
        {error ? <p className="text-sm text-red-600">{error}</p> : null}
        <Button className="w-full" disabled={busy} type="submit">
          {busy ? "Creating…" : "Create admin"}
        </Button>
      </form>
    </Card>
  );
}
