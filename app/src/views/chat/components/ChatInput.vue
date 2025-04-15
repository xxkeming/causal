<template>
  <div class="chat-input-floating-container">
    <div class="chat-input-wrapper">
      <n-input
        v-model:value="inputMessage"
        type="textarea"
        placeholder="输入消息，按Enter发送，Shift+Enter换行..."
        :autosize="{ minRows: 1, maxRows: 20 }"
        class="chat-input"
        @keydown="handleKeyDown"
        :show-count="false"
      />
      
      <div class="input-toolbar">
        <!-- 左侧上传附件按钮和文件列表 -->
        <n-tooltip trigger="hover">
          <template #trigger>
            <div class="toolbar-left">
              <input 
                type="file" 
                ref="fileInputRef" 
                class="native-file-input"
                @change="handleFilesSelected" 
                multiple
                accept=".txt,.md,.json,.csv,.pdf,.doc,.docx,.xls,.xlsx"
              />
              
              <n-button class="tool-button" text @click="triggerFileInput">
                <template #icon>
                  <n-icon><AttachOutline /></n-icon>
                </template>
              </n-button>

              <!-- 文件列表显示区 -->
              <div v-if="selectedFiles.length > 0" class="file-list">
                <div v-for="(file, index) in selectedFiles" :key="index" class="file-tag">
                  <n-icon class="file-icon" :color="fileIconStore.getIconByFilename(file.name).color">
                    <component :is="fileIconStore.getIconByFilename(file.name).icon" />
                  </n-icon>
                  <span class="file-name">{{ file.name }}</span>
                  <n-button quaternary circle size="tiny" @click="removeFile(index)" class="file-remove-btn">
                    <template #icon>
                      <n-icon><CloseCircleOutline /></n-icon>
                    </template>
                  </n-button>
                </div>
              </div>
            </div>
          </template>
          上下文附件
        </n-tooltip>

        <!-- 右侧发送按钮 -->
        <div class="toolbar-right">
          <!-- 替换开关为图标按钮 -->
          <n-tooltip trigger="hover" placement="top">
            <template #trigger>
              <n-button
                text
                :class="streamButtonClass"
                @click="$emit('update:stream', !props.stream)"
              >
                <template #icon>
                  <n-icon>
                    <component :is="props.stream ? FlashOutline : FlashOffOutline" />
                  </n-icon>
                </template>
              </n-button>
            </template>
            {{ props.stream ? '流式输出已开启' : '流式输出已关闭' }}
          </n-tooltip>
          
          <n-button 
            class="tool-button" 
            text
            :disabled="!canSendMessage && !props.loading"
            @click="handleSendOrStop()"
          >
            <template #icon>
              <n-icon>
                <component :is="props.loading ? PauseCircleOutline : SendOutline" />
              </n-icon>
            </template>
          </n-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { NInput, NButton, NIcon, NTooltip, useMessage } from 'naive-ui';
import { 
  SendOutline, 
  AttachOutline, 
  CloseCircleOutline, 
  PauseCircleOutline,
  FlashOutline, // 添加流式输出图标
  FlashOffOutline // 添加非流式输出图标
} from '@vicons/ionicons5';
import { useFileIconStore } from '../../../stores/fileIconStore'; // 导入文件图标存储

const props = defineProps<{
  loading: boolean;
  stream?: boolean; // 添加stream属性
}>();

const emit = defineEmits<{
  (e: 'send', text: string, files?: File[]): void;
  (e: 'update:stream', value: boolean): void;
}>();

const inputMessage = ref('');
const fileInputRef = ref<HTMLInputElement | null>(null);
const selectedFiles = ref<File[]>([]);
const message = useMessage();

// 获取文件图标存储
const fileIconStore = useFileIconStore();

const canSendMessage = computed(() => 
  (inputMessage.value.trim() !== '' || selectedFiles.value.length > 0) && 
  !props.loading
);

// 流式输出按钮样式类计算
const streamButtonClass = computed(() => ({
  'tool-button': true,
  'stream-active': props.stream,
  'stream-inactive': !props.stream
}));

// 触发文件选择对话框
function triggerFileInput() {
  if (fileInputRef.value) {
    fileInputRef.value.click();
  }
}

// 处理选择的文件
function handleFilesSelected(event: Event) {
  const target = event.target as HTMLInputElement;
  if (target.files && target.files.length > 0) {
    // 复制FileList为数组
    const newFiles = Array.from(target.files);
    
    // 文件大小限制检查 (每个文件10MB)
    const maxSize = 10 * 1024 * 1024;
    const oversizedFiles = newFiles.filter(file => file.size > maxSize);
    
    if (oversizedFiles.length > 0) {
      message.warning(`有${oversizedFiles.length}个文件超过10MB大小限制，已被过滤`);
      // 过滤掉超大文件
      const validFiles = newFiles.filter(file => file.size <= maxSize);
      selectedFiles.value.push(...validFiles);
    } else {
      // 所有文件都符合大小要求
      selectedFiles.value.push(...newFiles);
    }
    
    // 重置文件输入框，这样可以再次选择同一文件
    if (fileInputRef.value) {
      fileInputRef.value.value = '';
    }
  }
}

