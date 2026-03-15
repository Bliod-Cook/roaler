import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./tests/e2e",
  fullyParallel: false,
  webServer: {
    command: "bash tests/e2e/start-e2e-stack.sh",
    port: 4173,
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
  use: {
    baseURL: "http://127.0.0.1:4173",
    headless: true,
  },
});
