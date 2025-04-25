<template>
  <div class="chat-container">
    <n-layout has-sider>
      <!-- 侧边栏组件 - 添加currentSessionId属性 -->
      <chat-sidebar
        :key="`chat-sidebar-${forceUpdate}`"
        :sessions="chatSessionStore.sessions"
        :current-session-id="currentSessionId"
        v-model:collapsed="siderCollapsed"
        @create="createSessionDirectly"
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
            @show-agent-config="showAgentConfig"
            @show-provider-config="showProviderConfig"
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
            :loading="globalStore.isLoading"
            :suggested-prompts="customQuestions"
            @retry="retryMessage"
            @delete="deleteMessage"
            @send="sendMessage"
            @feedback="handleFeedback"
          />

          <!-- 使用拆分出的输入组件 -->
          <chat-input 
            v-model:stream="stream"
            v-model:search="search"
            v-model:time="time"
            @send="sendMessage"
          />
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
      @select="createSessionWithAgent"
    />

    <!-- 智能体配置弹窗 -->
    <agent-form-modal 
      v-model:visible="agentConfigVisible"
      :is-edit="true"
      :agent-data="agent"
      @submit="handleAgentConfigSubmit"
    />

    <!-- 修改模型提供商配置模态框 -->
    <n-modal 
      v-model:show="providerConfigVisible"
      preset="card"
      class="provider-modal"
      :closable="false"
      style="max-width: 900px; width: 95%; margin-top: 50px"
      :style="{ 
        '--n-modal-margin-top': '50px', 
        '--n-modal-transform-origin': 'center top'
      }"
    >
      <provider-form
        v-if="currentProvider"
        :provider="currentProvider"
        :is-edit="true"
        :embedded="false"
        @submit="handleProviderSubmit"
        @cancel="providerConfigVisible = false"
      />
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from 'vue';
import { useRouter } from 'vue-router';
import { 
  NLayout, NModal, useMessage, useDialog // 添加 NModal 和 useDialog
} from 'naive-ui';
import { Agent, ChatMessage, ChatSession, Attachment, Provider } from '../../services/typings';
import { useAgentStore } from '../../stores/agentStore';
import { useIconStore } from '../../stores/iconStore';
import { useChatSessionStore } from '../../stores/chatSessionStore';
import { useProviderStore} from '../../stores/providerStore';
import { chatEvent, MessageEvent  } from '../../services/api';
import * as api from '../../services/api';
import { useGlobalStore } from '../../stores/globalStore';

// 导入组件
import AgentFormModal from '../agents/components/AgentFormModal.vue';
import ChatHeader from './components/ChatHeader.vue'; 
import ChatSidebar from './components/ChatSidebar.vue';
import AgentSelector from '../../components/AgentSelector.vue';
import ChatMessageList from './components/ChatMessageList.vue';
import NoSessionsAgentGrid from './components/NoSessionsAgentGrid.vue';
import ChatInput from './components/ChatInput.vue'; // 导入新的输入组件
import ProviderForm from '../settings/components/ProviderForm.vue'; // 导入 ProviderForm 组件

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
const dialog = useDialog(); // 在 setup 中添加 dialog

// Store
const agentStore = useAgentStore();
const iconStore = useIconStore();
const chatSessionStore = useChatSessionStore();
const providerStore = useProviderStore(); // 添加 providerStore
const globalStore = useGlobalStore(); // 添加 globalStore

// 本地状态管理会话和消息
const currentSessionId = ref<number | null>(null);
const currentSession = ref<ChatSession | null>(null);
const messages = ref<ChatMessage[]>([]);

// 状态
const agent = ref<Agent | null>(null);
const siderCollapsed = ref(false);
const stream = ref(true); // 添加stream状态
const search = ref(false); // 添加联网搜索状态
const time = ref(true); // 添加附件时间状态
const providerConfigVisible = ref(false); // 添加状态
const currentProvider = ref<Provider | undefined>(undefined); // 添加状态

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

// 建议问题
const customQuestions = computed(() => 
  agent.value?.customQuestions || []
);

// 加载当前会话的消息
async function loadSessionMessages(sessionId: number) {
  try {
    globalStore.setLoadingState(true);
    // 使用API获取会话消息而不是本地存储
    messages.value = await api.getMessagesBySession(sessionId);
  } catch (error) {
    console.error('加载会话消息失败:', error);
    messages.value = [];
  } finally {
    globalStore.setLoadingState(false);
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
      selectedModelValue.value = agent.value.model.id + '|' + agent.value.model.name;
    }
    
  } catch (error) {
    console.error('加载智能体信息失败:', error);
    message.error('加载智能体信息失败');
  }
}

