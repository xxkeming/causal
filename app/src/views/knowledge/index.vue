<template>
  <div class="knowledge-view">
    <!-- 移除了左侧分类列表，直接显示内容区域 -->
    <div class="knowledge-content">
      <div class="knowledge-header">
        <div class="knowledge-header-left">
          <n-input
            v-model:value="searchKeyword"
            placeholder="搜索知识库..."
            clearable
            class="knowledge-search"
          >
            <template #prefix>
              <n-icon><SearchOutline /></n-icon>
            </template>
          </n-input>
          
          <!-- 增加类别选择器 -->
          <n-select
            v-model:value="selectedCategory"
            :options="categoryOptions"
            placeholder="全部类别"
            clearable
            class="category-select"
          />
        </div>
        <n-button type="primary" class="create-button" @click="handleCreateKnowledgeBase">
          <template #icon>
            <n-icon><AddOutline /></n-icon>
          </template>
          创建知识库
        </n-button>
      </div>
  
      <n-spin :show="knowledgeStore.loading || categoryStore.loading">
        <div class="knowledge-cards">
          <n-grid :cols="3" :x-gap="16" :y-gap="16" responsive="screen">
            <n-grid-item v-for="kb in filteredKnowledgeBases" :key="kb.id">
              <n-card hoverable class="kb-card" :bordered="false">
                <template #header>
                  <div class="kb-card-header">
                    <div class="kb-card-header-left">
                      <n-avatar round :size="32" :color="getKnowledgeBaseIcon(kb)?.color || '#d9d9d9'">
                        <n-icon size="24" v-if="getKnowledgeBaseIcon(kb)">
                          <component :is="getKnowledgeBaseIcon(kb)?.icon" />
                        </n-icon>
                        <n-icon size="24" v-else><BookOutline /></n-icon>
                      </n-avatar>
                      <div class="kb-card-title">{{ kb.name }}</div>
                    </div>
                    
                    <!-- 将类别标签移到这里 - 卡片右上角，移除round属性使其变为方形 -->
                    <div class="kb-card-category">
                      <n-tag size="small" :type="kb.categoryId === '1' ? 'success' : 'info'">
                        {{ getCategoryName(kb.categoryId) }}
                      </n-tag>
                    </div>
                  </div>
                </template>
            
                <div class="kb-card-description">
                  {{ kb.description }}
                </div>
                
                <!-- 站点地址（仅站点同步类型的知识库显示） -->
                <div v-if="kb.categoryId === '2' && kb.site" class="kb-card-site">
                  <n-icon size="14"><LinkOutline /></n-icon>
                  <a :href="kb.site" target="_blank" class="site-link">{{ formatSiteUrl(kb.site) }}</a>
                </div>
            
                <template #footer>
                  <div class="kb-card-footer">
                    <!-- 日期信息 -->
                    <div class="kb-card-date">
                      更新于: {{ formatDate(kb.updatedAt || kb.createdAt) }}
                    </div>
                
                    <!-- 操作按钮 -->
                    <div class="kb-card-actions">
                      <n-space>
                        <n-button circle tertiary size="small" type="info" title="编辑" @click="handleEditKnowledgeBase(kb)">
                          <template #icon>
                            <n-icon><CreateOutline /></n-icon>
                          </template>
                        </n-button>
                        <n-button circle tertiary size="small" type="error" title="删除" @click="confirmDeleteKnowledgeBase(kb)">
                          <template #icon>
                            <n-icon><TrashBinOutline /></n-icon>
                          </template>
                        </n-button>
                      </n-space>
                    </div>
                  </div>
                </template>
              </n-card>
            </n-grid-item>
          </n-grid>

          <!-- 空状态 -->
          <div v-if="filteredKnowledgeBases.length === 0 && !knowledgeStore.loading" class="empty-state-container">
            <n-empty :description="emptyDescription" />
          </div>
        </div>
      </n-spin>
    </div>
    
    <!-- 删除知识库确认对话框 -->
    <n-modal v-model:show="showDeleteModal" style="width: 400px">
      <n-card
        title="确认删除"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
      >
        <div class="confirm-content">
          您确定要删除"<span class="highlight">{{ knowledgeBaseToDelete?.name }}</span>"知识库吗？
          <div class="confirm-warning">
            此操作不可逆，删除后将无法恢复。
          </div>
        </div>
      
        <template #footer>
          <div style="display: flex; justify-content: flex-end; gap: 12px;">
            <n-button @click="showDeleteModal = false">取消</n-button>
            <n-button type="error" @click="deleteKnowledgeBase">删除</n-button>
          </div>
        </template>
      </n-card>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { 
  NCard, NAvatar, NButton, NIcon, NGrid, NGridItem, 
  NInput, NEmpty, NSpace, NSpin, NModal, NTag, NSelect
} from 'naive-ui';
import { 
  AddOutline, SearchOutline, BookOutline, CreateOutline, 
  TrashBinOutline, LinkOutline
} from '@vicons/ionicons5';
import { useMessage } from 'naive-ui';
import { KnowledgeBase } from '../../services/typings';
import { useKnowledgeBaseStore } from '../../stores/knowledgeBaseStore';
import { useKnowledgeBaseCategoryStore } from '../../stores/knowledgeBaseCategoryStore';
import { useIconStore } from '../../stores/iconStore';

