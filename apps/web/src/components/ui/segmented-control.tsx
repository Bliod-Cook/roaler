import { cn } from "@/utils/cn";

type SegmentedItem<T extends string> = {
  label: string;
  value: T;
};

type SegmentedControlProps<T extends string> = {
  items: SegmentedItem<T>[];
  value: T;
  onChange: (value: T) => void;
};

export function SegmentedControl<T extends string>({
  items,
  onChange,
  value,
}: SegmentedControlProps<T>) {
  return (
    <div className="inline-grid min-h-11 w-full grid-cols-2 gap-1 rounded-[1.25rem] border border-line bg-panel-soft p-1">
      {items.map((item) => {
        const active = item.value === value;
        return (
          <button
            key={item.value}
            className={cn(
              "rounded-[1rem] px-3 py-2 text-sm font-semibold transition-colors",
              active ? "bg-panel text-text shadow-sm" : "text-text-secondary hover:text-text",
            )}
            onClick={() => onChange(item.value)}
            type="button"
          >
            {item.label}
          </button>
        );
      })}
    </div>
  );
}