// 添加消息 - 仅更新本地状态，API调用在其他函数中进行
function addMessage(message: ChatMessage) {
  // 添加到本地消息列表
  messages.value.push(message);

  // 如果是有会话的情况，更新会话的更新时间
  if (currentSessionId.value) {
    const timestamp = Date.now();
    chatSessionStore.updateSession(currentSessionId.value, {
      updatedAt: timestamp
    });
  }
}

// 添加滚动到底部的辅助函数
function scrollToBottom() {
  nextTick(() => {
    const chatMessages = document.querySelector('.chat-messages');
    if (chatMessages) {
      chatMessages.scrollTop = chatMessages.scrollHeight;
    }
  });
}

// 发送消息
async function sendMessage(text: string, attachments?: Attachment[]) {
  if ((!text && (!attachments || attachments.length === 0)) || globalStore.isLoading) return;

  if (agent.value === null) {
    message.error('请先选择智能体');
    return;
  }

  try {
    // 添加用户消息
    const userMessage = {
      id: Date.now(), // 临时ID，API会替换
      sessionId: currentSessionId.value as number,
      role: 'user' as const,
      content: text,
      status: 'success', // 使用与API匹配的状态
      createdAt: Date.now(),
      attachments: attachments ? attachments : undefined
    };
    
    // 通过API添加用户消息
    const savedUserMessage = await api.addMessage(userMessage);
    addMessage(savedUserMessage);
    scrollToBottom(); // 用户消息添加后滚动

    // 添加助手消息
    const assistantMessage = {
      id: savedUserMessage.id + 1, // 临时ID，API会替换
      sessionId: currentSessionId.value as number,
      role: 'assistant' as const,
      content: '',
      status: 'sending', // 使用与API匹配的状态
      createdAt: Date.now()
    };
    
    // 通过API添加助手初始消息
    const savedAssistantMessage = await api.addMessage(assistantMessage);
    addMessage(savedAssistantMessage);
    scrollToBottom(); // 助手消息添加后滚动

    const assistantIndex = messages.value.findIndex(msg => msg.id === savedAssistantMessage.id);

    globalStore.setLoadingState(true);
    // 将文件信息传递给API
    await sendApiMessage(agent.value.id as number, currentSessionId.value as number, savedAssistantMessage.id, assistantIndex);
  } catch (error) {
    console.error('API 请求失败:', error);
    message.error('发送消息失败: ' + error);
  } finally {
    globalStore.setLoadingState(false);
  }
}

async function sendApiMessage(agentId: number, sessionId: number, messageId: number, assistantIndex: number) {
  try {
    // 模拟流式输出，将文件信息传递给API
    await chatEvent(agentId, sessionId, messageId, search.value, time.value, stream.value, async (event: MessageEvent) => {
      switch (event.event) {
        case 'started':
          console.log('started');
          messages.value[assistantIndex].content = '';
          break;
        case 'finished':
          // 要等到流式输出完成后再更新最终状态
          messages.value[assistantIndex].status = 'success';

          messages.value[assistantIndex].cost = event.data.cost;
          messages.value[assistantIndex].promptTokens = event.data.promptTokens;
          messages.value[assistantIndex].completionTokens = event.data.completionTokens;
          messages.value[assistantIndex].totalTokens = event.data.totalTokens;

          console.log('finished');
          console.log(JSON.stringify(event.data));

          // 通过API更新最终状态
          await api.updateMessage(messages.value[assistantIndex]);
          break;
        case 'chat':
          // 处理流式输出
          // 更新助手消息的状态和内容
          messages.value[assistantIndex].content = messages.value[assistantIndex].content + event.data.content;
          messages.value[assistantIndex].status = 'processing';
          break;
        case 'tool':
          // 更新工具结果消息
          if (!messages.value[assistantIndex].tools) {
            messages.value[assistantIndex].tools = [];
          }
          messages.value[assistantIndex].tools.push(event.data);
          console.log('tool', messages.value[assistantIndex].tools);
          break;
      }

      await nextTick(); // 确保视图更新
    });

  } catch (error) {
    console.error('API 请求失败:', error);

    const errorMessage = {
      ...messages.value[assistantIndex],
      content: JSON.stringify(error),
      status: 'error'
    };
    
    // 通过API更新错误状态
    await api.updateMessage(errorMessage);
    
    // 更新本地状态
    messages.value[assistantIndex] = errorMessage;
    throw error;
  }
}