const message = useMessage();

// Store
const knowledgeStore = useKnowledgeBaseStore();
const categoryStore = useKnowledgeBaseCategoryStore();
const iconStore = useIconStore();

// 本地状态
const searchKeyword = ref<string>('');
const selectedCategory = ref<string | null>('all');
const showDeleteModal = ref<boolean>(false);
const knowledgeBaseToDelete = ref<KnowledgeBase | null>(null);

// 计算属性 - 根据本地的搜索关键词和类别过滤知识库
const filteredKnowledgeBases = computed(() => {
  let result = knowledgeStore.knowledgeBases;
  
  // 按类别筛选
  if (selectedCategory.value && selectedCategory.value !== 'all') {
    result = result.filter(kb => kb.categoryId === selectedCategory.value);
  }
  
  // 按关键词搜索
  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase();
    result = result.filter(kb => 
      kb.name.toLowerCase().includes(keyword) || 
      (kb.description && kb.description.toLowerCase().includes(keyword))
    );
  }
  
  return result;
});

// 类别选项
const categoryOptions = computed(() => {
  return [
    { label: '全部类别', value: 'all' },
    ...categoryStore.categories.map(category => ({
      label: category.name,
      value: category.id
    }))
  ];
});

// 空状态描述信息
const emptyDescription = computed(() => {
  if (searchKeyword.value) {
    return '没有找到匹配的知识库';
  }
  
  if (selectedCategory.value && selectedCategory.value !== 'all') {
    return '该分类下暂无知识库';
  }
  
  return '暂无知识库';
});

// 格式化日期
function formatDate(dateString: string): string {
  if (!dateString) return '未知时间';
  const date = new Date(dateString);
  return date.toLocaleDateString('zh-CN');
}

// 格式化站点URL，显示更友好
function formatSiteUrl(url: string): string {
  try {
    const urlObj = new URL(url);
    return urlObj.hostname;
  } catch (e) {
    return url;
  }
}

// 获取知识库图标
function getKnowledgeBaseIcon(kb: KnowledgeBase) {
  if (kb.iconId) {
    return iconStore.getIconById(kb.iconId);
  }
  return null;
}

// 获取类别名称
function getCategoryName(categoryId: string): string {
  const category = categoryStore.categories.find(cat => cat.id === categoryId);
  return category ? category.name : '未知类别';
}

// 创建知识库
function handleCreateKnowledgeBase() {
  message.info('创建知识库功能开发中');
  // 实际项目中可以导航到创建页面
  // router.push('/knowledge/create');
}

// 编辑知识库
function handleEditKnowledgeBase(_kb: KnowledgeBase) {
  message.info('编辑知识库功能开发中');
  // 实际项目中可以导航到编辑页面
  // router.push(`/knowledge/edit/${kb.id}`);
}

// 确认删除知识库
function confirmDeleteKnowledgeBase(kb: KnowledgeBase) {
  knowledgeBaseToDelete.value = kb;
  showDeleteModal.value = true;
}

