<template>
  <div class="provider-details">
    <div class="details-header">
      <h2>{{ isEdit ? '编辑模型提供商' : '新建模型提供商' }}</h2>
      <div class="details-actions">
        <n-button v-if="!isEdit" @click="$emit('cancel')" class="cancel-btn">
          取消
        </n-button>
        <n-button type="primary" @click="handleSubmit" :loading="submitting">
          {{ isEdit ? '保存' : '创建' }}
        </n-button>
        <n-button v-if="isEdit && embedded" tertiary type="error" @click="handleDelete">
          删除
        </n-button>
        <n-button v-if="!embedded" class="cancel-btn" @click="$emit('cancel')">
          关闭
        </n-button>
      </div>
    </div>
    
    <n-divider />
    
    <!-- 原有的表单内容 -->
    <n-form
      ref="formRef"
      :model="formData"
      :rules="formRules"
      label-placement="left"
      label-width="80px"
      require-mark-placement="right-hanging"
      class="provider-form"
    >
      <!-- 将名称和API类别放在一行 -->
      <div class="form-row">
        <n-form-item label="名称" path="name" required class="form-item-name">
          <n-input v-model:value="formData.name" placeholder="请输入提供商名称" />
        </n-form-item>
        
        <n-form-item label="接口类型" path="apiCategory" required class="form-item-category">
          <n-select
            v-model:value="formData.apiCategory"
            :options="apiCategoryOptions"
            placeholder="请选择API接口类型"
          />
        </n-form-item>
      </div>
      
      <n-form-item required>
        <!-- 修改为使用自定义标签模板 -->
        <template #label>
          <div class="label-with-help">
            <n-popover trigger="hover" placement="top">
              <template #trigger>
                <n-icon size="16" class="help-icon">
                  <HelpCircleOutline />
                </n-icon>
              </template>
              <span class="popover-content">服务地址，例如：https://api.example.com/v1，如果是完整的地址以@结束，例如：https://api.example.com/v1/chat/completions@</span>
            </n-popover>
            <span style="margin-left: 8px;">地址</span>
          </div>
        </template>
        <div class="api-input-wrapper">
          <n-input v-model:value="formData.url" placeholder="请输入 API 地址" />
          <!-- 将预览放在输入框的容器里，而不是直接放在表单项内 -->
          <div class="api-path-preview" v-if="formData.url">
            <span class="api-path-value">{{ completionApiPath }}</span>
          </div>
        </div>
      </n-form-item>

      <n-form-item label="密钥" path="apiKey">
        <n-input
          v-model:value="formData.apiKey"
          type="password"
          show-password-on="click"
          placeholder="请输入 API 密钥"
        />
      </n-form-item>
      
      <n-divider>模型列表</n-divider>
      
      <div class="models-section">
        <div v-if="formData.models && formData.models.length > 0" class="models-list">
          <div
            v-for="(model, index) in formData.models"
            :key="index"
            class="model-item"
          >
            <div class="model-item-content">
              <n-input
                v-model:value="model.name"
                placeholder="模型名称"
                size="small"
                class="model-name-input"
                :status="!model.name && submitAttempted ? 'error' : undefined"
                required
              />
              
              <n-select
                v-model:value="model.tags"
                multiple
                placeholder="选择标签"
                size="small"
                :options="modelTagOptions"
                class="model-tags-select"
                :status="(!model.tags || model.tags.length === 0) && submitAttempted ? 'error' : undefined"
                required
              />
              
              <n-button
                quaternary
                circle
                size="small"
                @click="removeModel(index)"
                class="model-delete-btn"
              >
                <template #icon>
                  <n-icon><TrashOutline /></n-icon>
                </template>
              </n-button>
            </div>
            <div v-if="submitAttempted && (!model.name || !model.tags || model.tags.length === 0)" class="model-error-tip">
              <span v-if="!model.name" class="error-text">请输入模型名称</span>
              <span v-if="!model.tags || model.tags.length === 0" class="error-text">请至少选择一个标签</span>
            </div>
          </div>
        </div>
        
        <div v-else class="empty-models">
          <n-empty description="暂无模型" size="small" />
        </div>
        
        <div class="add-model-btn">
          <n-button dashed @click="addNewModel">
            <template #icon>
              <n-icon><AddOutline /></n-icon>
            </template>
            添加模型
          </n-button>
        </div>
      </div>
    </n-form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import {
  NForm,
  NFormItem,
  NInput,
  NSelect,
  NButton,
  NDivider,
  NIcon,
  NPopover,
  NEmpty,
  useDialog,
  useMessage,
  FormRules
} from 'naive-ui';
import { HelpCircleOutline, TrashOutline, AddOutline } from '@vicons/ionicons5';
import { Provider, Model } from '../../../services/typings';
import { ProviderApiCategory } from '../../../stores/providerStore';

