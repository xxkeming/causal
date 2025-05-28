<template>
  <div class="sidebar">
    <div class="sidebar-top-icons">
      <n-tooltip placement="right" trigger="hover">
        <template #trigger>
          <div class="sidebar-icon" :class="{ active: activeIcon === 'chat' }" @click="setActive('chat')">
            <n-icon size="24"><ChatbubbleOutline /></n-icon>
          </div>
        </template>
        对话
      </n-tooltip>

      <n-tooltip placement="right" trigger="hover">
        <template #trigger>
          <div class="sidebar-icon" :class="{ active: activeIcon === 'agents' }" @click="setActive('agents')">
            <n-icon size="24"><ServerOutline /></n-icon>
          </div>
        </template>
        智能体
      </n-tooltip>
      
      <n-tooltip placement="right" trigger="hover">
        <template #trigger>
          <div class="sidebar-icon" :class="{ active: activeIcon === 'tools' }" @click="setActive('tools')">
            <n-icon size="24"><ConstructOutline /></n-icon>
          </div>
        </template>
        工具
      </n-tooltip>

      <!-- <n-tooltip placement="right" trigger="hover">
        <template #trigger>
          <div class="sidebar-icon" :class="{ active: activeIcon === 'knowledge' }" @click="setActive('knowledge')">
            <n-icon size="24"><BookOutline /></n-icon>
          </div>
        </template>
        知识库
      </n-tooltip> -->
      
    </div>
    
    <div class="sidebar-bottom-icons">
      <n-tooltip placement="right" trigger="hover">
        <template #trigger>
          <div class="sidebar-icon" @click="toggleTheme">
            <n-icon size="24">
              <SunnyOutline v-if="themeStore.theme === 'light'" />
              <MoonOutline v-else />
            </n-icon>
          </div>
        </template>
        {{ themeStore.theme === 'light' ? '切换到暗色模式' : '切换到亮色模式' }}
      </n-tooltip>
      
      <!-- 修改设置按钮为下拉菜单 -->
      <n-dropdown 
        placement="right-start" 
        trigger="hover" 
        :options="settingsOptions" 
        @select="handleSettingsSelect"
      >
        <div class="sidebar-icon" :class="{ active: isSettingsActive }">
          <n-icon size="24"><Settings /></n-icon>
        </div>
      </n-dropdown>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { NIcon, NTooltip, NDropdown } from 'naive-ui'
import { 
  ServerOutline, 
  ConstructOutline, 
  Settings,
  // BookOutline,
  SunnyOutline,
  MoonOutline,
  ChatbubbleOutline,
  InformationCircleOutline,
  SettingsOutline
} from '@vicons/ionicons5'
import { useRouter, useRoute } from 'vue-router'
import { useThemeStore } from '../stores/themeStore'
import { useGlobalStore } from '../stores/globalStore'
import { createDiscreteApi } from 'naive-ui'
import { h } from 'vue'

const router = useRouter()
const route = useRoute()
const themeStore = useThemeStore()
const globalStore = useGlobalStore()

const activeIcon = ref('messages')

// 修改设置菜单选项，使用不同的图标渲染方式
const settingsOptions = [
  {
    label: '模型配置',
    key: 'settings/model',
    icon: () => h('div', { class: 'dropdown-menu-icon' }, [
      h(NIcon, { size: 18, style: 'display:flex; align-items:center;' }, { default: () => h(ConstructOutline) })
    ])
  },
  {
    label: '系统配置',
    key: 'settings/settings',
    icon: () => h('div', { class: 'dropdown-menu-icon' }, [
      h(NIcon, { size: 18, style: 'display:flex; align-items:center;' }, { default: () => h(SettingsOutline) })
    ])
  },
  {
    label: '关于',
    key: 'settings/about',
    icon: () => h('div', { class: 'dropdown-menu-icon' }, [
      h(NIcon, { size: 18, style: 'display:flex; align-items:center;' }, { default: () => h(InformationCircleOutline) })
    ])
  }
]

// 判断设置页面是否激活
const isSettingsActive = computed(() => {
  return route.path.startsWith('/settings');
})

const setActive = (icon: string) => {
  // 如果正在测试中，阻止导航并显示提示
  if (globalStore.isLoading) {
    const { message } = createDiscreteApi(['message']);
    message.warning('请等待任务完成后再进行操作');
    return;
  }

  // 判断路由是否发生变化
  if (icon === activeIcon.value) return

  activeIcon.value = icon
  router.push(`/${icon}`)
}

// 处理设置菜单的选择
const handleSettingsSelect = (key: string) => {
  // 如果正在测试中，阻止导航并显示提示
  if (globalStore.isLoading) {
    const { message } = createDiscreteApi(['message']);
    message.warning('请等待任务完成后再进行操作');
    return;
  }
  
  router.push(`/${key}`)
}

const toggleTheme = () => {
  themeStore.toggleTheme()
}

// 同步路由和激活的图标
watch(() => route.path, (path) => {
  const pathSegment = path.split('/')[1] || 'messages'
  if (['chat', 'messages', 'agents', 'knowledge', 'tools', 'settings'].includes(pathSegment)) {
    activeIcon.value = pathSegment
  }
}, { immediate: true })

</script>

<style scoped>
.sidebar {
  height: 100%;
  width: 48px;
  display: flex;
  flex-direction: column;
  background-color: var(--sidebar-bg);
}

.sidebar-top-icons {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex-grow: 1;
}

.sidebar-bottom-icons {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding-bottom: 10px;
}

.sidebar-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  color: var(--text-color-secondary);
}

.sidebar-icon:hover {
  color: var(--hover-color);
}

.sidebar-icon.active {
  color: var(--active-color);
  border-left: 2px solid var(--active-color);
}

/* 完全重写下拉菜单图标的样式 */
.dropdown-menu-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: 8px;
  line-height: 1;
}

:deep(.n-dropdown-option-body__prefix) {
  display: flex !important;
  align-items: center !important;
  height: 100% !important;
}

:deep(.n-dropdown-option) {
  padding: 8px 12px;
}

:deep(.n-dropdown-option-body) {
  display: flex !important;
  align-items: center !important;
  height: 24px !important;
}

:deep(.n-dropdown-option-body__label) {
  display: flex !important;
  align-items: center !important;
  height: 100% !important;
  line-height: 1 !important;
}

/* 强制图标居中 */
:deep(.n-icon) {
  display: flex !important;
  align-items: center !important;
  justify-content: center !important;
}

/* 改进下拉菜单图标样式 */
.dropdown-menu-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: 8px;
  height: 22px; /* 添加固定高度 */
}

:deep(.n-dropdown-option-body__prefix) {
  width: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%; /* 确保高度填满父容器 */
}

/* 调整下拉菜单项的样式 */
:deep(.n-dropdown-option) {
  padding: 8px 12px;
  height: 36px; /* 添加固定高度 */
}

:deep(.n-dropdown-option-body) {
  display: flex;
  align-items: center;
  height: 100%; /* 确保高度填满父容器 */
}

:deep(.n-dropdown-option-body__label) {
  font-size: 14px;
  line-height: 1.4; /* 添加行高保证文字垂直居中 */
  display: flex;
  align-items: center;
}
</style>
