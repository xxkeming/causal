<template>
  <div class="chat-view">
    <n-layout has-sider position="absolute">
      <!-- 左侧智能体助手列表 -->
      <n-layout-sider
        :width="220"
        bordered
        show-trigger="arrow-circle"
        collapse-mode="width"
        :collapsed-width="0"
        position="absolute"
        class="chat-sidebar"
        :native-scrollbar="false"
      >
        <div class="chat-header">
          <n-h3>智能体助手</n-h3>
          <n-button circle size="small">
            <template #icon>
              <n-icon><AddOutline /></n-icon>
            </template>
          </n-button>
        </div>
        
        <div class="chat-agent-container">
          <n-scrollbar>
            <n-list class="chat-agent-list" :show-divider="false">
              <n-list-item v-for="(agent, index) in agents" :key="index" 
                class="chat-agent-item" 
                :class="{ active: index === activeAgentIndex }" 
                @click="selectAgent(index)"
              >
                <div class="chat-agent-content">
                  <n-avatar round class="chat-agent-avatar" :size="24" :color="getAgentIcon(agent)?.color || '#d9d9d9'">
                    <n-icon size="14" v-if="getAgentIcon(agent)">
                      <component :is="getAgentIcon(agent)?.icon" />
                    </n-icon>
                    <n-icon size="14" v-else><ServerOutline /></n-icon>
                  </n-avatar>
                  <span class="chat-agent-name">{{ agent.name }}</span>
                </div>
              </n-list-item>
            </n-list>
          </n-scrollbar>
        </div>
      </n-layout-sider>
      
      <!-- 右侧内容区域 -->
      <n-layout position="absolute" style="left: 220px; right: 0;" class="chat-content">
        <n-empty description="选择一个智能体助手或创建新的对话">
          <template #extra>
            <n-button type="primary">开始对话</n-button>
          </template>
        </n-empty>
      </n-layout>
    </n-layout>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { 
  NLayout, NLayoutSider, NList, NListItem, 
  NAvatar, NIcon, NH3, NScrollbar,
  NEmpty, NButton 
} from 'naive-ui'
import { ServerOutline, AddOutline } from '@vicons/ionicons5'
import { useIconStore } from '../../stores/iconStore'

// 选中的智能体索引
const activeAgentIndex = ref(-1)

// 选择智能体
const selectAgent = (index: number) => {
  activeAgentIndex.value = index
}

// 模拟智能体数据
const agents = ref([
  {
    name: '通用助手',
    avatar: '',
    color: '#2080f0'
  },
  {
    name: '代码助手',
    avatar: '',
    color: '#18a058'
  },
  {
    name: '创意助手',
    avatar: '',
    color: '#f0a020'
  },
  {
    name: '数据分析师',
    avatar: '',
    color: '#d03050'
  },
  {
    name: '个人助理',
    avatar: '',
    color: '#8a2be2'
  }
])

const iconStore = useIconStore()

// 获取智能体图标
function getAgentIcon(agent: any) {
  if (agent.iconId) {
    return iconStore.getIconById(agent.iconId)
  }
  return null
}
</script>

<style scoped>
.chat-view {
  height: 100%;
  width: 100%;
  overflow: hidden;
  position: relative;
}

.chat-sidebar {
  top: 0;
  bottom: 0;
  left: 0;
  overflow: visible; /* 修改为可见，确保折叠按钮显示 */
}

.chat-header {
  padding: 12px 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--border-color);
}

.chat-agent-container {
  position: absolute;
  top: 50px;
  bottom: 0;
  left: 0;
  right: 0;
  overflow: hidden;
  padding: 8px 0;
}

.chat-agent-list {
  height: 100%;
  padding: 0 4px;
}

.chat-agent-item {
  cursor: pointer;
  padding: 10px 16px;
  transition: all 0.3s;
  border-radius: 6px;
  margin: 4px 8px;
}

.chat-agent-content {
  display: flex;
  align-items: center;
  gap: 12px;
}

.chat-agent-avatar {
  flex-shrink: 0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.chat-agent-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-color);
}

.chat-agent-item:hover {
  background-color: rgba(128, 128, 128, 0.08);
  transform: translateY(-1px);
}

.chat-agent-item.active {
  background-color: rgba(24, 160, 88, 0.12);
  box-shadow: 0 2px 8px rgba(24, 160, 88, 0.15);
}

.chat-content {
  top: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

:deep(.n-layout-sider-scroll-container) {
  overflow: hidden !important;
  z-index: 1;
}

:deep(.n-scrollbar-container) {
  height: 100%;
}

:deep(.n-scrollbar-content) {
  min-height: 100%;
}

:deep(.n-list) {
  --n-padding: 0;
  --n-divider-color: transparent;
}

:deep(.n-list-item) {
  padding: 3px 0;
  border-bottom: none !important;
}

:deep(.n-h3) {
  margin: 0;
  font-size: 16px;
}

/* 添加折叠触发器样式 */
:deep(.n-layout-toggle-button) {
  z-index: 100;
  width: 24px;
  right: -12px !important;
  top: 50%;
  transform: translateY(-50%);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  background-color: var(--primary-color);
  color: white;
}

:deep(.n-layout-toggle-button:hover) {
  background-color: var(--hover-color);
}

:deep(.n-layout-toggle-bar) {
  right: 0;
  height: 100%;
  width: 3px;
}

/* 自定义箭头折叠触发器样式 */
:deep(.n-layout-toggle-button) {
  z-index: 100;
  width: 24px;
  height: 24px;
  right: -12px !important;
  top: 50%;
  transform: translateY(-50%);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
  background-color: #ffffff;
  color: var(--primary-color);
  border: 1px solid var(--border-color);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s;
}

:deep(.n-layout-toggle-button:hover) {
  color: #ffffff;
  background-color: var(--primary-color);
}

:deep(.n-layout-toggle-bar) {
  display: none; /* 隐藏条形触发器 */
}
</style>
