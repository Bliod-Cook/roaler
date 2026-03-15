import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Button } from "@/components/ui/button";

import { SubviewShell } from "./subview-shell";

describe("SubviewShell", () => {
  it("renders header copy, actions, and children", () => {
    render(
      <SubviewShell
        actions={<Button>Action</Button>}
        description="Search and manage your content."
        eyebrow="Search"
        title="Search the whole archive"
      >
        <div>Subview body</div>
      </SubviewShell>,
    );

    expect(screen.getByText("Search the whole archive")).toBeInTheDocument();
    expect(screen.getByText("Search and manage your content.")).toBeInTheDocument();
    expect(screen.getByText("Action")).toBeInTheDocument();
    expect(screen.getByText("Subview body")).toBeInTheDocument();
  });
});
