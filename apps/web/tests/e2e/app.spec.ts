import { expect, test } from "@playwright/test";

const admin = {
  email: "admin@roaler.test",
  password: "RoalerPass123!",
  displayName: "Roaler Admin",
};

async function login(page: import("@playwright/test").Page) {
  await page.goto("/login");
  await page.getByLabel("Email").fill(admin.email);
  await page.getByLabel("Password").fill(admin.password);
  await page.getByRole("button", { name: "Sign in" }).click();
  await expect(page.getByText("Latest feed flow")).toBeVisible();
}

test.describe.serial("roaler folo shell", () => {
  test("bootstraps the first admin from setup", async ({ page }) => {
    await page.goto("/setup");
    const setupHeading = page.getByText("Create the first and only admin.");

    if (await setupHeading.isVisible().catch(() => false)) {
      await page.getByLabel("Display name").fill(admin.displayName);
      await page.getByLabel("Email").fill(admin.email);
      await page.getByLabel("Password").fill(admin.password);
      await page.getByRole("button", { name: "Create admin" }).click();
    } else {
      await expect(page).toHaveURL(/\/login$/);
      await page.getByLabel("Email").fill(admin.email);
      await page.getByLabel("Password").fill(admin.password);
      await page.getByRole("button", { name: "Sign in" }).click();
    }

    await expect(page.getByText("Latest feed flow")).toBeVisible();
  });

  test("shows login and subview navigation", async ({ page }) => {
    await login(page);
    await page.getByRole("link", { name: /Sources/i }).click();
    await expect(page.getByText("Bring RSS and RSSHub into the desk")).toBeVisible();
    await page.getByRole("link", { name: /Collections/i }).click();
    await expect(page.getByText("Group subscriptions into reusable views")).toBeVisible();
    await page.getByRole("link", { name: /Search/i }).click();
    await expect(page.getByText("Search the whole archive")).toBeVisible();
    await page.getByRole("link", { name: /Settings/i }).click();
    await expect(page.getByText("Runtime configuration")).toBeVisible();
  });

  test("creates a source and opens the synced entry detail pane", async ({ page }) => {
    await login(page);
    await page.goto("/sources");
    const seededFeedUrl = page.getByText("http://127.0.0.1:9324/feed.xml");

    if (!(await seededFeedUrl.isVisible().catch(() => false))) {
      await page.getByRole("button", { name: "RSS / Atom" }).click();
      await page.getByLabel("Feed URL").fill("http://127.0.0.1:9324/feed.xml");
      await page.getByRole("main").getByRole("button", { name: "Add source" }).click();
      await expect(seededFeedUrl).toBeVisible();
    }

    await expect
      .poll(async () => {
        const response = await page.context().request.get("/api/entries");
        if (!response.ok()) {
          return "request-failed";
        }
        const entries = (await response.json()) as Array<{ title: string }>;
        return entries[0]?.title ?? "empty";
      }, { timeout: 25_000 })
      .toBe("Roaler fixture entry");

    await page.goto("/");
    await page.getByRole("link", { name: /Roaler fixture entry/i }).click();
    await expect(page.getByRole("heading", { name: "Roaler fixture entry" })).toBeVisible();
    await expect(page.getByText("This entry is served by the local fixture feed")).toBeVisible();
  });
});
