<template>
  <div class="provider-settings">
    <n-layout has-sider position="absolute">
      <!-- 左侧提供商列表 - 减小宽度 -->
      <n-layout-sider
        :width="220"
        bordered
        :show-trigger="false"
        position="absolute"
        class="provider-sider"
        :native-scrollbar="false"
      >
        <n-scrollbar>
          <div class="sider-header">
            <n-space justify="space-between" align="center">
              <h3>模型提供商</h3>
              <n-button tertiary circle size="small" @click="createNewProvider">
                <template #icon>
                  <n-icon><AddOutline /></n-icon>
                </template>
              </n-button>
            </n-space>
          </div>
          
          <n-list class="provider-list" :show-divider="false">
            <n-empty v-if="providerStore.providers.length === 0" size="small" description="暂无提供商" />
            
            <n-list-item
              v-for="provider in providerStore.providers"
              :key="provider.id"
              class="provider-item"
              :class="{ active: currentProviderId === provider.id }"
              @click="selectProvider(provider.id)"
            >
              <div class="provider-item-content">
                <div class="provider-name">{{ provider.name }}</div>
                <n-tag size="small" :type="getApiCategoryTagType(provider.apiCategory)">
                  {{ provider.apiCategory }}
                </n-tag>
              </div>
            </n-list-item>
          </n-list>
        </n-scrollbar>
      </n-layout-sider>
      
      <!-- 右侧提供商详情 - 修改为n-scrollbar包裹 -->
      <n-layout position="absolute" style="left: 220px; right: 0;" class="provider-content">
        <n-scrollbar class="scrollbar-container">
          <n-spin :show="loadingDetails">
            <n-empty 
              v-if="!currentProvider && !isCreating" 
              description="请选择或创建模型提供商" 
              class="empty-state"
            />
            
            <!-- 提供商详情表单 -->
            <div v-if="currentProvider || isCreating" class="provider-details">
              <div class="details-header">
                <h2>{{ isCreating ? '新建模型提供商' : '编辑模型提供商' }}</h2>
                <div class="details-actions">
                  <n-button
                    v-if="isCreating"
                    @click="cancelCreate"
                    class="cancel-btn"
                  >
                    取消
                  </n-button>
                  <n-button
                    type="primary"
                    @click="saveProvider"
                    :loading="saving"
                    :disabled="saving"
                  >
                    {{ isCreating ? '创建' : '保存' }}
                  </n-button>
                  <n-button v-if="!isCreating" tertiary type="error" @click="confirmDeleteProvider">
                    删除
                  </n-button>
                </div>
              </div>
              
              <n-divider />
              
              <n-form
                ref="formRef"
                :model="formModel"
                :rules="formRules"
                label-placement="left"
                label-width="80px"
                require-mark-placement="right-hanging"
                class="provider-form"
              >
                <!-- 将名称和API类别放在一行 -->
                <div class="form-row">
                  <n-form-item label="名称" path="name" required class="form-item-name">
                    <n-input v-model:value="formModel.name" placeholder="请输入提供商名称" />
                  </n-form-item>
                  
                  <n-form-item label="接口类型" path="apiCategory" required class="form-item-category">
                    <n-select
                      v-model:value="formModel.apiCategory"
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
                    <n-input v-model:value="formModel.url" placeholder="请输入 API 地址" />
                    <!-- 将预览放在输入框的容器里，而不是直接放在表单项内 -->
                    <div class="api-path-preview" v-if="formModel.url">
                      <span class="api-path-value">{{ completionApiPath }}</span>
                    </div>
                  </div>
                </n-form-item>

                <n-form-item label="密钥" path="apiKey">
                  <n-input
                    v-model:value="formModel.apiKey"
                    type="password"
                    show-password-on="click"
                    placeholder="请输入 API 密钥"
                  />
                </n-form-item>
                
                <n-divider>模型列表</n-divider>
                
                <div class="models-section">
                  <div v-if="formModel.models && formModel.models.length > 0" class="models-list">
                    <div
                      v-for="(model, index) in formModel.models"
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
          </n-spin>
        </n-scrollbar>
      </n-layout>
    </n-layout>
    
    <!-- 删除确认对话框 -->
    <n-modal
      v-model:show="showDeleteConfirm"
      preset="dialog"
      title="确认删除"
      content="确定要删除这个模型提供商吗？此操作不可恢复。"
      positive-text="删除"
      negative-text="取消"
      @positive-click="deleteProvider"
      @negative-click="showDeleteConfirm = false"
    >
      <template #icon>
        <n-icon :component="WarningOutline" />
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';
import {
  NLayout,
  NLayoutSider,
  NList,
  NListItem,
  NInput,
  NButton,
  NEmpty,
  NSelect,
  NForm,
  NFormItem,
  NSpace,
  NIcon,
  NScrollbar,
  NDivider,
  NTag,
  NSpin,
  NModal,
  NPopover,
  FormRules,
  useMessage
} from 'naive-ui';
import {
  AddOutline,
  TrashOutline,
  WarningOutline,
  HelpCircleOutline // 添加帮助图标
} from '@vicons/ionicons5';
import { useProviderStore, ProviderApiCategory } from '../../stores/providerStore';
import { Model } from '../../services/typings';