// 移除特定文件
function removeFile(index: number) {
  selectedFiles.value.splice(index, 1);
}

// 清除所有已选择的文件
function clearSelectedFiles() {
  selectedFiles.value = [];
  if (fileInputRef.value) {
    fileInputRef.value.value = '';
  }
}

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
  if ((!text && selectedFiles.value.length === 0) || props.loading) return;
  
  emit('send', text, selectedFiles.value.length > 0 ? [...selectedFiles.value] : undefined);
  inputMessage.value = '';
  clearSelectedFiles();
}

// 处理发送或停止
function handleSendOrStop() {
  if (props.loading) {
    // 停止逻辑（如果需要）
    console.log('Stop action triggered');
  } else {
    sendMessage();
  }
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
  flex-direction: column;
  gap: 12px;
  background-color: #fff;
  padding: 4px 6px;
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
  padding: 0px 6px;
}

.chat-input :deep(.n-input__border),
.chat-input :deep(.n-input__state-border) {
  display: none;
}

.chat-input :deep(.n-input-wrapper) {
  padding: 0px 6px;
  background-color: #fff;
}

.chat-input :deep(textarea) {
  line-height: 1.5;
  background-color: transparent;
  font-size: 15px; /* 调整字体大小 */
}

/* 特别针对 n-input 组件的滚动条隐藏 */
.chat-input :deep(.n-textarea) {
  --n-scrollbar-width: 0 !important;
  --n-scrollbar-color: transparent !important;
}

.chat-input :deep(.n-scrollbar) {
  overflow: hidden !important;
}

.chat-input :deep(.n-scrollbar-rail) {
  display: none;
}

/* 移除可能限制高度的设置，允许文本框自由扩展 */
.chat-input :deep(.n-input__textarea),
.chat-input :deep(.n-input-wrapper) {
  max-height: none;
}

/* 工具栏按钮通用样式 - 同时应用于附件和发送按钮 */
.tool-button {
  width: 36px;
  height: 36px;
  border-radius: 6px;
  display: flex;
  justify-content: center;
  align-items: center;
  color: #666;
  transition: all 0.2s;
  margin-left: 4px;
}

.tool-button:not(:disabled):hover {
  color: var(--primary-color);
  transform: scale(1.1);
}

.tool-button:not(:disabled):active {
  color: var(--primary-color-hover);
  transform: scale(0.95);
}

.tool-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none;
}

/* 附件按钮特殊处理 */
.toolbar-left .tool-button {
  pointer-events: none;
  margin-left: 0;
}

/* 底部工具栏 */
.input-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.toolbar-left {
  display: flex;
  align-items: center;
  position: relative;
}

.native-file-input:hover + .tool-button {
  color: var(--primary-color);
  transform: scale(1.1);
}

.native-file-input:active + .tool-button {
  color: var(--primary-color-hover);
  transform: scale(0.95);
}

/* 文件列表显示区样式 */
.file-list {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  margin-left: 8px;
  overflow: hidden;
}

.file-tag {
  display: flex;
  align-items: center;
  background-color: var(--n-color, rgba(24, 160, 88, 0.1));
  border-radius: 16px;
  padding: 4px 10px;
  font-size: 13px;
  color: var(--n-text-color, #333);
  transition: all 0.2s;
  border: 1px solid var(--n-border-color, rgba(24, 160, 88, 0.2));
}

.file-tag:hover {
  background-color: var(--n-color-hover, rgba(24, 160, 88, 0.15));
}

.file-icon {
  font-size: 15px;
  margin-right: 6px;
  opacity: 0.8;
}

.file-name {
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-remove-btn {
  width: 18px;
  height: 18px;
  margin-left: 6px;
  padding: 0;
  color: var(--n-close-icon-color, rgba(24, 160, 88, 0.6));
  transition: all 0.2s;
}

.file-remove-btn:hover {
  color: var(--n-close-icon-color-hover, rgba(24, 160, 88, 0.8));
  background-color: var(--n-close-color-hover, rgba(24, 160, 88, 0.1));
}

.file-remove-btn:active {
  background-color: var(--n-close-color-pressed, rgba(24, 160, 88, 0.15));
}

/* 自定义文件选择器样式 */
.native-file-input {
  position: absolute;
  width: 36px;
  height: 36px;
  opacity: 0;
  cursor: pointer;
  z-index: 1;
}

.native-file-input::-webkit-file-upload-button {
  display: none;
}

.native-file-input::file-selector-button {
  display: none;
}

/* 隐藏以前的文件信息显示 */
.file-info {
  display: none;
}

/* 文件列表样式 */
.selected-files {
  display: none;
}

.file-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 4px 8px;
  border-radius: 4px;
  background-color: #eaeaea;
}

.file-name {
  font-size: 13px;
  color: #333;
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-right: 8px;
}

.files-count {
  font-size: 13px;
  color: #666;
  margin-left: 8px;
}

.file-icon {
  font-size: 14px;
  margin-right: 4px;
}

.toolbar-right {
  display: flex;
  align-items: center;
  gap: 4px;
}

/* 流式输出按钮样式 */
.stream-active {
  color: var(--primary-color) !important;
}

.stream-inactive {
  color: #999 !important;
}
</style>