// 修复重试消息功能
async function retryMessage(index: number) {
  if (globalStore.isLoading) return;

  const currentMessage = messages.value[index];
  if (!currentMessage || !currentSessionId.value || !agent.value) return;
  
  // 判断索引是否存在, 找到要重试消息之前的最后一条用户消息
  let userMessageIndex = index - 1;
  const userMessage = messages.value[userMessageIndex] as ChatMessage;

  if (!userMessage || userMessage.role !== 'user') {
    message.error('找不到相关的用户消息');
    return;
  }

  try {
    messages.value[index].content = ''; // 清空当前消息内容
    messages.value[index].tools = []; // 清空工具结果
    messages.value[index].status = 'sending'; // 设置状态为发送中
    messages.value[index].createdAt = Date.now(); // 更新创建时间
    await api.updateMessage(messages.value[index]);

    // 重新发送用户消息
    await sendApiMessage(agent.value.id as number, currentSessionId.value as number, currentMessage.id, index);
  } catch (error) {
    console.error('重试消息失败:', error);
    message.error('重试消息失败:' + error);
  } finally {
    globalStore.setLoadingState(false);
  }
  
}

// 删除消息函数
async function deleteMessage(index: number) {
  if (globalStore.isLoading) return;

  const currentMessage = messages.value[index];
  if (!currentMessage) return;
  
  try {
    // 显示确认对话框
    await dialog.warning({
      title: '确认删除',
      content: '会同时删除关联的消息,确认吗?,此操作不可恢复',
      positiveText: '确认',
      negativeText: '取消',
      style: {
        position: 'relative',
        marginTop: '20vh'
      },
      onPositiveClick: async () => {
        if (currentMessage.id) {
          if (currentMessage.role === 'assistant') {
            await api.deleteMessage(currentMessage.id);
            messages.value.splice(index, 1);
            await api.deleteMessage(currentMessage.id - 1);
            messages.value.splice(index - 1, 1);
          } else {
            await api.deleteMessage(currentMessage.id);
            messages.value.splice(index, 1);
            await api.deleteMessage(currentMessage.id + 1);
            messages.value.splice(index, 1);
          }
        }
        message.success('消息已删除');
      }
    });
  } catch (error) {
    console.error('删除消息失败', error);
    message.error('删除消息失败');
  }
}

// 清除当前会话消息
async function clearSession() {
  if (globalStore.isLoading) return;

  if (currentSessionId.value) {
    try {
      // 显示确认对话框
      await dialog.warning({
        title: '确认清除',
        content: '是否清除当前会话的所有消息？此操作不可恢复',
        positiveText: '确认',
        negativeText: '取消',
        style: {
          position: 'relative',
          marginTop: '20vh'
        },
        onPositiveClick: async () => {
          // 通过API删除会话的所有消息
          await api.deleteMessagesBySession(currentSessionId.value as number);
          // 更新本地状态
          messages.value = [];
          message.success('会话消息已清除');
        }
      });
    } catch (error) {
      console.error('清除会话消息失败:', error);
      message.error('清除会话消息失败');
    }
  }
}

// 关闭当前会话
function closeSession() {
  if (globalStore.isLoading) return;

  currentSessionId.value = null;
  currentSession.value = null;
  messages.value = [];

  // 对话关闭, 侧边栏要打开
  siderCollapsed.value = false;
}

// 切换会话
async function switchSession(sessionId: number) {
  if (globalStore.isLoading) return;

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

    // 自动折叠侧边栏，为聊天内容提供更多空间
    // siderCollapsed.value = true;
  }
}

// 删除会话
async function deleteSession(sessionId: number) {
  try {
    const isCurrentSession = currentSessionId.value === sessionId;

    // 如果当前会话正在加载，则不允许删除
    if (globalStore.isLoading && isCurrentSession) return;

    // 调用 store 删除会话
    const success = await chatSessionStore.deleteSession(sessionId);
    
    if (success) {
      message.success('会话已删除');
      
      // 如果删除的是当前会话，则关闭当前会话
      if (isCurrentSession) {
        closeSession();
      }
      
      // 如果还有其他会话并且当前会话被关闭了，则切换到第一个可用会话
      if (chatSessionStore.sessions.length > 0 && !hasActiveSession.value) {
        await switchSession(chatSessionStore.sessions[0].id);
      }
      
      // 强制刷新侧边栏
      forceUpdate.value += 1;
    } else {
      message.error('删除会话失败');
    }
  } catch (error) {
    console.error('删除会话出错:', error);
    message.error('删除会话时出错');
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
        (agent.value.model?.id === Number(newModel.providerId) && 
         agent.value.model?.name === newModel.modelName)) return;
    
    globalStore.setLoadingState(true);

    const newAgent = {
      ...agent.value,
      model: {
        id: Number(newModel.providerId),
        name: newModel.modelName
      }
    };

    await agentStore.modifyAgent(newAgent);
    agent.value.model = newAgent.model;
  } catch (error) {
    console.error('切换模型时出错:', error);
    message.error('切换模型时出错');
  } finally {
    globalStore.setLoadingState(false);
  }
}

