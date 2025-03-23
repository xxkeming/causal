<template>
  <div class="chat-container">
    <n-layout has-sider>
      <!-- 侧边栏组件 - 添加currentSessionId属性 -->
      <chat-sidebar
        :key="`chat-sidebar-${forceUpdate}`"
        :sessions="chatSessionStore.sessions"
        :current-session-id="currentSessionId"
        v-model:collapsed="siderCollapsed"
        @create="openAgentSelectorForNewSession"
        @switch="switchSession"
        @delete="deleteSession"
      />

      <!-- 右侧聊天内容 -->
      <n-layout>
        <!-- 当有会话时显示聊天界面 -->
        <template v-if="hasActiveSession">
          <!-- 聊天头部组件 -->
          <chat-header
            :agent="agent"
            :agent-icon="agentIcon"
            v-model:selected-model-value="selectedModelValue"
            @toggle-sidebar="toggleSidebar"
            @show-config="showAgentConfig"
            @clear-session="clearSession"
            @close-session="closeSession"
            @model-change="handleModelChange"
          />

          <!-- 聊天消息列表组件 -->
          <chat-message-list
            :messages="messages"
            :agent-name="agent?.name"
            :agent-description="agent?.description"
            :agent-icon="agentIcon"
            :loading="loading"
            :suggested-prompts="suggestedPrompts"
            @retry="retryMessage"
            @send="sendMessage"
          />

          <!-- 输入区域 -->
          <div class="chat-input-floating-container">
            <div class="chat-input-wrapper">
              <n-input
                v-model:value="inputMessage"
                type="textarea"
                placeholder="输入消息，按Enter发送，Shift+Enter换行..."
                :autosize="{ minRows: 1, maxRows: 6 }"
                class="chat-input"
                @keydown="handleKeyDown"
              />
              <n-button 
                type="primary" 
                class="send-button" 
                :disabled="!canSendMessage"
                @click="sendMessage()"
              >
                <template #icon>
                  <n-icon><SendOutline /></n-icon>
                </template>
              </n-button>
            </div>
          </div>
        </template>

        <!-- 当没有会话时显示智能体选择界面组件 -->
        <no-sessions-agent-grid 
          v-else
          :agents="agentStore.agents"
          @select-agent="createSessionWithAgent"
          @create-agent="navigateToCreateAgent"
        />
      </n-layout>
    </n-layout>

    <!-- 智能体选择器组件 -->
    <agent-selector
      v-model:visible="agentSelectorVisible"
      @select="handleAgentSwitch"
    />

    <!-- 智能体配置弹窗 -->
    <agent-form-modal 
      v-model:visible="agentConfigVisible"
      :is-edit="true"
      :agent-data="agent"
      @submit="handleAgentConfigSubmit"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, reactive, nextTick } from 'vue';
import { useRouter } from 'vue-router';
import { 
  NLayout, NIcon, NButton, NInput, useMessage
} from 'naive-ui';
import { SendOutline } from '@vicons/ionicons5';
import { Agent, ChatMessage, ChatSession } from '../../services/typings';
import { useAgentStore } from '../../stores/agentStore';
import { useIconStore } from '../../stores/iconStore';
import { useChatSessionStore } from '../../stores/chatSessionStore';
import { simulateChatStream } from '../../services/api';

// 导入组件
import AgentFormModal from '../agents/components/AgentFormModal.vue';
import ChatHeader from './components/ChatHeader.vue'; 
import ChatSidebar from './components/ChatSidebar.vue';
import AgentSelector from '../../components/AgentSelector.vue';
import ChatMessageList from './components/ChatMessageList.vue';
import NoSessionsAgentGrid from './components/NoSessionsAgentGrid.vue';

// 强制刷新
const forceUpdate = ref(0);

// 配置弹窗控制
const agentConfigVisible = ref(false);
const agentSelectorVisible = ref(false);

// 模型选择器状态
const selectedModelValue = ref<string | undefined>(undefined);

