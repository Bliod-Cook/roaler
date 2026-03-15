import type { EntryDetail } from "@/api/types";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Icon } from "@/components/ui/icons";

type EntryDetailPanelProps = {
  detail: EntryDetail | undefined;
  hasSelection: boolean;
  busy: boolean;
  onStateChange: (field: "read" | "starred" | "saved", value: boolean) => void;
  onQueueTask: (taskType: "entry_summary" | "entry_translation" | "entry_topic_tags") => void;
};

export function EntryDetailPanel({
  detail,
  busy,
  hasSelection,
  onStateChange,
  onQueueTask,
}: EntryDetailPanelProps) {
  if (!hasSelection) {
    return (
      <Card className="flex min-h-[420px] flex-col justify-between rounded-panel p-0 lg:h-[calc(100vh-2rem)]">
        <div className="border-b panel-divider px-6 py-5">
          <p className="eyebrow">Details</p>
          <h2 className="mt-3 font-display text-2xl text-text">Choose an entry to read</h2>
        </div>
        <div className="flex flex-1 flex-col items-center justify-center gap-4 px-8 text-center">
          <div className="flex size-14 items-center justify-center rounded-[1.75rem] bg-accent-soft text-accent">
            <Icon className="size-6" name="entry" />
          </div>
          <div className="space-y-2">
            <p className="text-lg font-semibold text-text">The right pane stays focused.</p>
            <p className="mx-auto max-w-sm text-sm leading-7 text-text-secondary">
              从中间时间线选择一条内容后，这里会显示摘要、正文、翻译和标记动作。
            </p>
          </div>
        </div>
      </Card>
    );
  }

  if (!detail) {
    return (
      <Card className="flex min-h-[420px] items-center justify-center rounded-panel lg:h-[calc(100vh-2rem)]">
        加载中…
      </Card>
    );
  }

  return (
    <Card className="flex min-h-[520px] flex-col rounded-panel p-0 lg:h-[calc(100vh-2rem)]">
      <div className="border-b panel-divider px-6 py-5">
        <div className="flex flex-wrap items-center gap-2">
          <Badge tone="accent">{detail.source_title}</Badge>
          <Badge tone={detail.is_read ? "neutral" : "success"}>
            {detail.is_read ? "Read" : "Unread"}
          </Badge>
          {detail.is_starred ? <Badge tone="accent">Starred</Badge> : null}
          {detail.is_saved ? <Badge>Saved</Badge> : null}
        </div>
        <h1 className="mt-4 max-w-3xl font-display text-3xl leading-tight text-text">
          {detail.title}
        </h1>
        <p className="mt-4 max-w-3xl text-sm leading-7 text-text-secondary">
          {detail.ai_summary ?? detail.summary ?? "尚未生成摘要。"}
        </p>
      </div>
      <div className="thin-scrollbar flex-1 space-y-6 overflow-y-auto px-6 py-5">
        <div className="flex flex-wrap gap-3">
          <Button onClick={() => onStateChange("read", !detail.is_read)} variant="secondary">
            <Icon name="check" />
            {detail.is_read ? "Mark unread" : "Mark read"}
          </Button>
          <Button onClick={() => onStateChange("starred", !detail.is_starred)} variant="secondary">
            <Icon name="star" />
            {detail.is_starred ? "Unstar" : "Star"}
          </Button>
          <Button onClick={() => onStateChange("saved", !detail.is_saved)} variant="secondary">
            <Icon name="bookmark" />
            {detail.is_saved ? "Unsave" : "Save for later"}
          </Button>
        </div>
        <div className="flex flex-wrap gap-3">
          <Button disabled={busy} onClick={() => onQueueTask("entry_summary")}>
            <Icon name="sparkles" />
            Generate summary
          </Button>
          <Button
            disabled={busy}
            onClick={() => onQueueTask("entry_translation")}
            variant="secondary"
          >
            <Icon name="translate" />
            Translate
          </Button>
          <Button
            disabled={busy}
            onClick={() => onQueueTask("entry_topic_tags")}
            variant="secondary"
          >
            <Icon name="collections" />
            Tag topics
          </Button>
        </div>
        <section className="space-y-3">
          <div className="flex items-center justify-between">
            <p className="eyebrow">Text</p>
            {detail.url ? (
              <a
                className="text-sm font-semibold text-accent"
                href={detail.url}
                rel="noreferrer"
                target="_blank"
              >
                Open source
              </a>
            ) : null}
          </div>
          <div className="detail-prose rounded-card border border-line/70 bg-panel-soft/70 p-5">
            <p className="whitespace-pre-wrap">
              {detail.text_content ?? "正文尚未抽取。"}
            </p>
          </div>
        </section>
        <section className="space-y-3">
          <p className="eyebrow">Translation</p>
          <div className="detail-prose rounded-card border border-line/70 bg-panel-soft/70 p-5">
            <p className="whitespace-pre-wrap">
              {detail.ai_translation ?? "尚未生成翻译。"}
            </p>
          </div>
        </section>
      </div>
    </Card>
  );
}