// 显示配置弹窗
function showAgentConfig() {
  agentConfigVisible.value = true;
}

// 修改模型提供商配置函数
async function showProviderConfig() {
  if (agent.value?.model?.id) {
    try {
      // 获取提供商详情
      const provider = await providerStore.fetchProviderById(agent.value.model.id);
      if (provider) {
        currentProvider.value = provider;
        providerConfigVisible.value = true;
      } else {
        message.error('获取提供商信息失败');
      }
    } catch (error) {
      console.error('加载提供商失败:', error);
      message.error('加载提供商配置失败');
    }
  }
}

// 处理提供商配置提交
async function handleProviderSubmit(provider: Partial<Provider>) {
  try {
    if (!currentProvider.value?.id) {
      message.error('提供商ID不存在');
      return;
    }
    
    const updatedProvider: Provider = {
      id: currentProvider.value.id,
      name: provider.name || '',
      apiCategory: provider.apiCategory || '',
      url: provider.url || '',
      apiKey: provider.apiKey,
      models: provider.models || []
    };
    
    await providerStore.modifyProvider(updatedProvider);
    message.success('更新提供商配置成功');
    providerConfigVisible.value = false;
    // 重新加载智能体信息以更新模型列表
    await loadAgentInfo();
  } catch (error) {
    console.error('更新提供商失败:', error);
    message.error('更新提供商失败');
  }
}

// 配置提交
async function handleAgentConfigSubmit(updatedAgent: Agent) {
  try {
    globalStore.setLoadingState(true);
    
    await agentStore.modifyAgent(updatedAgent);
    
    // 修改：使用从服务器返回的完整智能体信息更新本地状态
    agent.value = await agentStore.fetchAgentById(updatedAgent.id);

    // 更改ChatSidebar组件的key，强制重新渲染
    forceUpdate.value += 1;
    
    message.success('智能体配置已更新');
    agentConfigVisible.value = false;
  } catch (error) {
    console.error('更新智能体时出错:', error);
    message.error('更新智能体失败');
  } finally {
    globalStore.setLoadingState(false);
  }
}

// 使用选定的智能体创建新会话
async function createSessionWithAgent(selectedAgent: Agent) {
  if (globalStore.isLoading) return;

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

// 改为根据agentId创建会话
async function createSessionDirectly(agentId: number) {
  if (globalStore.isLoading) return;

  // 如果agnetId === 0, 显示智能体选择器
  if (agentId === 0) {
    agentSelectorVisible.value = true;
    return;
  }
  
  // 如果agentId不为0, 创建会话
  // 获取智能体信息用于创建会话标题
  const selectedAgent = await agentStore.fetchAgentById(agentId);
  if (!selectedAgent) {
    message.error('找不到对应的智能体');
    return;
  }
  
  await createSessionWithAgent(selectedAgent);
}

// 处理消息反馈
async function handleFeedback({ messageId, feedback }: { messageId: number; feedback: number }) {
  if (globalStore.isLoading) return;

  try {
    globalStore.setLoadingState(true);
    
    // 查找消息
    const messageIndex = messages.value.findIndex(msg => msg.id === messageId);
    if (messageIndex === -1) {
      message.error('消息不存在');
      return;
    }

    // 更新本地状态
    const updatedMessage = {
      ...messages.value[messageIndex],
      feedback
    };

    // 调用 API 更新消息
    await api.updateMessage(updatedMessage);
    
    // 更新本地消息列表
    messages.value[messageIndex] = updatedMessage;

  } catch (error) {
    console.error('更新反馈失败:', error);
    message.error('更新反馈失败');
  } finally {
    globalStore.setLoadingState(false);
  }
}

// 初始化 - 修改为不依赖URL参数
onMounted(async () => {
  // 初始化聊天会话存储
  chatSessionStore.fetchAllSessions();
  
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

/* 添加提供商模态框样式 */
:deep(.provider-modal) {
  background-color: var(--surface-color, #fff);
  margin-top: 20vh;
}

:deep(.provider-modal .n-card) {
  background-color: var(--surface-color, #fff);
  margin-top: 50px !important;
}

:deep(.provider-modal) :deep(.n-modal-container) {
  align-items: flex-start !important;
}
</style>
