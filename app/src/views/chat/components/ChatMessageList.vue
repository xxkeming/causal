<template>
  <div class="chat-messages" ref="messagesContainer">
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
            <div class="message-sender">
              {{ message.role === 'user' ? '你' : agentName || 'AI助手' }}
            </div>
            <div class="message-time">
              {{ formatMessageTime(message.timestamp || new Date()) }}
            </div>
          </div>
          <div class="message-content">
            <div class="message-text" v-if="message.status !== 'error'">
              <!-- 显示动图效果 -->
              <div v-if="message.role === 'assistant' && message.status === 'sending'" class="loading-animation">
                <span class="dot"></span>
                <span class="dot"></span>
                <span class="dot"></span>
              </div>
              <!-- 显示正常内容 -->
              <markdown-renderer 
                v-else-if="message.role === 'assistant'" 
                :content="message.content" 
              />
              <n-text v-else>{{ message.content }}</n-text>
            </div>
            <div class="message-error" v-else>
              <div class="custom-error-alert">
                <div class="custom-error-icon">
                  <n-icon><AlertCircleOutline /></n-icon>
                </div>
                <div class="custom-error-content">
                  {{ message.content || '消息发送失败' }}
                </div>
                <div class="custom-error-action">
                  <n-button text @click="$emit('retry', index)">重试</n-button>
                </div>
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
  NAvatar, NIcon, NButton, NText, NSpin 
} from 'naive-ui';
import { ServerOutline, PersonOutline, AlertCircleOutline } from '@vicons/ionicons5';
import MarkdownRenderer from '../../../components/MarkdownRenderer.vue';
import { ChatMessage } from '../../../services/typings';

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

// 定义事件
defineEmits(['retry', 'send']);

// DOM引用
const messagesContainer = ref<HTMLElement | null>(null);

// 消息时间格式化
function formatMessageTime(date: Date): string {
  return new Intl.DateTimeFormat('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false
  }).format(date instanceof Date ? date : new Date(date));
}

// 滚动到底部函数
function scrollToBottom(immediate = false) {
  if (messagesContainer.value) {
    // 立即滚动一次
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
    
    if (!immediate) {
      // 延迟再滚动两次，确保所有内容都已渲染
      setTimeout(() => {
        if (messagesContainer.value) {
          messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
        }
      }, 200);
      
      setTimeout(() => {
        if (messagesContainer.value) {
          messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
        }
      }, 500);
    }
  }
}

// 监听图片加载完成事件
function setupImageLoadListener() {
  nextTick(() => {
    if (messagesContainer.value) {
      const images = messagesContainer.value.querySelectorAll('img');
      if (images.length > 0) {
        images.forEach(img => {
          if (!img.complete) {
            img.onload = () => scrollToBottom();
          }
        });
      }
    }
  });
}

// 监听消息变化，滚动到底部
watch(() => props.messages.length, () => {
  nextTick(() => {
    scrollToBottom();
    setupImageLoadListener();
  });
});

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
</script>

<style scoped>
/* 消息容器样式 */
.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 20px 16px;
  padding-bottom: 140px;
  display: flex;
  flex-direction: column;
  gap: 20px;
  height: 100vh;
  scroll-behavior: smooth;
}

/* 消息项样式 */
.message-item {
  display: flex;
  gap: 12px;
  align-self: flex-start;
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
  flex-direction: column;
  margin-bottom: 8px;
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
  gap: 4px;
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
</style>