// 删除知识库
async function deleteKnowledgeBase() {
  if (!knowledgeBaseToDelete.value) return;
  
  try {
    const success = await knowledgeStore.removeKnowledgeBase(knowledgeBaseToDelete.value.id);
    if (success) {
      message.success('知识库已删除');
    } else {
      message.error('删除知识库失败');
    }
  } catch (error) {
    message.error('删除知识库时发生错误');
  } finally {
    showDeleteModal.value = false;
    knowledgeBaseToDelete.value = null;
  }
}

// 监听需要刷新状态
watch(() => knowledgeStore.needRefresh, (needRefresh) => {
  if (needRefresh) {
    console.log("需要刷新知识库列表，正在重新加载...");
    knowledgeStore.fetchAllKnowledgeBases();
  }
}, { immediate: true }); 

// 在组件挂载时加载数据
onMounted(async () => {
  console.log('Knowledge view mounted');
  
  // 加载所有知识库数据
  knowledgeStore.fetchAllKnowledgeBases();
});
</script>

<style scoped>
.knowledge-view {
  height: 100%;
  width: 100%;
}

.knowledge-content {
  padding: 16px;
  overflow: auto;
  height: 100%;
  width: 100%;
  margin: 0 auto;
}

.knowledge-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.knowledge-header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.knowledge-search {
  width: 240px;
}

.knowledge-cards {
  margin-top: 20px;
}

.kb-card {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
  min-height: 70px; /* 大幅减少最小高度，原来是200px */
}

.kb-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
}

.kb-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.kb-card-header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.kb-card-title {
  font-size: 16px;
  font-weight: 500;
}

.kb-card-category {
  flex-shrink: 0;
}

.kb-card-description {
  color: var(--text-color-secondary);
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 1; /* 从2行减少到1行 */
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
  height: 1.5em; /* 减小高度，原来是3em */
  margin: 4px 0; /* 减少上下边距，原来是8px */
  font-size: 12px; /* 减小字体大小 */
}

/* 针对站点同步类别的特殊样式调整 */
.kb-card-category + .kb-card-site {
  margin-top: 0; /* 如果类别标签和站点链接相邻，减少间距 */
}

.kb-card-site {
  display: flex;
  align-items: center;
  gap: 4px; /* 减少间距，原来是6px */
  font-size: 12px; /* 减小字体大小，原来是13px */
  color: var(--text-color-secondary);
  margin-top: 4px; /* 减少上边距，原来是8px */
}

.site-link {
  color: var(--primary-color);
  text-decoration: none;
}

.site-link:hover {
  text-decoration: underline;
}

.kb-card-footer {
  display: flex;
  flex-direction: row; /* 改为水平布局，节省空间 */
  justify-content: space-between;
  align-items: center;
  gap: 4px; /* 减少间距，原来是8px */
  margin-top: 4px; /* 添加上边距 */
}

.kb-card-date {
  font-size: 11px; /* 进一步减小字体大小，原来是12px */
  color: var(--text-color-secondary);
}

.kb-card-actions {
  display: flex;
  justify-content: flex-end;
}

:deep(.n-list-item) {
  padding: 0 !important;
  border-bottom: none !important;
}

:deep(.n-list) {
  --n-divider-color: transparent;
  --n-border-color: transparent;
}

:deep(.n-button) {
  font-weight: 500;
}

:deep(.n-modal .n-card) {
  border-radius: 8px !important;
  overflow: hidden;
}

:deep(.n-card-header) {
  padding-bottom: 4px; /* 减少内边距，原来是8px */
}

:deep(.n-card-footer) {
  padding-top: 4px; /* 减少内边距，原来是8px */
  padding-bottom: 4px; /* 添加底部内边距减少 */
}

/* 添加全局卡片内边距调整 */
:deep(.n-card-content) {
  padding-top: 0;
  padding-bottom: 0;
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

.empty-state-container {
  display: flex;
  align-items: center;
  justify-content: center;
  height: calc(100vh - 200px);
  width: 100%;
}

@media (max-width: 768px) {
  .knowledge-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
  }
  
  .knowledge-header-left {
    width: 100%;
  }
  
  .knowledge-search {
    width: 100%;
  }
}

/* 添加类别选择器样式 */
.category-select {
  width: 180px;
}

/* 确保顶部搜索区域的间距和布局正确 */
.knowledge-header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

@media (max-width: 768px) {
  .knowledge-header-left {
    width: 100%;
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
  
  .knowledge-search,
  .category-select {
    width: 100%;
  }
}

</style>
