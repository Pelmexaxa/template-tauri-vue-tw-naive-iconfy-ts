// src/scripts/composables/useTheme.ts
import { ref, computed } from "vue";
import { darkTheme, useOsTheme } from "naive-ui";

const STORAGE_KEY = "theme_preference";
type ThemePreference = "dark" | "light" | null;

// ponytail: module-level singleton — один экземпляр на всё приложение
const osTheme = useOsTheme();
const userThemePreference = ref<ThemePreference>(loadPreference());

function loadPreference(): ThemePreference {
  const saved = localStorage.getItem(STORAGE_KEY);
  return saved === "dark" || saved === "light" ? saved : null;
}

const isDark = computed(
  () =>
    userThemePreference.value === "dark" ||
    (userThemePreference.value === null && osTheme.value === "dark"),
);

const theme = computed(() => (isDark.value ? darkTheme : null));

function toggleTheme() {
  userThemePreference.value = isDark.value ? "light" : "dark";
  localStorage.setItem(STORAGE_KEY, userThemePreference.value);
}

export function useTheme() {
  return { isDark, theme, toggleTheme };
}