// 组件状态
const currentProviderId = ref<number | null>(null);
const isCreating = ref(false);
const loadingDetails = ref(false);
const saving = ref(false);
const showDeleteConfirm = ref(false);
const formRef = ref(null);

// 获取消息 API
const message = useMessage();

// 获取提供商 Store
const providerStore = useProviderStore();

// 模型表单
const defaultFormModel = {
  name: '',
  apiCategory: '',
  url: '',
  apiKey: '',
  models: [] as Model[] // 明确标注类型为ProviderModel[]
};

// 创建响应式表单模型
const formModel = reactive<{
  name: string;
  apiCategory: string;
  url: string;
  apiKey: string;
  models: Model[]; // 明确标注类型
}>({...defaultFormModel});

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
  if (!formModel.url) return '';
  
  // 移除末尾的斜杠（如果有）
  let baseUrl = formModel.url.trim();
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
  { label: '向量', value: '向量' },
  { label: '图片', value: '图片' }
];

// 获取当前选中的提供商
const currentProvider = computed(() => {
  if (!currentProviderId.value) return null;
  return providerStore.providers.find(p => p.id === currentProviderId.value) || null;
});

// 根据 API 类别获取标签类型
function getApiCategoryTagType(category: string): 'success' | 'info' | 'warning' | 'error' {
  const categoryMap: Record<string, 'success' | 'info' | 'warning' | 'error'> = {
    'OpenAI': 'success',
    'Gemini': 'info',
    'Anthropic': 'warning',
    'Xai': 'info',
    'Ollama': 'success',
    'DeepSeek': 'info'
  };
  return categoryMap[category] || 'default';
}

// 选择提供商
async function selectProvider(id: number) {
  if (saving.value) return; // 如果正在保存，不允许切换
  
  currentProviderId.value = id;
  isCreating.value = false;
  loadingDetails.value = true;
  
  try {
    const provider = await providerStore.fetchProviderById(id);
    if (provider) {
      // 复制提供商数据到表单
      Object.assign(formModel, {
        name: provider.name,
        apiCategory: provider.apiCategory,
        url: provider.url || '',
        apiKey: provider.apiKey || '',
        models: provider.models ? [...provider.models] : []
      });
    } else {
      message.error('获取提供商详情失败');
    }
  } catch (error) {
    console.error('Failed to fetch provider details:', error);
    message.error('加载提供商详情出错');
  } finally {
    loadingDetails.value = false;
  }
}

// 创建新的提供商
function createNewProvider() {
  currentProviderId.value = null;
  isCreating.value = true;
  
  // 重置表单
  Object.assign(formModel, {
    name: '',
    apiCategory: '',
    url: '',
    apiKey: '',
    models: []
  });
}

// 添加新模型 - 实现正确的函数，确保类型正确
function addNewModel() {
  if (!formModel.models) {
    formModel.models = [];
  }
  formModel.models.push({ 
    name: '', 
    tags: [] as string[] // 明确标注tags类型为字符串数组
  });
}

// 取消创建 - 修复为正确的取消功能
function cancelCreate() {
  isCreating.value = false;
  if (providerStore.providers.length > 0) {
    selectProvider(providerStore.providers[0].id);
  }
}

// 移除模型
function removeModel(index: number) {
  if (formModel.models) {
    formModel.models.splice(index, 1);
  }
}

