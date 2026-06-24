<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { TagProps } from "naive-ui";

const greetMsg = ref("");
const name = ref("");

type TechItem = {
  name: string;
  desc: string;
  icon: string;
  tag: TagProps["type"];
};

type TechGroup = {
  category: string;
  items: TechItem[];
};

const techStack: TechGroup[] = [
  {
    category: "Frontend",
    items: [
      { name: "Vue 3", desc: "Composition API, script setup", icon: "icon-[logos--vue]", tag: "success" },
      { name: "TypeScript", desc: "Строгая типизация", icon: "icon-[logos--typescript-icon]", tag: "info" },
      { name: "Vite", desc: "Сборка и HMR", icon: "icon-[logos--vitejs]", tag: "warning" },
    ],
  },
  {
    category: "UI & стили",
    items: [
      { name: "Tailwind CSS 4", desc: "Utility-first CSS", icon: "icon-[logos--tailwindcss-icon]", tag: "success" },
      { name: "Naive UI", desc: "Vue 3 компоненты", icon: "icon-[logos--naiveui]", tag: "info" },
      { name: "Iconify", desc: "Иконки через Tailwind", icon: "icon-[simple-icons--iconify]", tag: "default" },
    ],
  },
  {
    category: "Desktop",
    items: [
      { name: "Tauri 2", desc: "Нативное приложение", icon: "icon-[logos--tauri]", tag: "success" },
      { name: "Rust", desc: "Backend-команды", icon: "icon-[logos--rust]", tag: "warning" },
      { name: "Serde", desc: "Сериализация данных", icon: "icon-[mdi--code-json]", tag: "info" },
    ],
  },
];

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <n-config-provider>
    <n-layout class="min-h-screen bg-neutral-950">
      <n-layout-header bordered class="px-6 py-4 backdrop-blur">
        <n-flex align="center" justify="space-between" wrap>
          <n-flex align="center" :size="12">
            <span class="icon-[logos--tauri] size-8" />
            <div>
              <n-h2 class="m-0!">Tauri + Vue Template</n-h2>
              <n-text depth="3">Vue · Tailwind · Naive UI · Iconify · TypeScript</n-text>
            </div>
          </n-flex>
          <n-tag type="success" round size="small">v0.1.0</n-tag>
        </n-flex>
      </n-layout-header>

      <n-layout-content class="p-6">
        <n-grid cols="1 m:2 l:3" :x-gap="16" :y-gap="16" responsive="screen">
          <n-gi v-for="group in techStack" :key="group.category">
            <n-card :title="group.category" size="small" hoverable>
              <n-list>
                <n-list-item v-for="tech in group.items" :key="tech.name">
                  <n-thing>
                    <template #avatar>
                      <n-avatar round>
                        <span :class="[tech.icon, 'size-5']" />
                      </n-avatar>
                    </template>
                    <template #header>
                      <n-flex align="center" :size="8">
                        <span>{{ tech.name }}</span>
                        <n-tag :type="tech.tag" size="tiny" round>{{ group.category }}</n-tag>
                      </n-flex>
                    </template>
                    <template #description>{{ tech.desc }}</template>
                  </n-thing>
                </n-list-item>
              </n-list>
            </n-card>
          </n-gi>
        </n-grid>

        <n-divider>Демо Tauri</n-divider>

        <n-card title="Rust ↔ Vue" size="small" class="max-w-xl mx-auto">
          <n-space vertical :size="12">
            <n-input
              v-model:value="name"
              placeholder="Введите имя..."
              clearable
              @keyup.enter="greet"
            />
            <n-button type="primary" block @click="greet">Поздороваться</n-button>
            <n-text v-if="greetMsg" type="success">{{ greetMsg }}</n-text>
          </n-space>
        </n-card>
      </n-layout-content>
    </n-layout>
  </n-config-provider>
</template>
