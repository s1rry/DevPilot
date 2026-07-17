/**
 * English translation dictionary.
 *
 * This is the reference language: its shape (the union of keys) is exported as
 * `TranslationKey` and every other language must provide exactly the same keys.
 * Keys are grouped by area with a dotted prefix (`nav.*`, `topbar.*`, ...).
 */
export const en = {
  // Top bar
  "topbar.searchPlaceholder": "Search — coming soon",
  "topbar.toggleSidebar": "Toggle sidebar",
  "topbar.switchToLight": "Switch to light theme",
  "topbar.switchToDark": "Switch to dark theme",
  "topbar.switchToRussian": "Switch to Russian",
  "topbar.switchToEnglish": "Switch to English",

  // Navigation
  "nav.primary": "Primary",

  // Navigation: labels
  "nav.repository.label": "Repository",
  "nav.analysis.label": "Analysis",
  "nav.architecture.label": "Architecture",
  "nav.ai-chat.label": "AI Chat",
  "nav.insights.label": "Insights",
  "nav.settings.label": "Settings",

  // Navigation: one-line hints (shown in empty states)
  "nav.repository.hint": "Open a local folder or a GitHub URL to explore its file tree.",
  "nav.analysis.hint": "Code metrics, complexity and structure from tree-sitter.",
  "nav.architecture.hint": "Interactive dependency, module, folder and call graphs.",
  "nav.ai-chat.hint": "Ask questions about the codebase with Ollama, Claude, OpenAI or Gemini.",
  "nav.insights.hint": "Reports on hotspots, risks and code quality trends.",
  "nav.settings.hint": "AI providers, API keys and appearance.",

  // Repository view
  "repo.openProject": "Open a project",
  "repo.recentProjects": "Recent projects",
  "repo.noRecent": "No recent projects yet.",
  "repo.removeFromRecent": "Remove from recent",
  "repo.removeAria": "Remove {name} from recent",
  "repo.cloneUrlPlaceholder": "https://github.com/owner/repo",
  "repo.openFolder": "Open folder",
  "repo.clone": "Clone",
  "repo.cloning": "Cloning…",

  // Project metadata panel
  "meta.branch": "Branch",
  "meta.commits": "Commits",
  "meta.files": "Files",
  "meta.size": "Size",
  "meta.languages": "Languages",

  // Analysis / scanner view
  "scan.scanFolder": "Scan a folder",
  "scan.scanning": "Scanning…",
  "scan.emptyHint":
    "Scan a project folder to detect its languages, frameworks, dependencies, structure and git history.",
  "scan.noFiles": "No files detected.",
  "scan.branch": "Branch",
  "scan.commits": "Commits",
  "scan.files": "Files",
  "scan.directories": "Directories",
  "scan.languages": "Languages",
  "scan.frameworks": "Frameworks",
  "scan.noFrameworks": "No frameworks detected.",
  "scan.dependencies": "Dependencies",
  "scan.noDependencies": "No dependencies detected.",
  "scan.structure": "Structure",
  "scan.topLevelDirs": "Top-level directories: {count}",
  "scan.git": "Git",
  "scan.topContributors": "Top contributors",

  // AI Chat view
  "chat.changeProject": "Change project",
  "chat.chooseProject": "Choose project",
  "chat.clear": "Clear",
  "chat.emptyWithProject":
    "Ask a question about this repository. Replies stream in, with Markdown and code blocks.",
  "chat.emptyNoProject": "Choose a project folder, then ask questions about its code.",
  "chat.thinking": "Thinking…",
  "chat.inputPlaceholder": "Ask about this repository…",
  "chat.send": "Send",
  "chat.sendMessage": "Send message",

  // Settings view
  "settings.loading": "Loading settings…",
  "settings.aiProvider": "AI provider",
  "settings.provider": "Provider",
  "settings.model": "Model",
  "settings.modelPlaceholder": "e.g. llama3, claude-sonnet-4, gpt-4o",
  "settings.apiKeyLabel": "{provider} API key",
  "settings.apiKeyPlaceholder": "Paste your API key",
  "settings.apiKeyStored": "Stored locally in your app data folder.",
  "settings.save": "Save",
  "settings.saving": "Saving…",
  "settings.saved": "Saved",

  // Shared: project picker + analyze (used by several views)
  "common.changeProject": "Change project",
  "common.chooseProject": "Choose project",
  "common.analyze": "Analyze",
  "common.analyzing": "Analyzing…",
  "common.noneFound": "None found.",

  // Insights view
  "insights.searchPlaceholder": "Where is authentication? Where is the database initialized?",
  "insights.search": "Search",
  "insights.searchResults": "Search results",
  "insights.explain": "Explain",
  "insights.cyclicDependencies": "Cyclic dependencies",
  "insights.deadCode": "Dead code",
  "insights.duplication": "Duplication",
  "insights.emptyHint":
    "Choose a project and run Analyze to find cyclic dependencies, dead code and duplication — or search to locate code.",

  // Architecture view
  "arch.graph.dependency": "Dependencies",
  "arch.graph.module": "Modules",
  "arch.graph.folder": "Folders",
  "arch.graph.call": "Calls",
  "arch.legend.file": "File",
  "arch.legend.module": "Module",
  "arch.legend.directory": "Directory",
  "arch.legend.function": "Function",
  "arch.legend.external": "External",
  "arch.emptyHint":
    "Choose a project and run Analyze to see its dependency, module, folder and call graphs. Drag nodes, scroll to zoom, drag the background to pan.",
  "arch.graphEmpty": "This graph is empty.",
  "arch.nodesLimited": "Showing the {max} most connected of {total} nodes",

  // Status bar
  "status.noRepository": "No repository",
  "status.ready": "Ready",

  // Auto-update
  "updater.title": "Update",
  "updater.available": "Version {version} is available. Install now?",
  "updater.install": "Install",
  "updater.later": "Later",
} as const;

/** The set of valid translation keys, derived from the reference language. */
export type TranslationKey = keyof typeof en;

/** A complete dictionary: every key of the reference language, translated. */
export type Dictionary = Record<TranslationKey, string>;
