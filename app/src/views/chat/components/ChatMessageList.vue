<template>
  <div class="chat-messages" ref="scrollContainer" @scroll="handleScroll">
    <!-- 消息列表 -->
    <template v-if="messages.length > 0">
      <div 
        v-for="(message, index) in messages" 
        :key="index" 
        :class="['message-item', message.role === 'user' ? 'message-user' : 'message-assistant']"
      >
        <div class="message-avatar">
          <n-avatar 
            round 
            :size="28" 
            :color="message.role === 'user' ? '#336aea' : (agentIcon?.color || '#10b981')"
          >
            <n-icon size="22">
              <component :is="message.role === 'user' ? PersonOutline : (agentIcon?.icon || ServerOutline)" />
            </n-icon>
          </n-avatar>
        </div>
        <div class="message-wrapper">
          <div class="message-header">
            <div class="message-sender-info">
              <div class="message-sender">
                {{ message.role === 'user' ? '你' : agentName || 'AI助手' }}
              </div>
              <div class="message-time">
                {{ formatDate(message.createdAt) }}
              </div>
            </div>
            
            <!-- 消息操作按钮 -->
            <div class="message-actions" v-if="(message.status === 'success' || message.status === 'error') || !loading">
              
              <!-- 反馈按钮：仅AI消息显示 -->
              <template v-if="message.role === 'assistant'">
                <n-button 
                  text
                  class="tool-button"
                  :class="{ 'active': message.feedback === 1 }"
                  @click="$emit('feedback', { 
                    messageId: message.id, 
                    feedback: message.feedback === 1 ? 0 : 1 
                  })"
                >
                  <template #icon>
                    <n-icon>
                      <component :is="message.feedback === 1 ? ThumbsUp : ThumbsUpOutline" />
                    </n-icon>
                  </template>
                </n-button>
                
                <n-button 
                  text
                  class="tool-button"
                  :class="{ 'active': message.feedback === 2 }"
                  @click="$emit('feedback', { 
                    messageId: message.id, 
                    feedback: message.feedback === 2 ? 0 : 2 
                  })"
                >
                  <template #icon>
                    <n-icon>
                      <component :is="message.feedback === 2 ? ThumbsDown : ThumbsDownOutline" />
                    </n-icon>
                  </template>
                </n-button>
              </template>
              
              <!-- 复制按钮：只在有内容时显示 -->
              <n-button v-if="message.content"
                        text
                        class="tool-button"
                        @click="copyMessage(message)">
                <template #icon>
                  <n-icon><CopyOutline /></n-icon>
                </template>
              </n-button>
              
              <!-- 重试按钮：仅AI消息显示 -->
              <n-button v-if="message.role === 'assistant'" 
                        text
                        class="tool-button"
                        @click="$emit('retry', index)">
                <template #icon>
                  <n-icon><RefreshOutline /></n-icon>
                </template>
              </n-button>
              
              <!-- 删除按钮：所有消息都显示 -->
              <n-button text
                        class="tool-button"
                        @click="$emit('delete', index)">
                <template #icon>
                  <n-icon><TrashOutline /></n-icon>
                </template>
              </n-button>
            </div>
          </div>
          <div class="message-content">
            <div class="message-text" v-if="message.status !== 'error'">
              <!-- 添加工具执行结果显示 -->
              <div v-if="message.tools && message.tools.length > 0" class="tools-results">
                <details v-for="tool in message.tools" :key="tool.id" class="tool-details">
                  <summary class="tool-summary">
                    <n-icon><TerminalOutline /></n-icon>
                    <span class="tool-name">{{ tool.name }}</span>
                  </summary>
                  <div class="tool-content">
                    <div class="tool-section">
                      <div class="tool-label">参数：</div>
                      <pre class="tool-data">{{ formatJson(tool.arguments) }}</pre>
                    </div>
                    <div class="tool-section">
                      <div class="tool-label">结果：</div>
                      <pre class="tool-data">{{ formatJson(tool.result) }}</pre>
                    </div>
                  </div>
                </details>
              </div>
              <!-- 显示动图效果 -->
              <div v-if="message.role === 'assistant' && message.status === 'sending'" class="loading-animation">
                <span class="dot"></span>
                <span class="dot"></span>
                <span class="dot"></span>
              </div>
              <!-- 显示正常内容，添加key强制重渲染 -->
              <markdown-renderer 
                v-else-if="message.role === 'assistant'" 
                :content="message.reasoningContent ? '<think>' + message.reasoningContent + '</think>' +  message.content: message.content" 
                :key="`md-${index}-${message.updatedAt || message.createdAt}`"
              />
              <div v-else class="user-message-content">{{ message.content }}</div>
              
              <!-- 添加统计信息显示 -->
              <div v-if="message.role === 'assistant' && message.status === 'success'" class="message-stats">
                <span v-if="message.totalTokens !== undefined && message.totalTokens > 0">
                  Tokens total: {{ message.totalTokens }} prompt: {{ message.promptTokens }}
                </span>
                <span v-if="message.cost">耗时: {{ formatCost(message.cost) }}</span>
              </div>
            </div>
            <div class="message-error" v-else>
              <div class="custom-error-alert">
                <div class="custom-error-icon">
                  <n-icon><AlertCircleOutline /></n-icon>
                </div>
                <pre class="custom-error-content" v-if="message.content && (message.content.startsWith('{') || message.content.startsWith('['))">{{ formatErrorMessage(message.content) }}</pre>
                <div class="custom-error-content" v-else>{{ message.content || '消息发送失败' }}</div>
                <div class="custom-error-action">
                  <n-button text @click="$emit('retry', index)">重试</n-button>
                </div>
              </div>
            </div>
            
            <!-- 附件列表，仅在用户消息中显示 -->
            <div v-if="message.role === 'user' && message.attachments && message.attachments.length > 0" class="file-list">
              <div v-for="(attachment, idx) in message.attachments" :key="idx" class="file-tag">
                <n-icon class="file-icon" :color="fileIconStore.getIconByFilename(attachment.name).color">
                  <component :is="fileIconStore.getIconByFilename(attachment.name).icon" />
                </n-icon>
                <span class="file-name">{{ attachment.name }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>

    <!-- 初始欢迎消息 -->
    <div class="welcome-container" v-if="messages.length === 0 && agentName">
      <div class="welcome-title">开始与 {{ agentName }} 对话</div>
      <div class="welcome-description" v-if="agentDescription">{{ agentDescription }}</div>
      <div class="suggested-prompts" v-if="suggestedPrompts.length > 0">
        <div class="suggested-prompts-title">建议的问题：</div>
        <div class="suggested-prompts-list">
          <n-button 
            v-for="(prompt, index) in suggestedPrompts" 
            :key="index" 
            class="suggested-prompt-button"
            text
            @click="$emit('send', prompt)"
          >
            {{ prompt }}
          </n-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick, watch } from 'vue';
