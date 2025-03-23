<template>
  <n-layout-sider
    :width="240"
    bordered
    collapse-mode="width"
    :collapsed-width="0"
    :show-trigger="false"
    :native-scrollbar="false"
    v-model:collapsed="collapsedValue"
    class="session-sider"
  >
    <n-scrollbar class="session-list-scrollbar">
      <div class="session-list">
        <!-- 顶部操作区域 - 移除切换智能体按钮，修改为单个新建会话按钮 -->
        <div class="sidebar-actions">
          <!-- 新建会话按钮 - 样式修改为占满整行 -->
          <div class="action-button full-width" @click="createNewSession">
            <n-icon size="16" class="action-icon"><AddOutline /></n-icon>
            <span>新建会话</span>
          </div>
        </div>
        
        <!-- 会话列表项 - 添加智能体图标 -->
        <div 
          v-for="session in sessions" 
          :key="session.id" 
          class="session-item"
          :class="{ active: currentSessionId === session.id }"
          @click="switchSession(session.id)"
        >
          <!-- 添加智能体图标 -->
          <div class="session-icon">
            <n-avatar
              round
              :size="24"
              :color="getAgentIconColor(session)"
            >
              <n-icon size="16">
                <component :is="getAgentIconComponent(session)" />
              </n-icon>
            </n-avatar>
          </div>
          
          <div class="session-info">
            <div class="session-preview">{{ session.topic }}</div>
          </div>
          <div class="session-actions">
            <n-button 
              circle 
              tertiary 
              size="tiny"
              @click.stop="confirmDeleteSession(session.id)"
            >
              <template #icon>
                <n-icon size="14"><TrashOutline /></n-icon>
              </template>
            </n-button>
          </div>
        </div>
        
        <div v-if="sessions.length === 0" class="no-sessions">
          暂无会话记录
        </div>
      </div>
    </n-scrollbar>
    
    <!-- 添加删除确认对话框 - 添加变换原点和上边距样式 -->
    <n-modal
      v-model:show="showDeleteConfirm"
      preset="dialog"
      title="删除会话"
      positive-text="确认"
      negative-text="取消"
      @positive-click="handleDeleteConfirm"
      @negative-click="cancelDelete"
      type="error"
      :transform-origin="'center'"
      style="margin-top: 80px;"
    >
      <template #icon>
        <n-icon color="#d03050">
          <WarningOutline />
        </n-icon>
      </template>
      确定要删除这个会话吗？此操作不可恢复。
    </n-modal>
  </n-layout-sider>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { NLayoutSider, NIcon, NButton, NScrollbar, NAvatar, NModal } from 'naive-ui';
import { AddOutline, TrashOutline, ServerOutline, WarningOutline } from '@vicons/ionicons5';
import { ChatSession } from '../../../services/typings';
import { useIconStore } from '../../../stores/iconStore';
import { useAgentStore } from '../../../stores/agentStore';

// Props
const props = defineProps<{
  sessions: ChatSession[];
  currentSessionId: number | null;
  collapsed: boolean;
}>();

// 事件
const emit = defineEmits<{
  'update:collapsed': [value: boolean];
  'create': [];
  'switch': [sessionId: number];
  'delete': [sessionId: number];
}>();

// 获取图标和智能体 store
const iconStore = useIconStore();
const agentStore = useAgentStore();

// 双向绑定折叠状态
const collapsedValue = computed({
  get: () => props.collapsed,
  set: (value) => emit('update:collapsed', value)
});

// 事件处理函数
const createNewSession = () => emit('create');
const switchSession = (sessionId: number) => emit('switch', sessionId);
const deleteSession = (sessionId: number) => emit('delete', sessionId);

// 获取智能体图标组件
const getAgentIconComponent = (session: ChatSession) => {
  
  // 获取对应智能体 - 确保每次都获取最新数据
  const agent = agentStore.agents.find(agent => agent.id === session.agentId);
  
  if (agent?.iconId) {
    const icon = iconStore.getIconById(agent.iconId);
    if (icon && icon.icon) {
      return icon.icon;
    }
  }
  // 默认图标
  return ServerOutline;
};

// 获取智能体图标颜色
const getAgentIconColor = (session: ChatSession) => {
  
  // 获取对应智能体 - 确保每次都获取最新数据
  const agent = agentStore.agents.find(agent => agent.id === session.agentId);
  
  if (agent?.iconId) {
    const icon = iconStore.getIconById(agent.iconId);
    if (icon) {
      return icon.color;
    }
  }
  // 默认颜色
  return '#18a058';
};

// 添加删除确认相关状态
const showDeleteConfirm = ref(false);
const sessionToDelete = ref<number | null>(null);

// 显示删除确认对话框
const confirmDeleteSession = (sessionId: number) => {
  sessionToDelete.value = sessionId;
  showDeleteConfirm.value = true;
};

// 确认删除
const handleDeleteConfirm = () => {
  if (sessionToDelete.value !== null) {
    deleteSession(sessionToDelete.value);
    sessionToDelete.value = null;
  }
  showDeleteConfirm.value = false;
};

// 取消删除
const cancelDelete = () => {
  sessionToDelete.value = null;
  showDeleteConfirm.value = false;
};
</script>

<style scoped>
.session-list-scrollbar {
  height: 100%;
}

.session-list {
  padding: 8px;
}

/* 顶部操作区域样式 - 修改为一行一个按钮 */
.sidebar-actions {
  display: flex;
  margin-bottom: 16px;
}

.action-button {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 10px;
  border-radius: 8px;
  background-color: rgba(0, 0, 0, 0.02);
  border: 1px dashed #ccc;
  cursor: pointer;
  transition: all 0.2s;
}

/* 添加全宽样式 */
.full-width {
  width: 100%;
}

.action-button:hover {
  background-color: rgba(0, 0, 0, 0.04);
}

.action-icon {
  margin-right: 6px;
}

/* 会话项样式 - 修改以适应图标 */
.session-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 4px 5px;
  border-radius: 6px;
  margin-bottom: 1px;
  cursor: pointer;
  transition: all 0.2s;
  position: relative;
}

/* 添加会话图标样式 */
.session-icon {
  margin-top: 5px;
  margin-right: 10px;
  flex-shrink: 0;
}

.session-item:hover {
  background-color: rgba(0, 0, 0, 0.05);
}

.session-item.active {
  background-color: rgba(36, 99, 235, 0.1);
}

.session-info {
  flex: 1;
  min-width: 0;
  padding-right: 24px; /* 为操作按钮留空间 */
}

.session-preview {
  font-size: 14px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.session-actions {
  visibility: hidden;
  position: absolute;
  right: 8px;
  top: 50%;
  transform: translateY(-50%);
}

.session-item:hover .session-actions {
  visibility: visible;
}

.no-sessions {
  padding: 20px;
  text-align: center;
  color: var(--text-color-secondary, #888888);
}

/* 侧边栏样式 */
.session-sider {
  transition: all 0.3s ease;
}

/* 添加靠上显示的样式 */
:deep(.n-modal-container) {
  margin-top: 8vh !important;
  align-items: flex-start !important;
}

:deep(.n-modal-body-wrapper) {
  align-items: flex-start !important;
}
</style>
