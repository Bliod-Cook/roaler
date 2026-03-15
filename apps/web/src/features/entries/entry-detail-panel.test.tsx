import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import type { EntryDetail } from "@/api/types";

import { EntryDetailPanel } from "./entry-detail-panel";

const detail: EntryDetail = {
  ai_summary: "AI summary",
  ai_tags: [],
  ai_translation: "Translated content",
  author_name: "Author",
  content_error: null,
  content_status: "ready",
  html_content: "<p>HTML</p>",
  id: "entry-1",
  is_read: false,
  is_saved: true,
  is_starred: true,
  media_json: [],
  published_at: "2026-03-15T08:00:00Z",
  raw_payload: {},
  source_id: "source-1",
  source_title: "Seeded Integration Feed",
  summary: "Fallback summary",
  text_content: "Body copy",
  title: "Roaler fixture entry",
  url: "https://example.com/fixture-entry",
};

describe("EntryDetailPanel", () => {
  it("renders placeholder when no entry is selected", () => {
    render(
      <EntryDetailPanel
        busy={false}
        detail={undefined}
        hasSelection={false}
        onQueueTask={vi.fn()}
        onStateChange={vi.fn()}
      />,
    );

    expect(screen.getByText("Choose an entry to read")).toBeInTheDocument();
  });

  it("renders detail data when an entry is selected", () => {
    render(
      <EntryDetailPanel
        busy={false}
        detail={detail}
        hasSelection
        onQueueTask={vi.fn()}
        onStateChange={vi.fn()}
      />,
    );

    expect(screen.getByRole("heading", { name: "Roaler fixture entry" })).toBeInTheDocument();
    expect(screen.getByText("Translated content")).toBeInTheDocument();
    expect(screen.getByText("Seeded Integration Feed")).toBeInTheDocument();
  });
});
