<template>
  <n-config-provider :theme="theme" :locale="locale" :date-locale="dateLocale">
    <n-global-style />
    <n-loading-bar-provider>
      <n-modal-provider>
        <n-message-provider>
          <n-dialog-provider>
            <slot />
          </n-dialog-provider>
        </n-message-provider>
      </n-modal-provider>
    </n-loading-bar-provider>
  </n-config-provider>
</template>

<script lang="ts" setup>
import { ruRU, dateRuRU } from "naive-ui";
import { useTheme } from "@/scripts/stores/theme.ts";

const { theme } = useTheme();

const dateLocale = dateRuRU;
const locale = ruRU;
</script>