// 路由和消息
const router = useRouter();
const message = useMessage();

// Store
const agentStore = useAgentStore();
const iconStore = useIconStore();
const chatSessionStore = useChatSessionStore();

// 本地状态管理会话和消息
const currentSessionId = ref<number | null>(null);
const currentSession = ref<ChatSession | null>(null);
const messages = ref<ChatMessage[]>([]);

// 状态
const agent = ref<Agent | null>(null);
const loading = ref(false);
const inputMessage = ref('');
const siderCollapsed = ref(false);

// 计算属性
const hasActiveSession = computed(() => 
  currentSessionId.value !== null && currentSession.value !== null
);

const agentIcon = computed(() => {
  if (agent.value?.iconId) {
    const icon = iconStore.getIconById(agent.value.iconId);
    return icon ? { icon: icon.icon, color: icon.color } : null;
  }
  return null;
});

const canSendMessage = computed(() => inputMessage.value.trim() !== '' && !loading.value);

// 建议问题
const suggestedPrompts = ref([
  '你能做什么?',
  '请介绍一下你自己',
  '帮我写一个简单的Python程序'
]);

// 加载当前会话的消息
async function loadSessionMessages(sessionId: number) {
  try {
    // 实际项目中应该从API获取消息
    // 这里我们使用本地存储模拟
    const key = `chat-messages-${sessionId}`;
    const storedMessages = localStorage.getItem(key);
    
    if (storedMessages) {
      messages.value = JSON.parse(storedMessages);
    } else {
      messages.value = [];
    }
  } catch (error) {
    console.error('加载会话消息失败:', error);
    messages.value = [];
  }
}

// 保存当前会话的消息
function saveSessionMessages() {
  if (!currentSessionId.value) return;
  
  try {
    const key = `chat-messages-${currentSessionId.value}`;
    localStorage.setItem(key, JSON.stringify(messages.value));
  } catch (error) {
    console.error('保存会话消息失败:', error);
  }
}

// 加载智能体信息
async function loadAgentInfo() {
  try {
    if (!currentSession.value) {
      return;
    }
    
    // 从当前会话获取智能体ID
    const agentId = currentSession.value.agentId;
    
    agent.value = await agentStore.fetchAgentById(agentId);
    
    if (!agent.value) {
      message.error('无法加载智能体信息');
      return;
    }

    // 更新模型选择器值
    if (agent.value.model) {
      selectedModelValue.value = agent.value.model.id + ':' + agent.value.model.name;
    }
    
  } catch (error) {
    console.error('加载智能体信息失败:', error);
    message.error('加载智能体信息失败');
  }
}

// 发送API消息
async function sendApiMessage(text: string) {
  if (!agent.value || !text.trim() || !currentSessionId.value) return;

  try {
    loading.value = true;

    // 添加用户消息
    const userMessage = {
      role: 'user' as const,
      content: text,
      timestamp: new Date(),
      sessionId: currentSessionId.value
    };
    addMessage(userMessage);

    // 添加助手消息
    const assistantMessage = reactive({
      role: 'assistant' as const,
      content: '',
      status: 'sending' as const,
      timestamp: new Date(),
      sessionId: currentSessionId.value
    });
    addMessage(assistantMessage);

    // 模拟流式输出
    await simulateChatStream(text, async (chunk) => {
      // 更新助手消息的状态和内容
      if (assistantMessage.status !== 'sending') {
        assistantMessage.status = 'streaming';
      }
      assistantMessage.content += chunk; // 实时更新内容
      updateLastMessage({ content: assistantMessage.content }); // 确保更新到最后一条消息
      await nextTick(); // 确保视图更新
    });

    // 更新助手消息的状态
    assistantMessage.status = 'success';
    updateLastMessage({ status: assistantMessage.status });

  } catch (error) {
    console.error('API 请求失败:', error);

    // 更新助手消息为错误状态
    updateLastMessage({
      status: 'error',
      content: '获取回复失败，请重试'
    });

  } finally {
    loading.value = false;
  }
}

