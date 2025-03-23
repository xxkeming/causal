<template>
  <n-config-provider :theme="currentNaiveTheme">
    <n-loading-bar-provider>
      <n-dialog-provider>
        <n-notification-provider>
          <n-message-provider>
            <router-view />
          </n-message-provider>
        </n-notification-provider>
      </n-dialog-provider>
    </n-loading-bar-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { 
  NConfigProvider, NLoadingBarProvider, NDialogProvider, 
  NNotificationProvider, NMessageProvider, lightTheme, darkTheme 
} from 'naive-ui'
import { useThemeStore } from './stores/themeStore'

const themeStore = useThemeStore()
const currentNaiveTheme = computed(() => 
  themeStore.theme === 'dark' ? darkTheme : lightTheme
)
</script>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
html, body {
  margin: 0;
  padding: 0;
  height: 100%;
  width: 100%;
}

#app {
  height: 100%;
  width: 100%;
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  color: #f6f6f6;
  background-color: #2f2f2f;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}
</style>