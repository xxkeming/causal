<template>
  <div class="chat-input-floating-container drop-zone"
    @dragover.prevent.stop="handleDragOver"
    @dragleave.prevent.stop="handleDragLeave"
    @drop.prevent.stop="handleDrop"
    :class="{ 'dragging': isDragging }">
    
    <div class="chat-input-wrapper" :class="{ 'recording-animation': isRecording }">
      <n-input
        v-model:value="inputMessage"
        type="textarea"
        :autosize="{ minRows: 1, maxRows: 20 }"
        class="chat-input"
        @keydown="handleKeyDown"
        :show-count="false"
        :spellcheck="false"
        :autocomplete="false"
        :readonly="isRecording"
        :placeholder="isRecording ? '正在录音中，请说话...' : '输入消息，按Enter发送，Shift+Enter换行...'"
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
          上下文附件 (可拖拽文件到此处)
        </n-tooltip>

        <!-- 右侧发送按钮 -->
        <div class="toolbar-right"> 

          <!-- 联网搜索按钮 -->
          <n-tooltip trigger="hover" placement="top">
            <template #trigger>
              <n-button
                text
                :class="searchButtonClass"
                @click="$emit('update:search', !props.search)"
              >
                <template #icon>
                  <n-icon>
                    <component :is="props.search ? Globe : GlobeOutline" />
                  </n-icon>
                </template>
              </n-button>
            </template>
            {{ props.search ? '联网模式' : '离线模式' }}
          </n-tooltip>

          <!-- 添加时间按钮 -->
          <n-tooltip trigger="hover" placement="top">
            <template #trigger>
              <n-button
                text
                :class="timeButtonClass"
                @click="$emit('update:time', !props.time)"
              >
                <template #icon>
                  <n-icon>
                    <TimeOutline />
                  </n-icon>
                </template>
              </n-button>
            </template>
            {{ props.time ? '上下文包含当前时间' : '上下文不包含当前时间' }}
          </n-tooltip>
          
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
          
          <!-- 语音录入按钮 -->
          <n-tooltip trigger="hover" placement="top">
            <template #trigger>
              <n-button 
                class="tool-button" 
                text 
                :class="{ 'recording-active': isRecording }"
                @click="toggleRecording"
              >
                <template #icon>
                  <n-icon>
                    <component :is="isRecording ? MicSharp : MicOutline" />
                  </n-icon>
                </template>
              </n-button>
            </template>
            {{ isRecording ? '正在录音...' : '语音输入' }}
          </n-tooltip>
          
          <n-button 
            class="tool-button" 
            text
            :disabled="!canSendMessage && !globalStore.isLoading"
            @click="handleSendOrStop()"
          >
            <template #icon>
              <n-icon>
                <component :is="globalStore.isLoading ? PauseCircleOutline : SendOutline" />
              </n-icon>
            </template>
          </n-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useGlobalStore } from '../../../stores/globalStore';
import { NInput, NButton, NIcon, NTooltip, useMessage } from 'naive-ui';
import { 
  SendOutline, 
  AttachOutline, 
  CloseCircleOutline, 
  PauseCircleOutline,
  FlashOutline, // 添加流式输出图标
  FlashOffOutline, // 添加非流式输出图标
  Globe,    // 修改为正确的图标名称
  GlobeOutline, // 修改为正确的离线模式图标
  TimeOutline, // 添加时间图标
  MicOutline, // 添加麦克风图标
  MicSharp // 添加激活状态麦克风图标
} from '@vicons/ionicons5';
import { useFileIconStore } from '../../../stores/fileIconStore'; // 导入文件图标存储
import { Attachment } from '../../../services/typings';
import { convertFile, transcriptionsAudioMessage } from '../../../services/api';

const props = defineProps<{
  stream?: boolean; // 添加stream属性
  search?: boolean; // 添加联网搜索属性
  time?: boolean; // 添加time属性
}>();

const globalStore = useGlobalStore();

const emit = defineEmits<{
  (e: 'send', text: string, attachments?: Attachment[]): void;
  (e: 'sendExit'): void;
  (e: 'update:stream', value: boolean): void;
  (e: 'update:search', value: boolean): void; // 添加事件
  (e: 'update:time', value: boolean): void; // 添加时间事件
}>();

const inputMessage = ref('');
const fileInputRef = ref<HTMLInputElement | null>(null);
const selectedFiles = ref<File[]>([]);
const selectedAttachments = ref<Attachment[]>([]);
const message = useMessage();
const isDragging = ref(false);
const isRecording = ref(false); // 录音状态

// 获取文件图标存储
const fileIconStore = useFileIconStore();

const canSendMessage = computed(() => 
  (inputMessage.value.trim() !== '' || selectedAttachments.value.length > 0) && 
  !globalStore.isLoading
);

// 流式输出按钮样式类计算
const streamButtonClass = computed(() => ({
  'tool-button': true,
  'stream-active': props.stream,
  'stream-inactive': !props.stream
}));

