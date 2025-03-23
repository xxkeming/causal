<template>
  <n-modal
    v-model:show="visible"
    preset="dialog"
    title="添加分类"
    positive-text="添加"
    negative-text="取消"
    @positive-click="handleSubmit"
    @negative-click="handleCancel"
    type="info"
    :transform-origin="'center'"
    style="margin-top: 80px;"
  >
    <template #icon>
      <n-icon><AddCircleOutline /></n-icon>
    </template>
    <div class="modal-content">
      <p>请输入新分类名称</p>
      <n-input
        v-model:value="categoryName"
        placeholder="分类名称"
        :maxlength="20"
        @keydown.enter="handleSubmit"
        autofocus
      />
      <div v-if="error" class="error-text">{{ error }}</div>
    </div>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { NModal, NInput, NIcon } from 'naive-ui';
import { AddCircleOutline } from '@vicons/ionicons5';

const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['update:visible', 'submit', 'cancel']);

// 计算属性以转发visible变化
const visible = computed({
  get: () => props.visible,
  set: value => emit('update:visible', value)
});

const categoryName = ref('');
const error = ref('');

// 确认添加
function handleSubmit() {
  if (!categoryName.value.trim()) {
    error.value = '分类名称不能为空';
    return false; // 返回false阻止对话框关闭
  }
  emit('submit', categoryName.value.trim());
  return false; // 返回true允许对话框关闭
}

// 取消添加
function handleCancel() {
  emit('update:visible', false);
  emit('cancel');
}

// 监听可见性变化，当模态框打开时清除之前的错误
watch(() => props.visible, (newVisible) => {
  if (newVisible) {
    categoryName.value = '';
    error.value = '';
  }
});

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

.modal-content {
  margin: 8px 0;
  display: flex;
  flex-direction: column;
}

.error-text {
  color: #d03050;
  font-size: 13px;
  margin-top: 4px;
}

:deep(.n-input) {
  margin-top: 8px;
}
</style>
