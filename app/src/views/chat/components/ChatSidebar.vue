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
        <!-- 搜索区域与新建会话按钮在同一行 -->
        <div class="sidebar-header">
          <n-input
            v-model:value="searchKeyword"
            placeholder="搜索会话..."
            clearable
            class="session-search"
          >
            <template #prefix>
              <n-icon><SearchOutline /></n-icon>
            </template>
          </n-input>
          <!-- 替换新建会话按钮为带tooltip的样式 -->
          <n-tooltip trigger="hover" placement="bottom">
            <template #trigger>
              <n-button
                quaternary 
                circle
                class="new-session-button"
                @click="createNewSession"
                :style="{ backgroundColor: '#18a058', color: 'white' }"
              >
                <n-icon size="18"><AddOutline /></n-icon>
              </n-button>
            </template>
            新建会话
          </n-tooltip>
        </div>
        
        <!-- 会话列表项 -->
        <div 
          v-for="session in filteredSessions" 
          :key="session.id" 
          class="session-item"
          :class="{ active: currentSessionId === session.id }"
          @click="switchSession(session.id)"
          @contextmenu="handleContextMenu($event, session)"
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
        </div>

        <!-- 添加右键菜单 -->
        <n-dropdown
          :show="showDropdown"
          :options="menuOptions"
          :x="x"
          :y="y"
          placement="bottom-start"
          @select="handleSelect"
          @clickoutside="showDropdown = false"
        />
        
        <div v-if="filteredSessions.length === 0" class="no-sessions">
          {{ sessions.length === 0 ? '暂无会话记录' : '没有找到匹配的会话' }}
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

    <!-- 修改编辑主题的模态框，参考删除确认框的样式 -->
    <n-modal 
      v-model:show="editModalVisible" 
      preset="dialog" 
      title="编辑会话主题"
      positive-text="确认"
      negative-text="取消"
      @positive-click="updateSessionName"
      @negative-click="editModalVisible = false"
      :positive-button-props="{ disabled: !editSessionName.trim(), loading: editLoading }"
      :transform-origin="'center'"
      style="margin-top: 80px;"
    >
      <template #icon>
        <n-icon color="#2080f0">
          <CreateOutline />
        </n-icon>
      </template>
      <n-input 
        v-model:value="editSessionName" 
        placeholder="请输入会话主题"
        autofocus
        style="margin-top: 12px;"
      />
    </n-modal>
  </n-layout-sider>
</template>

<script setup lang="ts">
import { computed, ref, nextTick, h } from 'vue';
import { NLayoutSider, NIcon, NButton, NScrollbar, NAvatar, NModal, NInput, NTooltip, NDropdown } from 'naive-ui';
import { AddOutline, TrashOutline, ServerOutline, WarningOutline, SearchOutline, CreateOutline, CopyOutline } from '@vicons/ionicons5';
import { ChatSession } from '../../../services/typings';
import { useIconStore } from '../../../stores/iconStore';
import { useAgentStore } from '../../../stores/agentStore';
import { useChatSessionStore } from '../../../stores/chatSessionStore';

// Props
const props = defineProps<{
  sessions: ChatSession[];
  currentSessionId: number | null;
  collapsed: boolean;
}>();

// 事件
const emit = defineEmits<{
  'update:collapsed': [value: boolean];
  'create': [agentId: number]; // 修改为必须传递agentId
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
const createNewSession = () => emit('create', 0); // 传递0表示新建会话时需要选择智能体
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

// 搜索关键词
const searchKeyword = ref('');

// 过滤后的会话列表
const filteredSessions = computed(() => {
  if (!searchKeyword.value) {
    return props.sessions;
  }
  
  const keyword = searchKeyword.value.toLowerCase();
  return props.sessions.filter(session => 
    session.topic.toLowerCase().includes(keyword)
  );
});

// 添加ChatSessionStore
const chatSessionStore = useChatSessionStore();

// 编辑会话相关状态
const editModalVisible = ref(false);
const editSessionName = ref('');
const editSessionId = ref<number | null>(null);
const editLoading = ref(false);

// 打开编辑会话模态框
function openEditSessionModal(session: ChatSession) {
  editSessionId.value = session.id;
  editSessionName.value = session.topic;
  editModalVisible.value = true;
}

// 更新会话名称
async function updateSessionName() {
  if (!editSessionId.value || !editSessionName.value.trim()) return;

  editLoading.value = true;
  try {
    const success = await chatSessionStore.updateSessionTopic(
      editSessionId.value,
      editSessionName.value.trim()
    );

    if (success) {
      editModalVisible.value = false;
    }
  } catch (error) {
    console.error('更新会话主题出错:', error);
  } finally {
    editLoading.value = false;
  }
}

// 添加右键菜单配置
const menuOptions = [
  {
    label: '新对话',
    key: 'copy',
    icon: () => h(NIcon, null, { default: () => h(CopyOutline) })
  },
  {
    label: '编辑',
    key: 'edit',
    icon: () => h(NIcon, null, { default: () => h(CreateOutline) })
  },
  {
    label: '删除',
    key: 'delete',
    icon: () => h(NIcon, null, { default: () => h(TrashOutline) })
  }
];

// 右键菜单显示位置
const x = ref(0);
const y = ref(0);
const showDropdown = ref(false);
const selectedSessionId = ref<number | null>(null);

// 处理右键菜单
const handleContextMenu = (e: MouseEvent, session: ChatSession) => {
  e.preventDefault();
  selectedSessionId.value = session.id;
  showDropdown.value = false;
  nextTick().then(() => {
    x.value = e.clientX;
    y.value = e.clientY;
    showDropdown.value = true;
  });
};

// 处理菜单选择 - 修改复制逻辑
const handleSelect = (key: string) => {
  if (!selectedSessionId.value) return;
  
  const originalSession = props.sessions.find(s => s.id === selectedSessionId.value);
  if (!originalSession) return;
  
  switch (key) {
    case 'edit':
      openEditSessionModal(originalSession);
      break;
    case 'copy':
      emit('create', originalSession.agentId); // 直接使用原会话的agentId
      break;
    case 'delete':
      confirmDeleteSession(selectedSessionId.value);
      break;
  }
  showDropdown.value = false;
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
  display: none;
}

.action-button {
  display: none;
}

.full-width {
  display: none;
}

/* 侧边栏头部样式 */
.sidebar-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 16px;
  padding-top: 3px;
}

.session-search {
  flex: 1;
}

/* 修复搜索框圆角和样式 */
:deep(.session-search .n-input) {
  border-radius: 4px !important;
}

:deep(.session-search .n-input__border),
:deep(.session-search .n-input-wrapper) {
  border-radius: 4px !important;
}

/* 修改新建会话按钮样式 */
.new-session-button {
  flex-shrink: 0;
  width: 32px;
  height: 32px;
  padding: 0;
  min-width: unset;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: opacity 0.2s;
}

.new-session-button:hover {
  opacity: 0.9;
}

.new-session-button:active {
  opacity: 0.8;
}

.new-session-button :deep(.n-icon) {
  font-size: 18px;
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
  padding-right: 8px;
}

.session-preview {
  font-size: 14px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 移除旧的操作按钮样式 */
.session-actions {
  display: none;
}

.session-action-btn {
  display: none;
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
  align-items: flex-start;
}

:deep(.n-modal-body-wrapper) {
  align-items: flex-start;
}

/* 确保操作按钮样式一致 */
.session-action-btn {
  opacity: 0.6;
  transition: opacity 0.2s;
}

.session-action-btn:hover {
  opacity: 1;
}
</style>