// 联网搜索按钮样式计算
const searchButtonClass = computed(() => ({
  'tool-button': true,
  'search-active': props.search,
  'search-inactive': !props.search
}));

// 添加时间按钮样式计算
const timeButtonClass = computed(() => ({
  'tool-button': true,
  'time-active': props.time,
  'time-inactive': !props.time
}));

// 触发文件选择对话框
function triggerFileInput() {
  if (fileInputRef.value) {
    fileInputRef.value.click();
  }
}

// 添加文件处理公共函数
async function handleFiles(files: File[]) {
  if (files.length === 0) return;
  
  const maxSize = 10 * 1024 * 1024;
  // 过滤重复文件和大小限制
  const validFiles = files.filter(file => {
    const isDuplicate = selectedFiles.value.some(existingFile => 
      existingFile.name === file.name && existingFile.size === file.size
    );
    const isOversize = file.size > maxSize;
    
    if (isDuplicate) {
      message.warning(`文件 ${file.name} 已存在`);
    }
    if (isOversize) {
      message.warning(`文件 ${file.name} 超过10MB大小限制`);
    }
    
    return !isDuplicate && !isOversize;
  });

  if (validFiles.length === 0) return;

  try {
    // 开始转换时设置loading状态
    globalStore.setLoadingState(true);
    
    // 转换为 Attachment 对象
    const newAttachments = await Promise.all(validFiles.map(file => {
      return new Promise<Attachment>((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => {
            const base64Data = (reader.result as string).split(',')[1];
            const attachment: Attachment = {
              name: file.name,
              size: file.size,
              data: base64Data
            };
            resolve(attachment);
        };
        reader.onerror = () => reject(new Error(`读取文件失败: ${file.name}`));
        reader.readAsDataURL(file);
      });
    }));

    // 遍历附件, 然后转换
    for (const attachment of newAttachments) {
      try {
        const data = await convertFile(attachment.name, attachment.data);
        attachment.data = data;
        attachment.size = data.length;
      } catch (error) {
        console.error('附件解析失败:', error);
        message.error(`附件解析失败: ${attachment.name}`);
        return;
      }
    }

    selectedFiles.value.push(...validFiles);
    selectedAttachments.value.push(...newAttachments);
    message.success(`成功添加 ${newAttachments.length} 个文件`);
  } catch (error) {
    console.error('文件处理失败:', error);
    message.error('文件处理失败');
  } finally {
    // 确保在完成后重置loading状态
    globalStore.setLoadingState(false);
  }
}

// 处理选择的文件
function handleFilesSelected(event: Event) {
  const target = event.target as HTMLInputElement;
  if (target.files) {
    handleFiles(Array.from(target.files));
    target.value = ''; // 重置输入框
  }
}

// 移除特定文件
function removeFile(index: number) {
  selectedFiles.value.splice(index, 1);
  selectedAttachments.value.splice(index, 1);
}

// 清除所有已选择的文件
function clearSelectedFiles() {
  selectedFiles.value = [];
  selectedAttachments.value = [];
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
  if ((!text && selectedAttachments.value.length === 0) || globalStore.isLoading) return;
  
  emit('send', text, selectedAttachments.value.length > 0 ? [...selectedAttachments.value] : undefined);
  inputMessage.value = '';
  clearSelectedFiles();
}

// 处理发送或停止
function handleSendOrStop() {
  if (globalStore.isLoading) {
    emit('sendExit');
  } else {
    sendMessage();
  }
}

// 切换录音状态
function toggleRecording() {
  if (!isRecording.value) {
    // 开始录音
    startRecording();
  } else {
    // 停止录音
    stopRecording();
  }
  isRecording.value = !isRecording.value;
}

// 开始录音
async function startRecording() {
  try {
    // 设置全局加载状态为录音中
    globalStore.setLoadingState(true);
    
    // 请求麦克风权限
    const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
    const mediaRecorder = new MediaRecorder(stream);
    let audioChunks: BlobPart[] = [];

    mediaRecorder.ondataavailable = (event) => {
      audioChunks.push(event.data);
    };

    mediaRecorder.onstop = async () => {
      const audioBlob = new Blob(audioChunks, { type: 'audio/wav' });
      audioChunks = [];

      // 处理音频文件（如上传或转录）
      await handleAudioFile(audioBlob);
      
      // 完成录音和处理后，重置全局加载状态
      globalStore.setLoadingState(false);
    };

    mediaRecorder.start();
    globalStore.setMediaRecorder(mediaRecorder);
  } catch (error) {
    console.error('录音失败:', error);
    message.error('录音失败，请检查麦克风设置');
    isRecording.value = false;
    // 错误情况下也需要重置全局加载状态
    globalStore.setLoadingState(false);
  }
}

// 停止录音
function stopRecording() {
  const mediaRecorder = globalStore.getMediaRecorder();
  if (mediaRecorder) {
    mediaRecorder.stop();
    globalStore.setMediaRecorder(null);
  }
}

