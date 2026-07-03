/**
 * Root component of the DevPilot desktop app.
 *
 * The skeleton intentionally renders a static shell only. Feature slices
 * from `src/features/*` will mount here as they land (see the roadmap).
 */
export default function App() {
  return (
    <main className="flex h-full flex-col items-center justify-center gap-3">
      <h1 className="text-4xl font-semibold tracking-tight">DevPilot</h1>
      <p className="text-sm" style={{ color: "var(--color-text-muted)" }}>
        AI-powered repository analyzer — project skeleton
      </p>
    </main>
  );
}