// Props 定义
const props = defineProps<{
  provider?: Provider,
  isEdit?: boolean,
  embedded?: boolean
}>();

// Emits 定义
const emit = defineEmits<{
  (e: 'submit', provider: Partial<Provider>): void;
  (e: 'delete', id: number): void;
  (e: 'cancel'): void;
}>();

const message = useMessage();
const dialog = useDialog();

// 表单状态
const formRef = ref(null);
const submitting = ref(false);
const submitAttempted = ref(false);

// 表单数据
const formData = ref({
  name: '',
  apiCategory: '',
  url: '',
  apiKey: '',
  models: [] as Model[]
});

// 监听 provider 变化，更新表单数据
watch(
  () => props.provider,
  (newProvider) => {
    if (newProvider) {
      // 使用深拷贝避免直接引用
      formData.value = JSON.parse(JSON.stringify({
        name: newProvider.name,
        apiCategory: newProvider.apiCategory,
        url: newProvider.url,
        apiKey: newProvider.apiKey || '',
        models: newProvider.models || []
      }));
    } else {
      // 重置表单
      formData.value = {
        name: '',
        apiCategory: '',
        url: '',
        apiKey: '',
        models: []
      };
    }
  },
  { immediate: true, deep: true }
);

// 表单验证规则
const formRules: FormRules = {
  name: {
    required: true,
    message: '请输入提供商名称',
    trigger: ['blur', 'change']
  },
  apiCategory: {
    required: true,
    message: '请选择API类别',
    trigger: ['blur', 'change']
  },
  // 修改为更严格的URL格式验证规则
  url: {
    required: true,
    validator: (_rule, value) => {
      if (!value) {
        return new Error('请输入API地址');
      }
      // 使用更严格的URL验证正则表达式
      const urlPattern = /^https?:\/\/[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&//=]*)$/i;
      if (!urlPattern.test(value)) {
        return new Error('请输入有效的http://或https://开头的URL地址');
      }
      return true;
    },
    trigger: ['blur', 'change']
  }
};

// 添加计算属性实时计算API路径, 如果以@结尾, 则去掉结尾的@符号
const completionApiPath = computed(() => {
  if (!formData.value.url) return '';
  
  // 移除末尾的斜杠（如果有）
  let baseUrl = formData.value.url.trim();
  // if (baseUrl.endsWith('/')) {
  //   baseUrl = baseUrl.slice(0, -1);
  // }
  if (baseUrl.endsWith('@')) {
    return baseUrl.slice(0, -1);
  } else {
    return `${baseUrl}/chat/completions`;
  }
});

// API 类别选项
const apiCategoryOptions = ProviderApiCategory.map(category => ({
  label: category,
  value: category
}));

// 模型标签选项
const modelTagOptions = [
  { label: '推理', value: '推理' },
  { label: '工具', value: '工具' },
  { label: '向量', value: '向量' },
  { label: '图片', value: '图片' }
];

// 提交表单
async function handleSubmit() {
  if (!formRef.value) return;
  
  try {
    submitAttempted.value = true;
    
    if (!validateModelList()) {
      message.error('请完善所有模型的名称和标签');
      return;
    }
    
    // @ts-ignore
    await formRef.value.validate();
    
    submitting.value = true;
    
    const providerData = {
      ...props.provider,
      name: formData.value.name,
      apiCategory: formData.value.apiCategory,
      url: formData.value.url,
      apiKey: formData.value.apiKey,
      models: formData.value.models
    };

    emit('submit', providerData);
  } catch (error) {
    console.error('Form validation failed:', error);
    if (Array.isArray(error)) {
      message.error(error[0]?.message || '表单验证失败');
    }
  } finally {
    submitting.value = false;
  }
}

// 删除确认
async function handleDelete() {
  if (!props.provider?.id) return;
  
  try {
    await dialog.warning({
      title: '确认删除',
      content: '确定要删除这个模型提供商吗？此操作不可恢复。',
      positiveText: '确认',
      negativeText: '取消',
      style: {
        position: 'relative',
        marginTop: '20vh'
      },
      onPositiveClick: () => {
        emit('delete', props.provider!.id);
      }
    });
  } catch (error) {
    console.error('删除提供商失败:', error);
  }
}

// 添加新模型
function addNewModel() {
  if (!formData.value.models) {
    formData.value.models = [];
  }
  formData.value.models.push({ 
    name: '', 
    tags: [] as string[] // 明确标注tags类型为字符串数组
  });
}

// 移除模型
function removeModel(index: number) {
  if (formData.value.models) {
    formData.value.models.splice(index, 1);
  }
}

// 验证表单中的模型列表
function validateModelList(): boolean {
  if (!formData.value.models || formData.value.models.length === 0) {
    return true; // 如果没有模型则算通过验证
  }
  
  // 检查每一个模型的名称和标签
  for (const model of formData.value.models) {
    if (!model.name || !model.tags || model.tags.length === 0) {
      return false;
    }
  }
  
  return true;
}
</script>

<style scoped>
.provider-form-container {
  max-width: 100%;
  margin: 0 auto;
  padding: 0 24px 60px; /* 增加左右内边距，并保留底部内边距 */
}

.details-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.details-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 500;
}

