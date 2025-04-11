<template>
  <div class="chat-input-floating-container">
    <div class="chat-input-wrapper">
      <n-input
        v-model:value="inputMessage"
        type="textarea"
        placeholder="输入消息，按Enter发送，Shift+Enter换行..."
        :autosize="{ minRows: 2, maxRows: 6 }"
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

<script setup lang="ts">
import { ref, computed } from 'vue';
import { NInput, NButton, NIcon } from 'naive-ui';
import { SendOutline } from '@vicons/ionicons5';

const props = defineProps<{
  loading: boolean;
}>();

const emit = defineEmits<{
  (e: 'send', text: string): void;
}>();

const inputMessage = ref('');

const canSendMessage = computed(() => inputMessage.value.trim() !== '' && !props.loading);

// Enter键发送消息
function handleKeyDown(e: KeyboardEvent) {
  // 如果正在使用输入法编辑（如拼音输入），则不触发发送
  if (e.isComposing || e.keyCode === 229) return;
  
  if (e.key === 'Enter' && !e.shiftKey && canSendMessage.value) {
    e.preventDefault();
    sendMessage();
  }
}

// 发送消息
function sendMessage() {
  const text = inputMessage.value.trim();
  if (!text || props.loading) return;
  
  emit('send', text);
  inputMessage.value = ''; // 清空输入框
}
</script>

<style scoped>
/* 浮动输入框样式 */
.chat-input-floating-container {
  position: sticky;
  bottom: 0;
  padding: 4px;
  background-color: transparent;
  z-index: 10;
}

.chat-input-wrapper {
  display: flex;
  gap: 12px;
  background-color: var(--card-color, #ffffff);
  padding: 4px 12px;
  border-radius: 16px;
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
  min-height: 50px; /* 添加固定的最小高度，确保默认显示2行 */
}

/* 移除输入框边框和背景，使其融入整体 */
.chat-input :deep(.n-input) {
  border: none;
  background: transparent;
}

.chat-input :deep(.n-input__border),
.chat-input :deep(.n-input__state-border) {
  display: none;
}

.chat-input :deep(.n-input-wrapper) {
  padding: 0;
  background-color: transparent;
}

.chat-input :deep(textarea) {
  min-height: 50px;
  line-height: 1.5;
  padding: 0;
  background-color: transparent;
  resize: none; /* 防止用户手动调整大小 */
  font-size: 15px; /* 调整字体大小 */
}

.send-button {
  align-self: center; /* 调整为居中对齐 */
  width: 44px; /* 固定宽度 */
  height: 44px; /* 固定高度使按钮成为正方形 */
  border-radius: 14px; /* 增大圆角 */
  display: flex;
  justify-content: center;
  align-items: center;
  transition: all 0.2s ease;
  padding: 0; /* 重置内边距 */
  margin-left: 8px;
}

.send-button :deep(.n-icon) {
  font-size: 20px; /* 增大图标尺寸 */
  margin-right: 0; /* 确保图标居中 */
}

.send-button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1); /* 悬浮时添加阴影 */
}

.send-button:active:not(:disabled) {
  transform: translateY(0);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1); /* 点击时减小阴影 */
}
</style>
