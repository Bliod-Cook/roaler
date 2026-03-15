import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { AuthShell } from "./auth-shell";

describe("AuthShell", () => {
  it("renders the provided heading and content", () => {
    render(
      <AuthShell
        accentLabel="Bootstrap"
        description="Create the first admin user."
        eyebrow="Setup"
        title="Create the first and only admin."
      >
        <div>Form content</div>
      </AuthShell>,
    );

    expect(screen.getByText("Create the first and only admin.")).toBeInTheDocument();
    expect(screen.getByText("Form content")).toBeInTheDocument();
    expect(screen.getAllByText("Bootstrap")[0]).toBeInTheDocument();
  });
});
