import { useEffect } from "react";
import { AlertCircle, Check } from "lucide-react";

import { Button } from "@/shared/ui/Button";
import { PROVIDERS, needsApiKey } from "@/lib/ipc/settings";
import { isTauri } from "@/lib/ipc/env";
import { useSettingsStore } from "@/features/settings/store";
import { useT } from "@/lib/store/i18n";

/**
 * Provider settings: pick the active LLM provider and model, and enter API
 * keys. Values persist to the backend settings store.
 */
export function SettingsView() {
  const settings = useSettingsStore((state) => state.settings);
  const saving = useSettingsStore((state) => state.saving);
  const savedAt = useSettingsStore((state) => state.savedAt);
  const error = useSettingsStore((state) => state.error);
  const load = useSettingsStore((state) => state.load);
  const update = useSettingsStore((state) => state.update);
  const setKey = useSettingsStore((state) => state.setKey);
  const save = useSettingsStore((state) => state.save);
  const t = useT();

  useEffect(() => {
    if (isTauri()) {
      void load();
    }
  }, [load]);

  if (!settings) {
    return (
      <div className="mx-auto flex h-full w-full max-w-xl flex-col gap-6 p-6">
        <p className="text-sm text-muted">{t("settings.loading")}</p>
      </div>
    );
  }

  const requiresKey = needsApiKey(settings.provider);

  return (
    <div className="mx-auto flex h-full w-full max-w-xl flex-col gap-6 p-6">
      <section className="flex flex-col gap-4">
        <h2 className="text-sm font-medium text-muted">{t("settings.aiProvider")}</h2>

        <label className="flex flex-col gap-1.5">
          <span className="text-xs text-muted">{t("settings.provider")}</span>
          <select
            value={settings.provider}
            onChange={(event) => update({ provider: event.target.value as never })}
            className="h-9 rounded-md border border-border bg-canvas px-2 text-sm text-fg outline-none focus-visible:ring-2 focus-visible:ring-accent"
          >
            {PROVIDERS.map((provider) => (
              <option key={provider} value={provider}>
                {provider}
              </option>
            ))}
          </select>
        </label>

        <label className="flex flex-col gap-1.5">
          <span className="text-xs text-muted">{t("settings.model")}</span>
          <input
            value={settings.model}
            onChange={(event) => update({ model: event.target.value })}
            placeholder={t("settings.modelPlaceholder")}
            className="h-9 rounded-md border border-border bg-canvas px-3 text-sm text-fg outline-none placeholder:text-muted focus-visible:ring-2 focus-visible:ring-accent"
          />
        </label>

        {requiresKey && (
          <label className="flex flex-col gap-1.5">
            <span className="text-xs text-muted">
              {t("settings.apiKeyLabel", { provider: settings.provider })}
            </span>
            <input
              type="password"
              value={settings.api_keys[settings.provider] ?? ""}
              onChange={(event) => setKey(settings.provider, event.target.value)}
              placeholder={t("settings.apiKeyPlaceholder")}
              className="h-9 rounded-md border border-border bg-canvas px-3 text-sm text-fg outline-none placeholder:text-muted focus-visible:ring-2 focus-visible:ring-accent"
            />
            <span className="text-xs text-muted">{t("settings.apiKeyStored")}</span>
          </label>
        )}

        <div className="flex items-center gap-3">
          <Button variant="primary" onClick={() => void save()} disabled={saving}>
            {saving ? t("settings.saving") : t("settings.save")}
          </Button>
          {savedAt && !saving && (
            <span className="flex items-center gap-1 text-xs text-muted">
              <Check size={14} strokeWidth={2} className="text-accent" />
              {t("settings.saved")}
            </span>
          )}
        </div>

        {error && (
          <div className="flex items-start gap-2 rounded-md border border-border bg-surface px-3 py-2 text-sm text-fg">
            <AlertCircle size={16} strokeWidth={2} className="mt-0.5 shrink-0 text-accent" />
            <span className="min-w-0 break-words">{error}</span>
          </div>
        )}
      </section>
    </div>
  );
}
