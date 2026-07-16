import type { LanguageStat } from "@/lib/ipc/repository";
import { useT } from "@/lib/store/i18n";

interface LanguageBarsProps {
  /** Per-language file counts. */
  languages: LanguageStat[];
}

/** A horizontal bar chart of file counts per language. */
export function LanguageBars({ languages }: LanguageBarsProps) {
  const t = useT();
  const total = languages.reduce((sum, stat) => sum + stat.file_count, 0);
  if (total === 0) {
    return <p className="text-sm text-muted">{t("scan.noFiles")}</p>;
  }

  return (
    <div className="flex flex-col gap-1.5">
      {languages.map((stat) => {
        const percent = (stat.file_count / total) * 100;
        return (
          <div key={stat.language} className="flex items-center gap-2">
            <span className="w-24 shrink-0 truncate text-xs text-fg">{stat.language}</span>
            <div className="h-2 flex-1 overflow-hidden rounded-full bg-elevated">
              <div className="h-full rounded-full bg-accent" style={{ width: `${percent}%` }} />
            </div>
            <span className="w-10 shrink-0 text-right text-xs text-muted">{stat.file_count}</span>
          </div>
        );
      })}
    </div>
  );
}
