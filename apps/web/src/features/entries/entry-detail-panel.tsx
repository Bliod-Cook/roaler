import type { EntryDetail } from "@/api/types";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";

type EntryDetailPanelProps = {
  detail: EntryDetail | undefined;
  busy: boolean;
  onStateChange: (field: "read" | "starred" | "saved", value: boolean) => void;
  onQueueTask: (taskType: "entry_summary" | "entry_translation" | "entry_topic_tags") => void;
};

export function EntryDetailPanel({
  detail,
  busy,
  onStateChange,
  onQueueTask,
}: EntryDetailPanelProps) {
  if (!detail) {
    return <Card>加载中…</Card>;
  }
  return (
    <div className="space-y-6">
      <Card className="space-y-4">
        <p className="eyebrow">{detail.source_title}</p>
        <h1 className="font-display text-4xl leading-tight">{detail.title}</h1>
        <p className="text-sm leading-7 text-ink/70">
          {detail.ai_summary ?? detail.summary ?? "尚未生成摘要。"}
        </p>
        <div className="flex flex-wrap gap-3">
          <Button variant="secondary" onClick={() => onStateChange("read", !detail.is_read)}>
            {detail.is_read ? "Mark unread" : "Mark read"}
          </Button>
          <Button variant="secondary" onClick={() => onStateChange("starred", !detail.is_starred)}>
            {detail.is_starred ? "Unstar" : "Star"}
          </Button>
          <Button variant="secondary" onClick={() => onStateChange("saved", !detail.is_saved)}>
            {detail.is_saved ? "Unsave" : "Save for later"}
          </Button>
        </div>
      </Card>
      <Card className="space-y-4">
        <div className="flex flex-wrap gap-3">
          <Button disabled={busy} onClick={() => onQueueTask("entry_summary")}>
            Generate summary
          </Button>
          <Button disabled={busy} variant="secondary" onClick={() => onQueueTask("entry_translation")}>
            Translate
          </Button>
          <Button disabled={busy} variant="secondary" onClick={() => onQueueTask("entry_topic_tags")}>
            Tag topics
          </Button>
        </div>
        <div className="grid gap-4 lg:grid-cols-2">
          <article className="rounded-[1.5rem] bg-white/70 p-4">
            <p className="eyebrow">Text</p>
            <p className="mt-3 whitespace-pre-wrap text-sm leading-7 text-ink/75">
              {detail.text_content ?? "正文尚未抽取。"}
            </p>
          </article>
          <article className="rounded-[1.5rem] bg-white/70 p-4">
            <p className="eyebrow">Translation</p>
            <p className="mt-3 whitespace-pre-wrap text-sm leading-7 text-ink/75">
              {detail.ai_translation ?? "尚未生成翻译。"}
            </p>
          </article>
        </div>
      </Card>
    </div>
  );
}