// 确认删除提供商
function confirmDeleteProvider() {
  showDeleteConfirm.value = true;
}

// 删除提供商
async function deleteProvider() {
  if (!currentProviderId.value) return;
  
  saving.value = true;
  try {
    const success = await providerStore.removeProvider(currentProviderId.value);
    if (success) {
      message.success('提供商已删除');
      
      // 更新选中的提供商
      if (providerStore.providers.length > 0) {
        selectProvider(providerStore.providers[0].id);
      } else {
        currentProviderId.value = null;
        isCreating.value = false;
      }
    } else {
      message.error('删除提供商失败');
    }
  } catch (error) {
    console.error('Failed to delete provider:', error);
    message.error('删除提供商时出错');
  } finally {
    saving.value = false;
  }
}

// 添加提交尝试状态变量
const submitAttempted = ref(false);

// 验证表单中的模型列表，修复类型检查
function validateModelList(): boolean {
  if (!formModel.models || formModel.models.length === 0) {
    return true; // 如果没有模型则算通过验证
  }
  
  // 检查每一个模型的名称和标签
  for (const model of formModel.models) {
    if (!model.name || !model.tags || model.tags.length === 0) {
      return false;
    }
  }
  
  return true;
}

// 保存提供商 - 修改此函数添加模型验证
async function saveProvider() {
  if (!formRef.value) return;
  
  try {
    // 标记已尝试提交，用于显示验证错误
    submitAttempted.value = true;
    
    // 验证模型列表
    if (!validateModelList()) {
      message.error('请完善所有模型的名称和标签');
      return;
    }
    
    // @ts-ignore
    await formRef.value.validate();
    
    saving.value = true;
    
    const providerData = {
      name: formModel.name,
      apiCategory: formModel.apiCategory,
      url: formModel.url,
      apiKey: formModel.apiKey,
      models: formModel.models
    };
    
    if (isCreating.value) {
      // 创建新提供商
      await providerStore.createProvider(providerData);
      message.success('创建提供商成功');
      
      // 选择新创建的提供商
      isCreating.value = false;
    } else if (currentProviderId.value) {
      // 更新现有提供商
      await providerStore.modifyProvider({
        id: currentProviderId.value,
        ...providerData
      });
      message.success('更新提供商成功');
    }
  } catch (error) {
    console.error('Form validation or save failed:', error);
    if (Array.isArray(error)) {
      message.error(error[0]?.message || '表单验证失败');
    } else {
      message.error('保存提供商时出错');
    }
  } finally {
    saving.value = false;
  }
}

// 初始化
onMounted(async () => {
  loadingDetails.value = true;
  try {
    await providerStore.fetchAllProviders();
    if (providerStore.providers.length > 0) {
      selectProvider(providerStore.providers[0].id);
    }
  } catch (error) {
    console.error('Failed to fetch providers:', error);
    message.error('加载提供商列表出错');
  } finally {
    loadingDetails.value = false;
  }
});
</script>

<style scoped>
.provider-settings {
  height: 100%;
  width: 100%;
  position: relative;
}

.provider-sider {
  top: 0;
  bottom: 0;
  left: 0;
  overflow: hidden;
}

.sider-header {
  padding: 16px;
}

.sider-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 500;
}

.provider-list {
  padding: 0;
  --n-divider-color: transparent;
  --n-border-color: transparent;
}

.provider-item {
  cursor: pointer;
  transition: all 0.2s;
  margin: 2px 0;
}

.provider-item-content {
  padding: 12px 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.provider-name {
  font-weight: 500;
}

.provider-item:hover {
  background-color: rgba(128, 128, 128, 0.08);
}

.provider-item.active {
  background-color: rgba(24, 160, 88, 0.12);
}

.provider-item.active .provider-name {
  color: var(--primary-color);
  font-weight: 500;
}

.provider-content {
  top: 18px;
  bottom: 0;
  height: 100%; 
  position: absolute;
  left: 0;
  right: 0;
  overflow: hidden; /* 改为hidden，由n-scrollbar处理滚动 */
}

.scrollbar-container {
  height: 100%;
  scrollbar-width: none !important; /* Firefox */
  -ms-overflow-style: none !important; /* IE and Edge */
}

/* 确保内部元素可以正确显示 */
.provider-details {
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
