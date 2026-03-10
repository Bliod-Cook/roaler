type SectionHeaderProps = {
  eyebrow: string;
  title: string;
  description: string;
};

export function SectionHeader({ eyebrow, title, description }: SectionHeaderProps) {
  return (
    <header className="mb-6 space-y-2">
      <p className="eyebrow">{eyebrow}</p>
      <h1 className="font-display text-4xl leading-tight text-ink">{title}</h1>
      <p className="max-w-2xl text-sm leading-7 text-ink/70">{description}</p>
    </header>
  );
}
