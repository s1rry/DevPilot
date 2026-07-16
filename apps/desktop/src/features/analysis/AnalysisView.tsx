import { AlertCircle, Loader2, ScanSearch } from "lucide-react";

import { Button } from "@/shared/ui/Button";
import { ScanReportView } from "@/features/analysis/components/ScanReportView";
import { useAnalysisStore } from "@/features/analysis/store";
import { useT } from "@/lib/store/i18n";

/**
 * Repository Scanner view. Scans a chosen project folder and reports its
 * languages, frameworks, dependencies, structure and git information. No code
 * analysis or AI — pure manifest and git detection.
 */
export function AnalysisView() {
  const status = useAnalysisStore((state) => state.status);
  const report = useAnalysisStore((state) => state.report);
  const scannedPath = useAnalysisStore((state) => state.scannedPath);
  const error = useAnalysisStore((state) => state.error);
  const scanDialog = useAnalysisStore((state) => state.scanDialog);
  const t = useT();

  const scanning = status === "scanning";

  return (
    <div className="mx-auto flex h-full w-full max-w-3xl flex-col gap-6 p-6">
      <section className="flex flex-col gap-3">
        <div className="flex items-center gap-3">
          <Button
            variant="primary"
            icon={scanning ? Loader2 : ScanSearch}
            onClick={() => void scanDialog()}
            disabled={scanning}
          >
            {scanning ? t("scan.scanning") : t("scan.scanFolder")}
          </Button>
          {scannedPath && !scanning && (
            <span className="min-w-0 truncate text-xs text-muted">{scannedPath}</span>
          )}
        </div>
      </section>

      {error && (
        <div className="flex items-start gap-2 rounded-md border border-border bg-surface px-3 py-2 text-sm text-fg">
          <AlertCircle size={16} strokeWidth={2} className="mt-0.5 shrink-0 text-accent" />
          <span className="min-w-0 break-words">{error}</span>
        </div>
      )}

      {report ? (
        <ScanReportView report={report} />
      ) : (
        !error && (
          <div className="flex flex-1 flex-col items-center justify-center gap-3 p-8 text-center">
            <div className="flex h-14 w-14 items-center justify-center rounded-xl border border-border bg-surface text-muted">
              <ScanSearch size={26} strokeWidth={1.75} />
            </div>
            <p className="max-w-sm text-sm text-muted">{t("scan.emptyHint")}</p>
          </div>
        )
      )}
    </div>
  );
}
