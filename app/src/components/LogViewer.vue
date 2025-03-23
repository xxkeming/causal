<template>
  <div class="log-viewer">
    <div class="log-content" ref="logContentRef">
      <div v-if="logs.length === 0" class="log-empty">
        <n-empty :description="emptyDescription" size="small" />
      </div>
      
      <div v-else class="log-entries">
        <div v-for="(log, index) in logs" :key="index" class="log-entry" :class="getLogClass(log)">
          <div class="log-header">
            <span class="log-time">{{ log.time }}</span>
            <span v-if="log.title" class="log-title" :class="`log-title-${log.type}`">
              {{ log.title }}
            </span>
          </div>
          <div class="log-body">
            {{ log.content }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { NEmpty } from 'naive-ui';

export interface LogEntry {
  time: string;
  type: 'success' | 'error' | 'info' | string;
  title?: string;
  content: string;
}

const props = defineProps({
  logs: {
    type: Array as () => LogEntry[],
    required: true
  },
  emptyDescription: {
    type: String,
    default: '暂无日志'
  },
  autoScroll: {
    type: Boolean,
    default: true
  }
});

const logContentRef = ref<HTMLElement | null>(null);

function getLogClass(log: LogEntry) {
  return {
    [`log-${log.type}`]: true
  };
}

// 滚动到最新日志
function scrollToLatest() {
  setTimeout(() => {
    if (logContentRef.value && props.autoScroll) {
      logContentRef.value.scrollTop = logContentRef.value.scrollHeight;
    }
  }, 50);
}

// 监听日志变化，自动滚动到底部
watch(() => props.logs.length, () => {
  scrollToLatest();
});

onMounted(() => {
  if (props.logs.length > 0) {
    scrollToLatest();
  }
});

defineExpose({
  scrollToLatest
});
</script>

<style scoped>
.log-viewer {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
  background-color: transparent;
}

.log-content {
  flex: 1;
  overflow-y: auto;
  padding: 6px;
  font-family: monospace;
  font-size: 12px;
}

.log-empty {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}

.log-entries {
  display: flex;
  flex-direction: column;
}

.log-entry {
  position: relative;
  padding: 4px 0;
  border-bottom: 1px solid #eaeaea;
}

.log-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 2px;
}

.log-time {
  font-size: 12px;
  color: #999;
  min-width: 45px;
}

.log-title {
  font-weight: 500;
  font-size: 12px;
}

.log-body {
  white-space: pre-wrap;
  word-break: break-word;
  color: #555;
  line-height: 1.3;
  font-size: 12px;
  padding-left: 45px; /* 与时间戳对齐 */
  margin: 2px 0;
}

/* 类型样式 - 只在标题使用颜色，不再使用整个背景和边框 */
.log-title-success {
  color: #18a058;
}

.log-title-error {
  color: #d03050;
}

.log-title-info {
  color: #2080f0;
}

/* 暗色模式适配 */
html.dark .log-entry {
  border-bottom-color: #333;
}

html.dark .log-body {
  color: #bbb;
}

html.dark .log-time {
  color: #777;
}
</style>
