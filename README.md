# template-tauri-vue-tw-naive-iconfy-ts

Шаблон десктоп-приложения. Стек технологий:

## Frontend

- **Vue 3** — UI, Composition API, `<script setup>`
- **TypeScript** — типизация
- **Vite** — dev-сервер и сборка

## UI и стили

- **Tailwind CSS 4** — utility-классы (`@tailwindcss/vite`)
- **Naive UI** — компоненты Vue 3
- **Iconify** — иконки через Tailwind (`@iconify/tailwind4`, `@iconify/json`)

## Desktop

- **Tauri 2** — нативная оболочка приложения
- **Rust** — backend-команды
- **Serde / serde_json** — сериализация данных
- **tauri-plugin-opener** — открытие ссылок в системном браузере

## Инструменты

- **vue-tsc** — проверка типов Vue
- **@vue/tsconfig** — базовый TS-конфиг для Vue
- **@tsconfig/node22** — TS-конфиг для Node (Vite)
- **@vitejs/plugin-vue** — поддержка `.vue` в Vite
- **@tauri-apps/cli** — CLI Tauri

---

## Требования

- **Node.js** 20+
- **npm** (или другой менеджер пакетов)
- **Rust** — [rustup](https://rustup.rs/)
- **Зависимости Tauri 2** для вашей ОС — [Prerequisites](https://v2.tauri.app/start/prerequisites/)

На Windows дополнительно нужны MSVC Build Tools и WebView2 (обычно уже установлен в Windows 10/11).

## Запуск

```bash
# Установка зависимостей
npm install

# Только фронтенд (браузер, порт 1420)
npm run dev

# Десктоп-приложение с hot reload
npm run tauri dev

# Проверка типов + production-сборка фронтенда
npm run build

# Сборка установщика / бинарника
npm run tauri build

# Превью собранного фронтенда
npm run preview
```

### npm-скрипты

| Команда | Описание |
|---|---|
| `npm run dev` | Vite dev-сервер на `http://localhost:1420` |
| `npm run tauri dev` | Tauri + Vite, открывает окно приложения |
| `npm run build` | `vue-tsc` + `vite build` → папка `dist/` |
| `npm run tauri build` | Сборка фронтенда и нативного приложения |
| `npm run preview` | Локальный просмотр `dist/` |
| `npm run tauri` | Прокси к CLI Tauri (`tauri --help`) |

## Структура проекта

```
.
├── index.html              # Точка входа Vite
├── vite.config.ts          # Vite: Vue, Tailwind, алиас @ → src/
├── tsconfig*.json          # TypeScript (app + node)
├── package.json
│
├── src/                    # Фронтенд (Vue)
│   ├── main.ts             # createApp, подключение Naive UI
│   ├── App.vue             # Корневой компонент
│   ├── BaseProvider.vue    # n-config-provider, провайдеры Naive UI, тема
│   ├── naive.ts            # Глобальная регистрация компонентов Naive UI
│   ├── vite-env.d.ts       # Типы Vite / .vue
│   ├── assets/
│   │   └── styles.css      # Tailwind + глобальные стили
│   ├── scripts/
│   │   └── stores/
│   │       └── theme.ts    # Тема: localStorage + системная (useTheme)
│   └── views/
│       └── pages/
│           └── MainPage.vue  # Главная страница, демо Tauri-команд
│
├── src-tauri/              # Backend (Rust / Tauri)
│   ├── tauri.conf.json     # Конфиг приложения, сборка, окно
│   ├── Cargo.toml          # Rust-зависимости
│   ├── build.rs
│   ├── capabilities/
│   │   └── default.json    # Разрешения Tauri 2
│   └── src/
│       ├── main.rs         # Точка входа бинарника
│       └── lib.rs          # Tauri-команды (greet, calc) и run()
│
└── .vscode/
    └── extensions.json     # Рекомендуемые расширения VS Code
```

### Как связаны части

1. **`src/main.ts`** монтирует `App.vue` и подключает Naive UI через `naive.ts`.
2. **`App.vue`** оборачивает страницы в `BaseProvider` (тема, локаль, провайдеры).
3. **Страницы** лежат в `src/views/pages/` и вызывают Rust через `@tauri-apps/api` (`invoke`).
4. **Rust-команды** объявляются в `src-tauri/src/lib.rs` с макросом `#[tauri::command]`.
5. **Tauri** при `dev`/`build` сам запускает `npm run dev` / `npm run build` (см. `tauri.conf.json`).

## VS Code

Рекомендуемые расширения (`.vscode/extensions.json`):

- **Vue - Official** (Volar) — подсказки в `.vue`
- **Tauri** — интеграция с Tauri
- **rust-analyzer** — Rust в `src-tauri/`

Для автодополнения Naive UI в шаблонах в `tsconfig.app.json` подключены типы `naive-ui/volar`.