// 添加消息
function addMessage(message: ChatMessage) {
  messages.value.push(message);
  saveSessionMessages();

  // 如果是有会话的情况，更新会话的更新时间
  if (currentSessionId.value) {
    chatSessionStore.updateSession(currentSessionId.value, {
      updatedAt: new Date().toISOString() // 这里保留字符串，因为updateSession API期望字符串
    });
  }
}

// 更新最后一条消息
function updateLastMessage(updates: Partial<ChatMessage>) {
  const lastMessage = messages.value[messages.value.length - 1];
  if (lastMessage) {
    Object.assign(lastMessage, updates); // 合并更新
    saveSessionMessages(); // 保存更新后的消息
  }
}

// 删除消息
function removeMessage(index: number) {
  if (index >= 0 && index < messages.value.length) {
    messages.value.splice(index, 1);
    saveSessionMessages();
  }
}

// 发送消息
async function sendMessage(text?: string) {
  const messageText = text || inputMessage.value.trim();
  
  if (!messageText || loading.value) return;
  
  // 清空输入框
  if (!text) {
    inputMessage.value = '';
  }
  
  // 通过API发送消息
  await sendApiMessage(messageText);
}

// 重试发送消息
async function retryMessage(index: number) {
  const lastUserMessageIndex = messages.value
    .slice(0, index)
    .map((msg, i) => ({ ...msg, index: i }))
    .filter(msg => msg.role === 'user')
    .pop();
    
  if (lastUserMessageIndex) {
    removeMessage(index);
    await sendMessage(lastUserMessageIndex.content);
  }
}

// Enter键发送消息
function handleKeyDown(e: KeyboardEvent) {
  if (e.key === 'Enter' && !e.shiftKey && canSendMessage.value) {
    e.preventDefault();
    sendMessage();
  }
}

// 清除当前会话消息
function clearSession() {
  if (currentSessionId.value) {
    messages.value = [];
    saveSessionMessages();
  }
}

// 关闭当前会话
function closeSession() {
  currentSessionId.value = null;
  currentSession.value = null;
  messages.value = [];
}

// 会话管理
function openAgentSelectorForNewSession() {
  // 显示智能体选择器
  agentSelectorVisible.value = true;
}

// 切换会话
async function switchSession(sessionId: number) {
  // 获取会话信息
  const session = chatSessionStore.getSessionById(sessionId);
  
  if (session) {
    currentSessionId.value = sessionId;
    currentSession.value = session;
    
    // 加载会话相关数据
    await loadSessionMessages(sessionId);
    
    // 加载智能体信息 - 添加此调用
    await loadAgentInfo();
    
    // 强制刷新侧边栏
    forceUpdate.value += 1;
  }
}

// 删除会话
function deleteSession(sessionId: number) {
  chatSessionStore.deleteSession(sessionId);
  
  // 如果删除的是当前会话，则关闭当前会话
  if (currentSessionId.value === sessionId) {
    closeSession();
  }
  
  // 如果还有其他会话，则切换到第一个会话
  if (chatSessionStore.sessions.length > 0) {
    switchSession(chatSessionStore.sessions[0].id);
  }
}

// 侧边栏控制
function toggleSidebar() {
  siderCollapsed.value = !siderCollapsed.value;
}

// 处理模型变更
async function handleModelChange(newModel: any) {
  try {
    if (!agent.value || 
        (agent.value.model?.id === newModel.providerId && 
         agent.value.model?.name === newModel.modelName)) return;
    
    loading.value = true;

    const newAgent = {
      ...agent.value,
      model: {
        id: newModel.providerId,
        name: newModel.modelName
      }
    };

    const success = await agentStore.modifyAgent(newAgent);
    
    if (success) {
      agent.value.model = newModel;
    } else {
      message.error('模型更改失败');
    }
  } catch (error) {
    console.error('切换模型时出错:', error);
    message.error('切换模型时出错');
  } finally {
    loading.value = false;
  }
}