// 处理音频文件
async function handleAudioFile(audioBlob: Blob) {
  try {
    // 将音频Blob转换为base64字符串
    const reader = new FileReader();
    const base64Promise = new Promise<string>((resolve, reject) => {
      reader.onload = () => {
        const base64String = (reader.result as string).split(',')[1]; // 获取base64部分
        resolve(base64String);
      };
      reader.onerror = () => reject(new Error('读取音频文件失败'));
      reader.readAsDataURL(audioBlob);
    });

    const base64Data = await base64Promise;
    // 调用转录接口
    const transcript = await transcriptionsAudioMessage(base64Data);
    
    // 将转录结果插入到输入框
    if (transcript.length > 0) {
      inputMessage.value += transcript;
    }
  } catch (error) {
    console.error('音频处理失败:', error);
    message.error('音频处理失败,请检查相关系统配置,或者重试(' + JSON.stringify(error) + ')');
  }
}

// 处理拖拽相关事件
function handleDragOver(e: DragEvent) {
  isDragging.value = true;
  e.dataTransfer!.dropEffect = 'copy';
}

function handleDragLeave() {
  isDragging.value = false;
}

// 处理拖拽文件
function handleDrop(e: DragEvent) {
  isDragging.value = false;
  if (e.dataTransfer?.files) {
    handleFiles(Array.from(e.dataTransfer.files));
  }
}

// 添加全局拖拽阻止
onMounted(() => {
  document.addEventListener('dragover', (e) => {
    // 检查是否在指定的拖拽区域内
    const dropZone = e.target as Element;
    if (!dropZone.closest('.drop-zone')) {
      e.preventDefault();
      e.dataTransfer!.effectAllowed = 'none';
      e.dataTransfer!.dropEffect = 'none';
    }
  });

  document.addEventListener('drop', (e) => {
    // 检查是否在指定的拖拽区域内
    const dropZone = e.target as Element;
    if (!dropZone.closest('.drop-zone')) {
      e.preventDefault();
    }
  });
});
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
  background-color: rgb(237, 237, 239);
  padding: 4px 6px;
  border-radius: 16px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  transition: all 0.3s ease;
  border: 1px solid rgba(0, 0, 0, 0.05);
  max-width: 820px;
  margin: 0 auto;
}

.chat-input-wrapper:hover {
  box-shadow: 0 6px 24px rgba(0, 0, 0, 0.1);
  transform: translateY(-2px);
}

.chat-input {
  flex: 1;
}

.chat-input :deep(.n-input__border),
.chat-input :deep(.n-input__state-border) {
  display: none;
}

.chat-input :deep(.n-input-wrapper) {
  padding: 0px 12px;
  background-color: rgb(237, 237, 239);
}

.chat-input :deep(textarea) {
  line-height: 1.5;
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
  margin-left: 2px;
  overflow: hidden;
}

.file-tag {
  display: flex;
  align-items: center;
  background-color: var(--n-color, rgba(24, 160, 88, 0.1));
  border-radius: 10px;
  padding: 2px 4px;
  font-size: 12px;
  color: var(--n-text-color, #333);
  transition: all 0.2s;
  border: 1px solid var(--n-border-color, rgba(24, 160, 88, 0.2));
}

.file-tag:hover {
  background-color: var(--n-color-hover, rgba(24, 160, 88, 0.15));
}

.file-remove-btn {
  width: 18px;
  height: 18px;
  margin-left: 2px;
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

.file-name {
  font-size: 13px;
  color: #333;
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-right: 8px;
}

.file-icon {
  font-size: 14px;
  margin-right: 4px;
}

.toolbar-right {
  display: flex;
  align-items: center;
  gap: 0;
}

.toolbar-right .tool-button {
  margin: 0;
  padding: 0;
  width: 32px; /* 减小按钮尺寸 */
  height: 32px;
}

/* 流式输出按钮样式 */
.stream-active {
  color: var(--primary-color) !important;
}

.stream-inactive {
  color: #999 !important;
}

/* 联网搜索按钮样式 */
.search-active {
  color: var(--primary-color) !important;
}

.search-inactive {
  color: #999 !important;
}

/* 添加时间按钮样式 */
.time-active {
  color: var(--primary-color) !important;
}

.time-inactive {
  color: #999 !important;
}

/* 简化拖拽区域样式 */
.drop-zone {
  width: 100%;
  margin: 0 auto;
}

.drop-zone.dragging .chat-input-wrapper {
  border: 2px dashed var(--primary-color);
}

/* 录音按钮激活状态样式 */
.recording-active {
  color: var(--primary-color) !important;
}

.recording-inactive {
  color: #999 !important;
}

/* 录音中的输入框动画效果 */
@keyframes recording-pulse {
  0% {
    box-shadow: 0 0 0 0 rgba(255, 0, 0, 0.2);
  }
  70% {
    box-shadow: 0 0 0 6px rgba(255, 0, 0, 0);
  }
  100% {
    box-shadow: 0 0 0 0 rgba(255, 0, 0, 0);
  }
}

.recording-animation {
  animation: recording-pulse 2s infinite;
  border: 1px solid rgba(255, 0, 0, 0.3) !important;
  transition: all 0.3s ease;
}

</style>
