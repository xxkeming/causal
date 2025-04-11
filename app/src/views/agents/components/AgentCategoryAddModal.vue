<template>
  <!-- 修改为与删除模态框相同的风格 -->
  <n-modal 
    v-model:show="showModal" 
    preset="dialog"
    :title="isEdit ? '编辑分类' : '添加分类'"
    positive-text="确认"
    negative-text="取消"
    @positive-click="handleSubmit"
    @negative-click="closeModal"
    :positive-button-props="{ loading: submitting, disabled: !formValue.name.trim() }"
    :transform-origin="'center'"
    style="margin-top: 80px;"
  >
    <template #icon>
      <n-icon :color="isEdit ? '#2080f0' : '#18a058'">
        <component :is="isEdit ? CreateOutline : AddOutline" />
      </n-icon>
    </template>
    
    <!-- 简化界面 -->
    <n-input 
      v-model:value="formValue.name" 
      placeholder="请输入分类名称" 
      autofocus
      style="margin-top: 12px;"
      :status="showError && !formValue.name.trim() ? 'error' : undefined"
    />
    <div v-if="showError && !formValue.name.trim()" class="error-text">
      请输入分类名称
    </div>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { NModal, NInput, NIcon } from 'naive-ui';
import { AgentCategory } from '../../../services/typings';
import { AddOutline, CreateOutline } from '@vicons/ionicons5';

// Props
const props = defineProps<{
  visible: boolean;
  category?: AgentCategory;
}>();

// Emits
const emit = defineEmits<{
  'update:visible': [value: boolean];
  'submit': [value: string];
  'update': [category: AgentCategory];
}>();

// 表单数据
const formValue = ref({
  name: ''
});

// 提交状态
const submitting = ref(false);
const showError = ref(false);

// 是否为编辑模式
const isEdit = computed(() => !!props.category);

// 显示状态双向绑定
const showModal = computed({
  get: () => props.visible,
  set: (value) => emit('update:visible', value)
});

// 监听visible和category变化，重置表单
watch(
  () => [props.visible, props.category], 
  ([visible, category]) => {
    if (visible) {
      // 如果是编辑模式，填充表单数据
      if (category) {
        formValue.value.name = (category as AgentCategory).name;
      } else {
        // 添加模式，重置表单
        formValue.value.name = '';
      }
      // 重置错误状态
      showError.value = false;
    }
  }
);

// 关闭模态框
function closeModal() {
  showModal.value = false;
}

// 提交表单
function handleSubmit() {
  const name = formValue.value.name.trim();
  
  if (!name) {
    showError.value = true;
    return;
  }
  
  submitting.value = true;
  
  try {
    if (isEdit.value && props.category) {
      // 编辑模式：发出update事件
      emit('update', {
        ...props.category,
        name
      });
    } else {
      // 添加模式：发出submit事件
      emit('submit', name);
    }
    
    // 不要立即关闭模态框，等待父组件处理结果
    // 父组件成功后会关闭模态框
  } finally {
    submitting.value = false;
  }
}
</script>

<style scoped>
.error-text {
  color: #d03050;
  font-size: 13px;
  margin-top: 8px;
}
</style>