import { 
  NAvatar, NIcon, NButton 
} from 'naive-ui';
import { 
  ServerOutline, PersonOutline, AlertCircleOutline,
  RefreshOutline, TrashOutline, CopyOutline, TerminalOutline,
  ThumbsUpOutline, ThumbsDownOutline, // 添加点赞图标
  ThumbsUp, ThumbsDown // 添加实心图标用于选中状态
} from '@vicons/ionicons5';
import MarkdownRenderer from '../../../components/MarkdownRenderer.vue';
import { ChatMessage } from '../../../services/typings';
import { useFileIconStore } from '../../../stores/fileIconStore'; // 导入文件图标存储

// 定义节流函数
function throttle(fn: Function, delay: number) {
  let lastCall = 0;
  return function(...args: any[]) {
    const now = Date.now();
    if (now - lastCall >= delay) {
      fn(...args);
      lastCall = now;
    }
  };
}

// 定义props - 修改types
const props = defineProps({
  messages: {
    type: Array as () => ChatMessage[],
    required: true,
  },
  agentName: {
    type: String,
    default: '',
  },
  agentDescription: {
    type: String,
    default: '',
  },
  agentIcon: {
    type: Object as () => { icon: any; color: string } | null | undefined,
    default: null,
  },
  loading: {
    type: Boolean,
    default: false,
  },
  suggestedPrompts: {
    type: Array as () => string[],
    default: () => [],
  },
});

