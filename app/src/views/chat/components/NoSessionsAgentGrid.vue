<template>
  <div class="no-sessions-view">
    <div class="welcome-header">
      <n-icon size="48" color="#336aea"><ChatbubblesOutline /></n-icon>
      <h1>开始一段新对话</h1>
      <p>选择一个智能体开始对话，或者创建您自己的智能体</p>
    </div>
    
    <div class="agents-grid">
      <div 
        v-for="agent in agents" 
        :key="agent.id" 
        class="agent-card"
        @click="selectAgent(agent)"
      >
        <div class="agent-card-avatar">
          <n-avatar
            round
            :size="48"
            :color="getAgentColor(agent)"
          >
            <n-icon size="28">
              <component :is="getAgentIcon(agent)" />
            </n-icon>
          </n-avatar>
        </div>
        <div class="agent-card-content">
          <div class="agent-card-name">{{ agent.name }}</div>
          <div class="agent-card-description" v-if="agent.description">
            {{ truncateText(agent.description, 60) }}
          </div>
        </div>
      </div>
      
      <!-- 创建新智能体的卡片 -->
      <div class="agent-card create-agent-card" @click="$emit('create-agent')">
        <div class="agent-card-avatar">
          <n-avatar
            round
            :size="48"
            color="#18a058"
          >
            <n-icon size="28">
              <AddOutline />
            </n-icon>
          </n-avatar>
        </div>
        <div class="agent-card-content">
          <div class="agent-card-name">创建新智能体</div>
          <div class="agent-card-description">
            创建您自己的定制智能体
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { NAvatar, NIcon } from 'naive-ui';
import { ChatbubblesOutline, AddOutline } from '@vicons/ionicons5';
import { ServerOutline } from '@vicons/ionicons5';
import { Agent } from '../../../services/typings';
import { useIconStore } from '../../../stores/iconStore';

// 定义props
defineProps<{
  agents: Agent[];
}>();

// 定义事件
const emit = defineEmits<{
  'select-agent': [agent: Agent];
  'create-agent': [];
}>();

// 获取图标 store
const iconStore = useIconStore();

// 文本截断函数
function truncateText(text: string, maxLength: number) {
  if (!text) return '';
  if (text.length <= maxLength) return text;
  return text.slice(0, maxLength) + '...';
}

// 获取智能体图标
function getAgentIcon(agent: Agent) {
  if (agent.iconId) {
    const icon = iconStore.getIconById(agent.iconId);
    return icon?.icon || ServerOutline;
  }
  return ServerOutline;
}

// 获取智能体颜色
function getAgentColor(agent: Agent) {
  if (agent.iconId) {
    const icon = iconStore.getIconById(agent.iconId);
    return icon?.color || '#18a058';
  }
  return '#18a058';
}

// 选择智能体 - 直接触发选择事件，无需二次确认
function selectAgent(agent: Agent) {
  emit('select-agent', agent);
}
</script>

<style scoped>
/* 无会话时的智能体选择界面样式 */
.no-sessions-view {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 40px 20px;
  height: 100%;
  overflow-y: auto;
}

.welcome-header {
  text-align: center;
  margin-bottom: 40px;
}

.welcome-header h1 {
  font-size: 28px;
  font-weight: 600;
  margin: 20px 0 10px;
  color: var(--text-color);
}

.welcome-header p {
  font-size: 16px;
  color: var(--text-color-secondary);
  margin-bottom: 0;
}

.agents-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); /* 减小最小宽度 */
  gap: 16px; /* 减少间距 */
  width: 100%;
  max-width: 1200px;
}

.agent-card {
  display: flex;
  padding: 12px; /* 减少内边距 */
  border-radius: 8px; /* 减小圆角 */
  border: 1px solid var(--border-color);
  cursor: pointer;
  transition: all 0.3s ease;
  background-color: var(--card-color);
  min-height: unset; /* 移除最小高度限制 */
  align-items: center; /* 垂直居中内容 */
}

.agent-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.06);
  background-color: rgba(24, 160, 88, 0.02);
}

/* 添加点击效果 */
.agent-card:active {
  transform: translateY(0);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.04);
}

.create-agent-card {
  border-style: dashed;
}

.agent-card-avatar {
  margin-right: 12px; /* 减少间距 */
  flex-shrink: 0;
}

.agent-card-content {
  flex: 1;
  min-width: 0;
  display: flex; 
  flex-direction: column;
}

.agent-card-name {
  font-weight: 500;
  font-size: 14px; /* 减小字体 */
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* 隐藏描述 */
.agent-card-description {
  display: none;
}
</style>
