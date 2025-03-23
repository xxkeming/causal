import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useThemeStore = defineStore('theme', () => {
  // 主题选项: 'light', 'dark'
  const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches
  const theme = ref(localStorage.getItem('app-theme') || (isDark ? 'dark' : 'light'))
  
  // 切换主题
  function toggleTheme() {
    theme.value = theme.value === 'light' ? 'dark' : 'light'
    localStorage.setItem('app-theme', theme.value)
    applyTheme()
  }
  
  // 应用主题到文档
  function applyTheme() {
    if (theme.value === 'dark') {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
  }
  
  // 初始应用主题
  applyTheme()
  
  return { theme, toggleTheme }
})
