import { expect, test } from "@playwright/test";

test("renders shell", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator("body")).toBeVisible();
});