// 定义事件 - 添加删除事件
defineEmits(['retry', 'send', 'delete', 'feedback']);

// DOM引用
const messagesContainer = ref<HTMLElement | null>(null);

// 改进的消息时间格式化
function formatDate(timestamp: number): string {
  if (!timestamp) return '';
  const date = new Date(timestamp);
  return date.toLocaleDateString('zh-CN', { 
    hour: '2-digit', 
    minute: '2-digit'
  });
}

// 添加 JSON 格式化函数
function formatErrorMessage(content: string): string {
  if (!content) return '消息发送失败';
  
  try {
    // 尝试解析 JSON
    if (content.startsWith('{') || content.startsWith('[')) {
      const jsonObj = JSON.parse(content);
      return JSON.stringify(jsonObj, null, 2);
    }
    return content;
  } catch {
    return content;
  }
}

// 添加格式化JSON的方法
function formatJson(jsonString: string): string {
  try {
    const parsed = JSON.parse(jsonString);
    return JSON.stringify(parsed, null, 2);
  } catch {
    return jsonString;
  }
}

// 在 setup 中添加格式化耗时的函数
function formatCost(ms: number): string {
  if (ms < 1000) {
    return `${ms}ms`;
  } else if (ms < 60000) {
    return `${(ms / 1000).toFixed(1)}秒`;
  } else {
    const minutes = Math.floor(ms / 60000);
    const seconds = ((ms % 60000) / 1000).toFixed(1);
    return `${minutes}分${seconds}秒`;
  }
}

// 添加新的响应式变量用于跟踪滚动状态
const isAtBottom = ref(true);
const scrollThreshold = 100; // 距离底部多少像素内认为在底部
const scrollContainer = ref<HTMLDivElement | null>(null);

// 检查是否在底部的函数
const checkIfAtBottom = () => {
  if (!scrollContainer.value) return;
  
  const { scrollHeight, scrollTop, clientHeight } = scrollContainer.value;
  isAtBottom.value = scrollHeight - scrollTop - clientHeight < scrollThreshold;
};

// 处理滚动事件
const handleScroll = throttle(() => {
  checkIfAtBottom();
}, 50);

// 优化的滚动到底部函数
const scrollToBottom = () => {
  if (!scrollContainer.value || !isAtBottom.value) return;
  
  // 使用requestAnimationFrame来优化性能
  requestAnimationFrame(() => {
    if (scrollContainer.value) {
      scrollContainer.value.scrollTop = scrollContainer.value.scrollHeight;
    }
  });
};

// 监听消息列表变化
watch(() => props.messages, (newMessages, oldMessages) => {
  // 如果消息列表为空，无需滚动
  if (!newMessages || newMessages.length === 0) return;
  
  // 只有在以下情况才滚动:
  // 1. 用户已经在查看底部（isAtBottom为true）
  // 2. 或者是有新消息添加（消息数量变多）
  const hasNewMessage = !oldMessages || newMessages.length > oldMessages.length;

  if (isAtBottom.value || hasNewMessage) {
    // 使用nextTick确保DOM已更新
    nextTick(() => {
      scrollToBottom();
    });
  }
}, { immediate: true, deep: true });

// 监听agentIcon变化，强制重新渲染
watch(() => props.agentIcon, () => {
  // 强制组件重新渲染图标
  nextTick(() => {
    console.log('Agent icon updated');
  });
}, { deep: true });

// 组件挂载后设置MutationObserver监听DOM变化
onMounted(() => {
  if (messagesContainer.value) {
    const observer = new MutationObserver(() => {
      scrollToBottom();
    });
    
    observer.observe(messagesContainer.value, {
      childList: true,
      subtree: true,
      characterData: true,
    });
  }
});

// 获取文件图标存储
const fileIconStore = useFileIconStore();

