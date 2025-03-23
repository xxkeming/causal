<template>
  <n-modal
    v-model:show="visible"
    preset="dialog"
    title="删除分类"
    positive-text="删除"
    negative-text="取消"
    @positive-click="handleConfirm"
    @negative-click="handleCancel"
    type="error"
    :transform-origin="'center'"
    style="margin-top: 80px;"
  >
    <template #icon>
      <n-icon><WarningOutline /></n-icon>
    </template>
    <div class="confirm-content">
      确定要删除分类 <span class="highlight">{{ category?.name }}</span> 吗？
      <div class="confirm-warning">此操作不可恢复，该分类下的智能体将被全部删除。</div>
    </div>
  </n-modal>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { NModal, NIcon } from 'naive-ui';
import { WarningOutline } from '@vicons/ionicons5';
import { AgentCategory } from '../../../services/typings';

const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  },
  category: {
    type: Object as () => AgentCategory | null,
    default: null
  }
});

const emit = defineEmits(['update:visible', 'confirm', 'cancel']);

// 计算属性以转发visible变化
const visible = computed({
  get: () => props.visible,
  set: value => emit('update:visible', value)
});

// 确认删除
function handleConfirm() {
  emit('confirm');
}

// 取消删除
function handleCancel() {
  emit('update:visible', false);
  emit('cancel');
}
</script>

<style scoped>
/* 添加靠上显示的样式 */
:deep(.n-modal-container) {
  margin-top: 8vh !important;
  align-items: flex-start !important;
}

:deep(.n-modal-body-wrapper) {
  align-items: flex-start !important;
}

.confirm-content {
  margin: 16px 0;
  line-height: 1.5;
}

.confirm-warning {
  margin-top: 12px;
  color: #d03050;
  font-size: 13px;
}

.highlight {
  font-weight: 600;
  color: var(--primary-color);
}
</style>
