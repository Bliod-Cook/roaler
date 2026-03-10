import { describe, expect, it } from "vitest";

import { cn } from "./cn";

describe("cn", () => {
  it("merges classes", () => {
    expect(cn("px-4", "px-2", "font-medium")).toContain("px-2");
  });
});
