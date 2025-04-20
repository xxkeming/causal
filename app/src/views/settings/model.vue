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
              :class="{ active: currentProvider?.id === provider.id }"
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
      
      <!-- 右侧内容区域改用新组件 -->
      <n-layout position="absolute" style="left: 220px; right: 0;" class="provider-content">
        <n-scrollbar class="scrollbar-container">
          <n-spin :show="providerStore.loading">
            <n-empty 
              v-if="!currentProvider && !isCreating" 
              description="请选择或创建模型提供商" 
              class="empty-state"
            />
            
            <provider-form
              v-if="currentProvider || isCreating"
              :provider="currentProvider"
              :is-edit="!isCreating"
              :embedded="true"
              @submit="handleProviderSubmit"
              @delete="handleProviderDelete"
              @cancel="cancelCreate"
            />
          </n-spin>
        </n-scrollbar>
      </n-layout>
    </n-layout>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import {
  NLayout, NLayoutSider, NList, NListItem,
  NButton, NEmpty, NSpace, NIcon, NScrollbar,
  NTag, NSpin, useMessage
} from 'naive-ui';
import { AddOutline } from '@vicons/ionicons5';
import { useProviderStore } from '../../stores/providerStore';
import { Provider } from '../../services/typings';
import ProviderForm from './components/ProviderForm.vue';

const route = useRoute();
const message = useMessage();

// 组件状态
const currentProvider = ref<Provider | undefined>(undefined);
const isCreating = ref(false);

// Store
const providerStore = useProviderStore();

// 处理提供商提交
async function handleProviderSubmit(provider: Partial<Provider>) {
  console.log('提交的提供商:', provider);
  try {
    if (isCreating.value) {
      await providerStore.createProvider(provider as Provider);
      message.success('创建提供商成功');
      isCreating.value = false;
    } else if (currentProvider.value) {
      await providerStore.modifyProvider({ ...provider, id: currentProvider.value.id } as Provider);
      message.success('更新提供商成功');
      // 更新当前提供商
      currentProvider.value = { ...provider, id: currentProvider.value.id } as Provider;
    }
  } catch (error) {
    message.error('保存提供商失败');
  }
}

// 处理提供商删除
async function handleProviderDelete(id: number) {
  try {
    const success = await providerStore.removeProvider(id);
    if (success) {
      message.success('提供商已删除');
      // 重置当前选中的提供商
      currentProvider.value = undefined;
      // 选择列表中的第一个提供商
      if (providerStore.providers.length > 0) {
        selectProvider(providerStore.providers[0].id);
      }
    }
  } catch (error) {
    console.error('删除提供商失败:', error);
    message.error('删除提供商失败');
  }
}

// 选择提供商
async function selectProvider(id: number) {
  const provider = await providerStore.fetchProviderById(id);
  if (provider) {
    currentProvider.value = provider;
    console.log('当前提供商:', provider);
    isCreating.value = false;
  }
}

// 创建新提供商
function createNewProvider() {
  currentProvider.value = undefined;
  isCreating.value = true;
}

// 取消创建
function cancelCreate() {
  isCreating.value = false;
  if (providerStore.providers.length > 0) {
    selectProvider(providerStore.providers[0].id);
  }
}

// 获取API类别标签类型
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

// 初始化
onMounted(async () => {
  try {
    await providerStore.fetchAllProviders();
    
    const providerId = route.query.providerId ? Number(route.query.providerId) : null;
    
    if (providerId && providerStore.providers.find(p => p.id === providerId)) {
      selectProvider(providerId);
    } else if (providerStore.providers.length > 0) {
      selectProvider(providerStore.providers[0].id);
    }
  } catch (error) {
    console.error('Failed to fetch providers:', error);
    message.error('加载提供商列表出错');
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
  height: 0;
  opacity: 0;
}

:deep(.n-scrollbar-container) {
  scrollbar-width: none;
  -ms-overflow-style: none;
}

:deep(.n-scrollbar-container::-webkit-scrollbar) {
  width: 0;
  height: 0;
  display: none;
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
