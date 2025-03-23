<template>
  <div class="chat-container">
    <n-layout has-sider>
      <!-- 侧边栏组件 - 添加key属性确保组件重新渲染 -->
      <chat-sidebar
        :key="`chat-sidebar-${forceUpdate}`"
        :sessions="chatSessionStore.sessions"
        :current-session-id="chatSessionStore.currentSessionId"
        v-model:collapsed="siderCollapsed"
        @create="openAgentSelectorForNewSession"
        @switch="switchSession"
        @delete="deleteSession"
      />

      <!-- 右侧聊天内容 -->
      <n-layout>
        <!-- 当有会话时显示聊天界面 -->
        <template v-if="agentId !== '0' && chatSessionStore.sessions.length > 0">
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

    <!-- 智能体选择器组件 - 保留在底部，因为其他地方也用到 -->
    <agent-selector
      v-model:visible="agentSelectorVisible"
      :current-agent-id="agentId"
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
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { 
  NLayout, NIcon, NButton, NInput, useMessage
} from 'naive-ui';
import { SendOutline } from '@vicons/ionicons5';
import { Agent } from '../../services/typings';
import { useAgentStore } from '../../stores/agentStore';
import { useIconStore } from '../../stores/iconStore';
import { useChatSessionStore } from '../../stores/chatSessionStore';

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
const route = useRoute();
const router = useRouter();
const message = useMessage();

// Store
const agentStore = useAgentStore();
const iconStore = useIconStore();
const chatSessionStore = useChatSessionStore();

// 状态
const agent = ref<Agent | null>(null);
const loading = ref(false);
const inputMessage = ref('');
const siderCollapsed = ref(false);

// 计算属性
const agentId = computed(() => route.params.agentId as string);
const agentIcon = computed(() => {
  if (agent.value?.iconId) {
    const icon = iconStore.getIconById(agent.value.iconId);
    return icon ? { icon: icon.icon, color: icon.color } : null;
  }
  return null;
});

const messages = computed(() => chatSessionStore.currentMessages);
const canSendMessage = computed(() => inputMessage.value.trim() !== '' && !loading.value);

// 示例建议问题
const suggestedPrompts = ref([
  '你能做什么?',
  '请介绍一下你自己',
  '帮我写一个简单的Python程序'
]);

// 加载智能体信息
async function loadAgentInfo() {
  try {
    if (!agentId.value || agentId.value === '0') {
      return;
    }
    
    agent.value = await agentStore.fetchAgentById(agentId.value);
    
    if (!agent.value) {
      message.error('无法加载智能体信息');
      router.push('/agents');
      return;
    }

    // 切换智能体
    await chatSessionStore.switchToAgent(agentId.value);
    
    // 更新模型选择器值
    if (agent.value.model) {
      selectedModelValue.value = agent.value.model.id + ':' + agent.value.model.name;
    }
    
  } catch (error) {
    console.error('加载智能体信息失败:', error);
    message.error('加载智能体信息失败');
    router.push('/agents');
  }
}

// 发送API消息
async function sendApiMessage(text: string) {
  if (!agent.value || !text.trim()) return;
  
  try {
    loading.value = true;
    
    // 添加用户消息
    const userMessage = {
      role: 'user' as const,
      content: text,
      timestamp: new Date()
    };
    chatSessionStore.addMessage(userMessage);
    
    // 添加等待消息
    const waitingMessage = {
      role: 'assistant' as const,
      content: '...',
      status: 'sending' as const,
      timestamp: new Date()
    };
    chatSessionStore.addMessage(waitingMessage);
    
    // 模拟API延迟 (实际项目中替换为真实API调用)
    await new Promise(resolve => setTimeout(resolve, 1500));
    
    // 删除等待消息
    const currentMessages = chatSessionStore.currentMessages;
    chatSessionStore.removeMessage(currentMessages.length - 1);
    
    // 添加AI回复
    const sampleResponse = `作为 ${agent.value.name}，我收到了您的消息："${text}"。\n\n这是一个API响应示例。在实际实现中，这将由API返回的真实内容替代。`;
    
    const assistantMessage = {
      role: 'assistant' as const,
      content: sampleResponse,
      status: 'success' as const,
      timestamp: new Date()
    };
    chatSessionStore.addMessage(assistantMessage);
    
  } catch (error) {
    console.error('API 请求失败:', error);
    
    // 删除等待消息
    const currentMessages = chatSessionStore.currentMessages;
    if (currentMessages.length > 0 && 
        currentMessages[currentMessages.length - 1].status === 'sending') {
      chatSessionStore.removeMessage(currentMessages.length - 1);
    }
    
    // 添加错误消息
    const errorMessage = {
      role: 'assistant' as const,
      content: '获取回复失败，请重试',
      status: 'error' as const,
      timestamp: new Date()
    };
    chatSessionStore.addMessage(errorMessage);
    
  } finally {
    loading.value = false;
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
    chatSessionStore.removeMessage(index);
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
  chatSessionStore.clearCurrentSessionMessages();
}

// 关闭当前会话
function closeSession() {
  router.replace('/chat/0');
  chatSessionStore.clearCurrentSessionId();
}

// 会话管理
function openAgentSelectorForNewSession() {
  // 显示智能体选择器
  agentSelectorVisible.value = true;
}

function switchSession(sessionId: number) {
  const session = chatSessionStore.switchSession(sessionId);
  if (session && (agentId.value === '0' || session.agentId !== agentId.value)) {
    router.replace(`/chat/${session.agentId}`);

    // 更新智能体
    agent.value = agentStore.agents.find(agent => agent.id === session.agentId) || null;
    selectedModelValue.value = agent.value?.model?.id + ':' + agent.value?.model?.name;
    
    // 强制刷新侧边栏
    forceUpdate.value += 1;
  }
}

function deleteSession(sessionId: number) {
  chatSessionStore.deleteSession(sessionId);
  // 切换session到第一个
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

// 智能体切换 - 更新处理逻辑，根据模式执行不同操作
async function handleAgentSwitch(selectedAgent: Agent) {
  // 新建会话逻辑 - 为选定的智能体创建新会话
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
    await chatSessionStore.createNewSession(selectedAgent.id, sessionTitle);
    
    // 如果当前不是在该智能体的页面，则跳转
    if (selectedAgent.id !== agentId.value) {
      // 更新当前智能体
      router.push(`/chat/${selectedAgent.id}`);
      agent.value = selectedAgent;
      
      // 更新模型选择器值
      if (selectedAgent.model) {
        selectedModelValue.value = selectedAgent.model.id + ':' + selectedAgent.model.name;
      }
    }
    
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
  // 创建带时间戳的会话名称
  const timestamp = new Date().toLocaleString('zh-CN', {
    month: 'numeric',
    day: 'numeric',
    hour: 'numeric',
    minute: 'numeric'
  });
  const sessionTitle = `与${selectedAgent.name}的对话 (${timestamp})`;
  
  // 创建新会话并导航到聊天页面
  const session = await chatSessionStore.createNewSession(selectedAgent.id, sessionTitle);
  
  // 切换session
  switchSession(session.id);
}

// 导航到创建智能体页面
function navigateToCreateAgent() {
  router.push('/agents');
}

// 初始化
onMounted(async () => {
  // 加载会话记录
  chatSessionStore.loadSessionsAndMessages();
  
  // 确保加载了所有智能体数据
  if (agentStore.agents.length === 0) {
    await agentStore.fetchAllAgents();
  }
  
  // 如果URL中有智能体ID，则加载该智能体
  if (agentId.value) {
    await loadAgentInfo();
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
