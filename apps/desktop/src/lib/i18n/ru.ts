import type { Dictionary } from "./en";

/**
 * Russian translation dictionary. Typed as `Dictionary`, so TypeScript fails
 * the build if any key from the reference language is missing or misspelled.
 */
export const ru: Dictionary = {
  // Top bar
  "topbar.searchPlaceholder": "Поиск — скоро",
  "topbar.toggleSidebar": "Свернуть боковую панель",
  "topbar.switchToLight": "Светлая тема",
  "topbar.switchToDark": "Тёмная тема",
  "topbar.switchToRussian": "Переключить на русский",
  "topbar.switchToEnglish": "Переключить на английский",

  // Navigation
  "nav.primary": "Основная навигация",

  // Navigation: labels
  "nav.repository.label": "Репозиторий",
  "nav.analysis.label": "Анализ",
  "nav.architecture.label": "Архитектура",
  "nav.ai-chat.label": "AI-чат",
  "nav.insights.label": "Обзор",
  "nav.settings.label": "Настройки",

  // Navigation: one-line hints (shown in empty states)
  "nav.repository.hint": "Откройте локальную папку или ссылку на GitHub, чтобы увидеть дерево файлов.",
  "nav.analysis.hint": "Метрики кода, сложность и структура из tree-sitter.",
  "nav.architecture.hint": "Интерактивные графы зависимостей, модулей, папок и вызовов.",
  "nav.ai-chat.hint": "Задавайте вопросы о коде с Ollama, Claude, OpenAI или Gemini.",
  "nav.insights.hint": "Отчёты о горячих точках, рисках и качестве кода.",
  "nav.settings.hint": "AI-провайдеры, API-ключи и внешний вид.",

  // Repository view
  "repo.openProject": "Открыть проект",
  "repo.recentProjects": "Недавние проекты",
  "repo.noRecent": "Пока нет недавних проектов.",
  "repo.removeFromRecent": "Убрать из недавних",
  "repo.removeAria": "Убрать {name} из недавних",
  "repo.cloneUrlPlaceholder": "https://github.com/owner/repo",
  "repo.openFolder": "Открыть папку",
  "repo.clone": "Клонировать",
  "repo.cloning": "Клонирование…",

  // Project metadata panel
  "meta.branch": "Ветка",
  "meta.commits": "Коммиты",
  "meta.files": "Файлы",
  "meta.size": "Размер",
  "meta.languages": "Языки",

  // Analysis / scanner view
  "scan.scanFolder": "Сканировать папку",
  "scan.scanning": "Сканирование…",
  "scan.emptyHint":
    "Просканируйте папку проекта, чтобы определить языки, фреймворки, зависимости, структуру и историю git.",
  "scan.noFiles": "Файлы не найдены.",
  "scan.branch": "Ветка",
  "scan.commits": "Коммиты",
  "scan.files": "Файлы",
  "scan.directories": "Папки",
  "scan.languages": "Языки",
  "scan.frameworks": "Фреймворки",
  "scan.noFrameworks": "Фреймворки не найдены.",
  "scan.dependencies": "Зависимости",
  "scan.noDependencies": "Зависимости не найдены.",
  "scan.structure": "Структура",
  "scan.topLevelDirs": "Папок верхнего уровня: {count}",
  "scan.git": "Git",
  "scan.topContributors": "Топ контрибьюторов",

  // AI Chat view
  "chat.changeProject": "Сменить проект",
  "chat.chooseProject": "Выбрать проект",
  "chat.clear": "Очистить",
  "chat.emptyWithProject":
    "Задайте вопрос об этом репозитории. Ответы приходят потоком, с Markdown и блоками кода.",
  "chat.emptyNoProject": "Выберите папку проекта, затем задавайте вопросы о его коде.",
  "chat.thinking": "Думаю…",
  "chat.inputPlaceholder": "Спросите об этом репозитории…",
  "chat.send": "Отправить",
  "chat.sendMessage": "Отправить сообщение",

  // Settings view
  "settings.loading": "Загрузка настроек…",
  "settings.aiProvider": "AI-провайдер",
  "settings.provider": "Провайдер",
  "settings.model": "Модель",
  "settings.modelPlaceholder": "например llama3, claude-sonnet-4, gpt-4o",
  "settings.apiKeyLabel": "API-ключ {provider}",
  "settings.apiKeyPlaceholder": "Вставьте свой API-ключ",
  "settings.apiKeyStored": "Хранится локально в папке данных приложения.",
  "settings.save": "Сохранить",
  "settings.saving": "Сохранение…",
  "settings.saved": "Сохранено",

  // Shared: project picker + analyze (used by several views)
  "common.changeProject": "Сменить проект",
  "common.chooseProject": "Выбрать проект",
  "common.analyze": "Анализировать",
  "common.analyzing": "Анализ…",
  "common.noneFound": "Ничего не найдено.",

  // Insights view
  "insights.searchPlaceholder": "Где аутентификация? Где инициализируется база данных?",
  "insights.search": "Искать",
  "insights.searchResults": "Результаты поиска",
  "insights.explain": "Объяснить",
  "insights.cyclicDependencies": "Циклические зависимости",
  "insights.deadCode": "Мёртвый код",
  "insights.duplication": "Дублирование",
  "insights.emptyHint":
    "Выберите проект и запустите анализ, чтобы найти циклические зависимости, мёртвый код и дублирование — или воспользуйтесь поиском по коду.",

  // Architecture view
  "arch.graph.dependency": "Зависимости",
  "arch.graph.module": "Модули",
  "arch.graph.folder": "Папки",
  "arch.graph.call": "Вызовы",
  "arch.legend.file": "Файл",
  "arch.legend.module": "Модуль",
  "arch.legend.directory": "Папка",
  "arch.legend.function": "Функция",
  "arch.legend.external": "Внешний",
  "arch.emptyHint":
    "Выберите проект и запустите анализ, чтобы увидеть графы зависимостей, модулей, папок и вызовов. Двигайте узлы, колесо — масштаб, перетаскивание фона — панорама.",
  "arch.graphEmpty": "Граф пуст.",
  "arch.nodesLimited": "Показаны {max} наиболее связанных из {total} узлов",

  // Status bar
  "status.noRepository": "Нет репозитория",
  "status.ready": "Готово",

  // Auto-update
  "updater.title": "Обновление",
  "updater.available": "Доступна версия {version}. Установить сейчас?",
  "updater.install": "Установить",
  "updater.later": "Позже",
};
