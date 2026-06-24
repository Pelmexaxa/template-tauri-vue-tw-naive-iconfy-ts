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
