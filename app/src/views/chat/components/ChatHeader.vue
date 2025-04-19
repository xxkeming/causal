<template>
  <div class="chat-header">
    <n-space align="center">
      <!-- 侧边栏切换按钮 - 添加点击事件 -->
      <n-tooltip trigger="hover" placement="bottom">
        <template #trigger>
          <n-button quaternary circle class="sidebar-button" 
            :style="{ backgroundColor: '#64748b', color: 'white' }"
            @click="toggleSidebar"
          >
            <n-icon size="20">
              <MenuOutline />
            </n-icon>
          </n-button>
        </template>
        会话列表
      </n-tooltip>
      
      <!-- 智能体头像 - 修改颜色与智能体一致 -->
      <n-tooltip trigger="hover" placement="bottom">
        <template #trigger>
          <n-button quaternary circle class="agent-button" 
            :style="{ backgroundColor: agentIcon?.color || '#18a058', color: 'white' }"
             @click="onShowAgentConfig"
          >
            <n-icon size="20">
              <component :is="agentIcon?.icon || ServerOutline" />
            </n-icon>
          </n-button>
        </template>
        {{ agent?.name || '未命名智能体' }}
      </n-tooltip>
      
      <!-- 模型选择器 - 添加不可清空属性 -->
      <div class="controls-wrapper">
        <model-selector 
          v-model="modelValue"
          :default-tags="['推理']"
          :show-tag-filter="false"
          @change="onModelChange"
          size="small"
        />
      </div>
      
      <!-- 配置按钮 -->
      <n-tooltip trigger="hover">
        <template #trigger>
          <n-button class="action-button" circle tertiary size="small" @click="onProviderConfig">
            <n-icon><SettingsOutline /></n-icon>
          </n-button>
        </template>
        模型配置
      </n-tooltip>
    </n-space>
    
    <!-- 右侧按钮组 - 修复图标 -->
    <n-space>
      <n-tooltip trigger="hover">
        <template #trigger>
          <n-button class="action-button" circle tertiary size="small" @click="onClearSession">
            <n-icon><TrashBinOutline /></n-icon>
          </n-button>
        </template>
        清除对话
      </n-tooltip>
      <n-tooltip trigger="hover">
        <template #trigger>
          <n-button class="action-button" circle tertiary size="small" @click="onCloseSession">
            <n-icon><CloseOutline /></n-icon>
          </n-button>
        </template>
        关闭对话
      </n-tooltip>
    </n-space>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { 
  NIcon, NButton, NTooltip, NSpace
} from 'naive-ui';
import { 
  ServerOutline, TrashBinOutline, CloseOutline,
  MenuOutline, SettingsOutline
} from '@vicons/ionicons5';
import { Agent } from '../../../services/typings';
import ModelSelector from '../../../components/ModelSelector.vue';

// 接收props
const props = defineProps<{
  agent: Agent | null;
  agentIcon: { icon: any; color: string } | null;
  selectedModelValue: string | undefined;
}>();

// 定义事件 (移除 switch-agent 事件)
const emit = defineEmits<{
  'toggle-sidebar': [];
  'show-agent-config': [];
  'show-provider-config': [];
  'clear-session': [];
  'close-session': [];
  'update:selectedModelValue': [value: string | undefined];
  'model-change': [model: any];
}>();

// 双向绑定的模型值
const modelValue = computed({
  get: () => props.selectedModelValue,
  set: (value) => emit('update:selectedModelValue', value)
});

// 事件处理函数 (移除 onSwitchAgent 函数)
const toggleSidebar = () => emit('toggle-sidebar');
const onShowAgentConfig = () => emit('show-agent-config');
const onProviderConfig = () => emit('show-provider-config');
const onClearSession = () => emit('clear-session');
const onCloseSession = () => emit('close-session');
const onModelChange = (model: any) => emit('model-change', model);
</script>

<style scoped>
.chat-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 14px;
  border-bottom: 1px solid var(--border-color, #eaeaea);
  background-color: var(--card-color, #ffffff);
}

/* 控件容器样式 */
.controls-wrapper {
  display: flex;
  align-items: center;
  margin-left: 8px;
}

/* 确保模型选择器得到适当的间距 */
.chat-header .n-space {
  gap: 8px !important;
}

/* 删除旧的汉堡菜单样式 */
.menu-button-container,
.menu-button,
.menu-button span {
  display: none;
}

/* 更新侧边栏按钮样式以匹配智能体图标 */
.sidebar-button {
  padding: 0;
  height: 32px;
  width: 32px;
  color: white;
  transition: opacity 0.2s;
}

.sidebar-button:hover {
  opacity: 0.9;  /* 添加悬停效果 */
}

.sidebar-button:active {
  opacity: 0.8;  /* 添加点击效果 */
}

.sidebar-button :deep(.n-icon) {
  font-size: 18px;
}

/* 添加按钮图标样式 */
.action-button {
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-button :deep(.n-icon) {
  font-size: 16px;
}

/* 修复头像图标对齐 */
.n-avatar :deep(.n-icon) {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: -1px; /* 微调图标垂直位置 */
}

/* 删除旧的头像相关样式 */
.avatar-container,
.agent-avatar-button,
.agent-avatar {
  display: none;
}

/* 修改按钮样式，移除固定的背景色 */
.agent-button {
  padding: 0;
  height: 32px;
  width: 32px;
  /* 移除固定背景色，改为内联样式绑定 */
  color: white;
  transition: opacity 0.2s;
}

.agent-button:hover {
  opacity: 0.9;  /* 添加悬停效果 */
}

.agent-button:active {
  opacity: 0.8;  /* 添加点击效果 */
}

/* 修复图标在按钮中的样式 */
.agent-button :deep(.n-icon) {
  font-size: 20px;
}

/* 删除对头像组件的引用，因为我们不再使用它 */
.agent-button :deep(.n-avatar),
.agent-button :deep(.n-avatar__icon) {
  display: none;
}

/* 确保头像在按钮中居中 */
.agent-button :deep(.n-avatar) {
  margin: 0;
  line-height: 1;
  display: flex;
  justify-content: center;
  align-items: center;
}

/* 确保图标在头像中居中 */
.agent-button :deep(.n-avatar__icon) {
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 16px;
}
</style>