// 显示配置弹窗
function showAgentConfig() {
  agentConfigVisible.value = true;
}

// 配置提交
async function handleAgentConfigSubmit(updatedAgent: Agent) {
  try {
    loading.value = true;
    
    const success = await agentStore.modifyAgent(updatedAgent);
    
    if (success) {
      // 修改：使用从服务器返回的完整智能体信息更新本地状态
      agent.value = await agentStore.fetchAgentById(updatedAgent.id);

      // 更改ChatSidebar组件的key，强制重新渲染
      forceUpdate.value += 1;
      
      message.success('智能体配置已更新');
      agentConfigVisible.value = false;
    } else {
      message.error('更新智能体配置失败');
    }
  } catch (error) {
    console.error('更新智能体时出错:', error);
    message.error('更新智能体失败');
  } finally {
    loading.value = false;
  }
}

// 智能体切换 - 创建新会话
async function handleAgentSwitch(selectedAgent: Agent) {
  try {
    loading.value = true;
    
    // 创建带时间戳的会话名称
    const timestamp = new Date().toLocaleString('zh-CN', {
      month: 'numeric',
      day: 'numeric',
      hour: 'numeric',
      minute: 'numeric'
    });
    const sessionTitle = `与${selectedAgent.name}的对话 (${timestamp})`;
    
    // 创建新会话
    const newSession = await chatSessionStore.createNewSession(selectedAgent.id, sessionTitle);
    
    // 切换到新会话
    await switchSession(newSession.id);
    
    message.success('创建会话成功');
  } catch (error) {
    console.error('创建会话失败:', error);
    message.error('创建会话失败');
  } finally {
    loading.value = false;
  }
}

// 使用选定的智能体创建新会话
async function createSessionWithAgent(selectedAgent: Agent) {
  try {
    // 创建带时间戳的会话名称
    const timestamp = new Date().toLocaleString('zh-CN', {
      month: 'numeric',
      day: 'numeric',
      hour: 'numeric',
      minute: 'numeric'
    });
    const sessionTitle = `与${selectedAgent.name}的对话 (${timestamp})`;
    
    // 创建新会话
    const session = await chatSessionStore.createNewSession(selectedAgent.id, sessionTitle);
    
    // 切换到新会话
    await switchSession(session.id);
    
  } catch (error) {
    console.error('创建会话失败:', error);
    message.error('创建会话失败');
  }
}

// 导航到创建智能体页面
function navigateToCreateAgent() {
  router.push('/agents');
}

// 初始化 - 修改为不依赖URL参数
onMounted(async () => {
  // 初始化聊天会话存储
  chatSessionStore.initialize();
  
  // 确保加载了所有智能体数据
  if (agentStore.agents.length === 0) {
    await agentStore.fetchAllAgents();
  }
  
  // 如果有会话，加载第一个会话
  if (chatSessionStore.sessions.length > 0) {
    await switchSession(chatSessionStore.sessions[0].id);
  }
});
</script>

<style scoped>
.chat-container {
  height: 100%;
  display: flex;
  background-color: var(--surface-color, #ffffff);
}

/* 浮动输入框样式 */
.chat-input-floating-container {
  position: sticky;
  bottom: 0;
  padding: 16px;
  background-color: transparent;
  z-index: 10;
}

.chat-input-wrapper {
  display: flex;
  gap: 12px;
  background-color: var(--card-color, #ffffff);
  padding: 16px;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  transition: all 0.3s ease;
  border: 1px solid rgba(0, 0, 0, 0.05);
  max-width: 850px;
  margin: 0 auto;
}

.chat-input-wrapper:hover {
  box-shadow: 0 6px 24px rgba(0, 0, 0, 0.1);
  transform: translateY(-2px);
}

.chat-input {
  flex: 1;
}

.send-button {
  align-self: flex-end;
  border-radius: 8px;
  transition: all 0.2s ease;
}

.send-button:hover:not(:disabled) {
  transform: scale(1.05);
}
</style>