.provider-form {
  margin-top: 24px;
}

.models-section {
  margin: 20px 0;
}

/* 修改模型列表样式 */
.models-list {
  display: flex;
  flex-direction: column;
  gap: 4px; /* 减小间距，原来是12px */
  margin-bottom: 16px;
}

.model-item-content {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 4px 12px;
  border-radius: 0; /* 移除圆角 */
  background-color: transparent; /* 移除背景色 */
  border: none; /* 移除边框 */
}

/* 最后一个元素不需要下边框 */
.models-list .model-item:last-child .model-item-content {
  border-bottom: none;
}

.model-item-content:hover {
  background-color: rgba(128, 128, 128, 0.04); /* 悬停时的轻微背景色 */
}

.model-name-input {
  width: 400px;
  flex-shrink: 0;
}

.model-tags-select {
  flex-grow: 1;
}

.model-delete-btn {
  flex-shrink: 0;
}

.empty-models {
  padding: 24px 0;
}

.add-model-btn {
  margin-top: 8px;
}

.details-actions {
  display: flex;
  justify-content: flex-end;
  gap: 6px;
}

.empty-state {
  margin-top: 100px;
}

:deep(.n-list-item) {
  padding: 0 !important;
  border-bottom: none !important;
}

:deep(.n-list) {
  --n-divider-color: transparent;
  --n-border-color: transparent;
}

:deep(.n-form-item .n-form-item-label) {
  font-weight: 500;
}

/* 添加名称和API类别输入框的宽度控制 */
.form-input {
  max-width: 300px;
  width: 100%;
}

/* 自定义滚动条样式，确保完全隐藏 */
:deep(.n-scrollbar) {
  --n-scrollbar-width: 0 !important;
  --n-scrollbar-height: 0 !important;
  --n-scrollbar-color: transparent !important;
  --n-scrollbar-color-hover: transparent !important;
}

:deep(.n-scrollbar-rail) {
  width: 0;
  height: 0 !important;
  opacity: 0 !important;
}

:deep(.n-scrollbar-container) {
  scrollbar-width: none !important;
  -ms-overflow-style: none !important;
}

:deep(.n-scrollbar-container::-webkit-scrollbar) {
  width: 0 !important;
  height: 0 !important;
  display: none !important;
  background: transparent;
}

/* 添加错误文本样式 */
.error-text {
  color: var(--error-color);
  font-size: 12px;
  margin-top: 4px;
  display: inline-block;
  margin-right: 12px;
}

.model-error-tip {
  padding-left: 12px;
  margin-top: 4px;
}

/* 移除不再需要的表单项布局调整 */
:deep(.n-form-item-feedback-wrapper) {
  display: block; /* 改回块级元素 */
}

/* 让表单反馈项显示在下方 */
:deep(.n-form-item-feedback) {
  margin-top: 4px;
}

/* 添加水平表单行样式 */
.form-row {
  display: flex;
  gap: 20px;
  width: 100%;
  margin-bottom: 6px;
}

.form-item-name {
  flex: 1;
  min-width: 200px;
}

.form-item-category {
  flex: 1;
  min-width: 200px;
}

.form-item-stream {
  flex: 0 0 auto;
  min-width: 120px;
  margin-left: 20px;
}

/* 确保表单项在同一行正确显示 */
:deep(.n-form-item-label) {
  white-space: nowrap;
}

/* 防止表单项内容溢出 */
:deep(.n-form-item-blank) {
  width: 100%;
}

/* API路径预览样式 */
.api-path-preview {
  font-size: 12px;
  color: var(--text-color-secondary);
  display: block;
  margin-top: 2px;
  margin-bottom: 0;
  width: 100%;
}

.api-path-value {
  color: var(--text-color-secondary);
  font-family: monospace;
  word-break: break-all;
  padding-left: 12px;
}

/* 添加输入框容器样式 */
.api-input-wrapper {
  width: 100%;
  position: relative;
}

/* 添加帮助图标相关样式 */
.help-icon {
  color: var(--text-color-3);
  margin-right: 4px;
  cursor: pointer;
  opacity: 0.7;
  transition: all 0.2s;
}

.help-icon:hover {
  opacity: 1;
  color: var(--primary-color);
  transform: scale(1.1);
}

.label-with-help {
  display: flex;
  align-items: center;
}

/* 提示框内容样式 */
.popover-content {
  font-size: 13px;
  line-height: 1.6;
  max-width: 300px;
}
</style>