// 复制消息内容
function copyMessage(message: ChatMessage) {
  const text = message.content;
  navigator.clipboard.writeText(text)
    .then(() => {
      // 可以使用 naive-ui 的 message 来显示复制成功提示
      // 但这需要引入 useMessage，这里使用简单的提示
      const button = document.activeElement as HTMLElement;
      if (button) {
        button.classList.add('copied');
        setTimeout(() => button.classList.remove('copied'), 2000);
      }
    })
    .catch(err => console.error('复制失败:', err));
}
</script>

<style scoped>
/* 消息容器样式 */
.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 20px 16px;
  padding-bottom: 120px;
  display: flex;
  flex-direction: column;
  gap: 20px;
  height: 93vh;
  scroll-behavior: smooth;
  align-items: center;
}

/* 消息项样式 */
.message-item {
  display: flex;
  gap: 12px;
  align-self: center;
  max-width: 850px;
  width: 100%;
  padding-bottom: 8px;
  border-bottom: 1px solid #e5e7eb;
  will-change: transform;
}

.message-item:last-child {
  border-bottom: none;
}

.message-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  width: calc(100% - 48px);
}

.message-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.message-sender-info {
  display: flex;
  flex-direction: column;
}

/* 消息操作按钮样式 */
.message-actions {
  display: flex;
  gap: 1px;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.message-item:hover .message-actions {
  opacity: 1;
}

/* 工具按钮通用样式 */
.tool-button {
  width: 28px;
  height: 28px;
  border-radius: 4px;
  display: flex;
  justify-content: center;
  align-items: center;
  color: #666;
  transition: all 0.2s;
}

.tool-button:hover {
  color: var(--primary-color);
  transform: scale(1.1);
}

.tool-button:active {
  color: var(--primary-color-hover);
  transform: scale(0.95);
}

.tool-button :deep(.n-icon) {
  font-size: 16px;
}

.tool-button.copied {
  color: #22c55e;
}

/* 添加反馈按钮激活状态样式 */
.tool-button.active {
  color: var(--primary-color);
}

.tool-button.active:hover {
  transform: none;
}

/* 移除旧的操作按钮样式 */
.message-action-btn {
  display: none;
}

.message-sender {
  font-weight: 500;
  font-size: 14px;
  color: #333;
  line-height: 1.2;
}

.message-time {
  font-size: 12px;
  color: #888;
  line-height: 1.2;
}

.message-content {
  position: relative;
  width: 100%;
  background-color: transparent;
}

.message-text {
  width: 100%;
  overflow-x: auto;
}

/* 错误消息样式 */
.custom-error-alert {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  border-radius: 4px;
  background-color: rgba(237, 60, 80, 0.05);
  color: #e53935;
  border-left: 4px solid #e53935;
  margin-top: 8px;
}

.custom-error-icon {
  margin-right: 8px;
  display: flex;
  align-items: center;
}

.custom-error-content {
  flex: 1;
  padding: 0 4px;
  white-space: pre-wrap;
  font-family: monospace;
  font-size: 13px;
  line-height: 1.4;
  max-height: 200px;
  overflow-y: auto;
}

.custom-error-content::-webkit-scrollbar {
  width: 4px;
}

.custom-error-content::-webkit-scrollbar-thumb {
  background-color: rgba(229, 57, 53, 0.3);
  border-radius: 2px;
}

.custom-error-content::-webkit-scrollbar-track {
  background-color: transparent;
}

.custom-error-action {
  margin-left: 8px;
}

.custom-error-action :deep(.n-button) {
  color: #e53935;
  font-size: 14px;
}

.custom-error-action :deep(.n-button:hover) {
  color: #c62828;
}

/* 加载中指示器 */
.loading-indicator {
  display: flex;
  justify-content: center;
  align-items: center;
  margin-top: 16px;
}

.loading-message {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background-color: rgba(128, 128, 128, 0.1);
  border-radius: 8px;
  font-size: 14px;
  color: var(--text-color-secondary, #888888);
}

/* 欢迎消息样式 */
.welcome-container {
  max-width: 600px;
  margin: 0 auto;
  text-align: center;
  padding: 40px 20px;
}

.welcome-title {
  font-size: 24px;
  font-weight: 600;
  margin-bottom: 16px;
}

.welcome-description {
  font-size: 16px;
  color: var(--text-color-secondary, #888888);
  margin-bottom: 32px;
}

.suggested-prompts {
  margin-top: 32px;
}

.suggested-prompts-title {
  font-weight: 500;
  margin-bottom: 16px;
}

.suggested-prompts-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-width: 400px;
  margin: 0 auto;
}

.suggested-prompt-button {
  text-align: left;
  padding: 10px 16px;
  border: 1px solid var(--border-color, #eaeaea);
  border-radius: 8px;
  transition: all 0.2s;
}

.suggested-prompt-button:hover {
  background-color: var(--hover-color, #f5f5f5);
  transform: translateY(-1px);
}

/* 加载状态作为消息样式 */
.loading-message-item {
  display: none;
}

/* 动图效果样式 */
.loading-animation {
  display: flex;
  gap: 2px;
  align-items: center;
  justify-content: flex-start;
}

.loading-animation .dot {
  width: 8px;
  height: 8px;
  background-color: var(--primary-color, #10b981);
  border-radius: 50%;
  animation: dot-flash 1.2s infinite ease-in-out;
}

.loading-animation .dot:nth-child(2) {
  animation-delay: 0.2s;
}

.loading-animation .dot:nth-child(3) {
  animation-delay: 0.4s;
}

@keyframes dot-flash {
  0%, 80%, 100% {
    opacity: 0;
  }
  40% {
    opacity: 1;
  }
}

/* 附件样式 - 与ChatInput组件中的样式保持一致 */
.file-list {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  margin-top: 8px;
}

.file-tag {
  display: flex;
  align-items: center;
  background-color: #f0f0f0;
  border-radius: 10px;
  padding: 2px 4px;
  font-size: 12px;
  color: #555;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-icon {
  font-size: 14px;
  margin-right: 4px;
  color: #2080f0;
}

.file-name {
  overflow: hidden;
  text-overflow: ellipsis;
}

/* 移除旧的附件样式 */
.attachments-list, .attachments-header, .attachments-items,
.attachment-item, .attachment-icon, .attachment-info,
.attachment-name, .attachment-meta {
  display: none;
}

/* 防止嵌套公式和图表出现重复渲染 */
.message-text :deep(.katex-block .katex-block),
.message-text :deep(.mermaid-container .mermaid-container) {
  display: none;
}

/* 修复消息列表中的Markdown样式 */
.message-text :deep(.markdown-body) {
  padding: 0;
  background: none;
  border: none;
}

/* 工具执行结果样式 */
.tools-results {
  margin: 6px 0;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.tool-details {
  border-radius: 2px;
  border-left: 1px solid #6b7280;
  background-color: #f9fafb;
  overflow: hidden;
}

.tool-summary {
  padding: 3px 5px;
  font-weight: 600;
  color: #4b5563;
  cursor: pointer;
  user-select: none;
  display: flex;
  align-items: center;
  font-size: 14px;
  gap: 6px;
}

.tool-summary::-webkit-details-marker {
  display: none;
}

.tool-content {
  padding: 4px 5px;
  border-top: 1px solid #e5e7eb;
  font-size: 0.95em;
  color: #4b5563;
}

.tool-section {
  margin-bottom: 8px;
}

.tool-section:last-child {
  margin-bottom: 0;
}

.tool-label {
  font-size: 13px;
  color: var(--text-color-secondary, #666);
  margin-bottom: 4px;
}

.tool-data {
  font-family: monospace;
  font-size: 13px;
  line-height: 1.4;
  background-color: var(--hover-color, #f5f5f5);
  padding: 8px;
  border-radius: 4px;
  white-space: pre-wrap;
  word-break: break-all;
}

/* 添加统计信息样式 */
.message-stats {
  margin-top: 8px;
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  font-size: 11px;
  color: #888;
}

.message-stats span {
  white-space: nowrap;
}

/* 添加用户消息内容样式 */
.user-message-content {
  white-space: pre-wrap;
  word-wrap: break-word;
  word-break: break-all;
  font-size: 15px;
  line-height: 1.6;
  color: #24292e;
  padding: 2px 0;
}
</style>
