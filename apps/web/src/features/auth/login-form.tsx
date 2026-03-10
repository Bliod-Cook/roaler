import { useState } from "react";

import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
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
    <Card className="mx-auto max-w-xl">
      <p className="eyebrow">Session</p>
      <h1 className="mt-3 font-display text-4xl">Return to your reading desk.</h1>
      <form className="mt-8 space-y-4" onSubmit={handleSubmit}>
        <Input
          autoComplete="email"
          placeholder="Email"
          type="email"
          value={email}
          onChange={(event) => setEmail(event.target.value)}
        />
        <Input
          autoComplete="current-password"
          placeholder="Password"
          type="password"
          value={password}
          onChange={(event) => setPassword(event.target.value)}
        />
        {error ? <p className="text-sm text-red-600">{error}</p> : null}
        <Button className="w-full" disabled={busy} type="submit">
          {busy ? "Signing in…" : "Sign in"}
        </Button>
      </form>
    </Card>
  );
}
