<template>
  <div class="tools-view">
    <n-layout has-sider position="absolute">
      <!-- 左侧分类列表 -->
      <n-layout-sider
        :width="120"
        bordered
        :show-trigger="false"
        position="absolute"
        class="category-sider"
        :native-scrollbar="false"
      >
        <n-scrollbar>
          <n-list class="category-list" :show-divider="false">
            <!-- 所有工具选项 -->
            <n-list-item 
              class="category-item"
              :class="{ active: selectedCategory === 'all' }"
              @click="handleSelectCategory('all')"
            >
              <div class="category-content">
                所有工具
              </div>
            </n-list-item>
            
            <!-- 分类列表 -->
            <n-list-item 
              v-for="category in categoryStore.categories" 
              :key="category.id"
              class="category-item"
              :class="{ active: selectedCategory === category.id }"
            >
              <div class="category-content category-item-wrapper">
                <span class="category-name" @click="handleSelectCategory(category.id)">
                  {{ category.name }}
                </span>
                <n-button 
                  circle 
                  size="tiny" 
                  quaternary 
                  class="category-delete-btn"
                  @click.stop="confirmDeleteCategory(category)"
                >
                  <template #icon>
                    <n-icon size="14"><CloseOutline /></n-icon>
                  </template>
                </n-button>
              </div>
            </n-list-item>
            
            <!-- 添加类别按钮 -->
            <n-list-item class="add-category-item" @click="showAddCategoryModal = true">
              <div class="category-content add-category">
                <n-icon size="18"><AddOutline /></n-icon>
              </div>
            </n-list-item>
          </n-list>
        </n-scrollbar>
      </n-layout-sider>
      
      <!-- 右侧内容区域 -->
      <n-layout position="absolute" style="left: 120px; right: 0;" class="tools-content">
        <div class="tools-header">
          <div class="tools-header-left">
            <n-input
              v-model:value="searchKeyword"
              placeholder="搜索工具..."
              clearable
              class="tools-search"
            >
              <template #prefix>
                <n-icon><SearchOutline /></n-icon>
              </template>
            </n-input>
          </div>
          <n-button type="primary" class="create-button" @click="showToolFormModal">
            <template #icon>
              <n-icon><AddOutline /></n-icon>
            </template>
            创建工具
          </n-button>
        </div>
    
        <n-spin :show="toolStore.loading || categoryStore.loading">
          <div class="tools-cards">
            <n-grid :cols="4" :x-gap="16" :y-gap="16" responsive="screen">
              <n-grid-item v-for="tool in filteredTools" :key="tool.id">
                <n-card hoverable class="tool-card" :bordered="false">
                  <template #header>
                    <div class="tool-card-header">
                      <n-avatar round :size="32" :color="getToolIcon(tool)?.color || '#d9d9d9'">
                        <n-icon size="24" v-if="getToolIcon(tool)">
                          <component :is="getToolIcon(tool)?.icon" />
                        </n-icon>
                        <n-icon size="24" v-else><BuildOutline /></n-icon>
                      </n-avatar>
                      <div class="tool-card-title">{{ tool.name }}</div>
                    </div>
                  </template>
              
                  <div class="tool-card-description">
                    {{ tool.description }}
                  </div>
              
                  <template #footer>
                    <div class="tool-card-footer">
                      <!-- 日期信息 -->
                      <div class="tool-card-date">
                        更新于: {{ formatDate(tool.updatedAt || tool.createdAt) }}
                      </div>
                  
                      <!-- 操作按钮 -->
                      <div class="tool-card-actions">
                        <n-space>
                          <n-button circle tertiary size="small" type="info" title="编辑" @click="handleEditTool(tool)">
                            <template #icon>
                              <n-icon><CreateOutline /></n-icon>
                            </template>
                          </n-button>
                          <n-button circle tertiary size="small" type="error" title="删除" @click="confirmDeleteTool(tool)">
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
            <div v-if="filteredTools.length === 0 && !toolStore.loading" class="empty-state-container">
              <n-empty :description="emptyDescription" />
            </div>
          </div>
        </n-spin>
      </n-layout>
    </n-layout>
    
    <!-- 添加分类对话框 -->
    <tool-category-add-modal
      v-model:visible="showAddCategoryModal"
      @submit="addNewCategory"
    />
    
    <!-- 删除分类确认对话框 -->
    <tool-category-delete-modal
      v-model:visible="showDeleteCategoryModal"
      :category="categoryToDelete"
      @confirm="deleteCategory"
    />
    
    <!-- 删除工具确认对话框 - 替换为组件 -->
    <tool-delete-modal
      v-model:visible="showDeleteToolModal"
      :tool="toolToDelete"
      @confirm="deleteTool"
      @cancel="showDeleteToolModal = false"
    />
    
    <!-- 工具表单模态框 -->
    <n-modal v-model:show="showToolModal" style="width: 800px">
      <n-card
        :title="isEditingTool ? '编辑工具' : '创建工具'"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
      >
        <n-empty description="工具表单待实现">
          <template #extra>
            <n-button @click="showToolModal = false">关闭</n-button>
          </template>
        </n-empty>
      
        <template #footer>
          <div style="display: flex; justify-content: flex-end; gap: 12px;">
            <n-button @click="showToolModal = false">取消</n-button>
            <n-button type="primary" :loading="submittingTool">
              {{ isEditingTool ? '保存' : '创建' }}
            </n-button>
          </div>
        </template>
      </n-card>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useRouter } from 'vue-router'
import { 
  NLayout, NLayoutSider, NList, NListItem, NCard, NAvatar, 
  NButton, NIcon, NGrid, NGridItem, NInput, NScrollbar,
  NEmpty, NSpace, NSpin, NModal
} from 'naive-ui';
import { 
  AddOutline, SearchOutline, BuildOutline, CreateOutline, 
  TrashBinOutline, CloseOutline
} from '@vicons/ionicons5';
import { useMessage } from 'naive-ui';
import { ToolCategory, Tool } from '../../services/typings';
import { useToolStore } from '../../stores/toolStore';
import { useToolCategoryStore } from '../../stores/toolCategoryStore';
import { useIconStore } from '../../stores/iconStore';
// 导入拆分的组件
import ToolCategoryAddModal from './components/ToolCategoryAddModal.vue';
import ToolCategoryDeleteModal from './components/ToolCategoryDeleteModal.vue';
import ToolDeleteModal from './components/ToolDeleteModal.vue'; // 导入新组件

const router = useRouter()

const message = useMessage();

// Store
const toolStore = useToolStore();
const categoryStore = useToolCategoryStore();
const iconStore = useIconStore();

// 本地状态
const searchKeyword = ref<string>('');
const selectedCategory = ref<string | number>('all');
const showAddCategoryModal = ref<boolean>(false);
const showDeleteCategoryModal = ref<boolean>(false);
const showDeleteToolModal = ref<boolean>(false);
const showToolModal = ref<boolean>(false);
const isEditingTool = ref<boolean>(false);
const categoryToDelete = ref<ToolCategory | null>(null);
const toolToDelete = ref<Tool | null>(null);
const submittingTool = ref<boolean>(false);

// 计算属性 - 同时处理分类筛选和关键词搜索
const filteredTools = computed(() => {
  let result = toolStore.tools;
  
  // 先按分类筛选
  if (selectedCategory.value !== 'all') {
    result = result.filter(tool => tool.categoryId === selectedCategory.value);
  }
  
  // 再按关键词搜索
  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase();
    result = result.filter(tool => 
      tool.name.toLowerCase().includes(keyword) || 
      (tool.description && tool.description.toLowerCase().includes(keyword))
    );
  }
  
  return result;
});

// 空状态描述
const emptyDescription = computed(() => {
  if (searchKeyword.value) {
    return '没有找到匹配的工具';
  }
  return selectedCategory.value === 'all' ? '暂无工具' : '该分类下暂无工具';
});

// 处理分类选择 - 简化为仅更新本地状态
function handleSelectCategory(categoryId: number | 'all') {
  selectedCategory.value = categoryId;
  // 不再调用 fetchToolsByCategory，所有筛选在客户端进行
}

// 格式化日期
function formatDate(dateString: string): string {
  if (!dateString) return '未知时间';
  const date = new Date(dateString);
  return date.toLocaleDateString('zh-CN');
}

// 确认删除分类
function confirmDeleteCategory(category: ToolCategory) {
  categoryToDelete.value = category;
  showDeleteCategoryModal.value = true;
}

// 删除分类
async function deleteCategory() {
  if (!categoryToDelete.value) return;
  
  try {
    const success = await categoryStore.removeCategory(categoryToDelete.value.id);
    if (success) {
      if (selectedCategory.value === categoryToDelete.value.id) {
        selectedCategory.value = 'all';
      }
      message.success('类别已删除');
    } else {
      message.error('删除类别失败');
    }
  } catch (error) {
    message.error('删除类别时发生错误');
  }
}

// 添加新类别
async function addNewCategory(name: string) {
  if (!name || !name.trim()) {
    message.error('请输入有效的分类名称');
    return; // 提前返回，不关闭模态框
  }
  
  try {
    // 先检查是否已存在同名类别
    const existingCategory = categoryStore.categories.find(cat => 
      cat.name.toLowerCase() === name.trim().toLowerCase()
    );
    
    if (existingCategory) {
      message.warning('类别名称已存在');
      return; // 提前返回，不关闭模态框
    }
    
    // 添加类别
    await categoryStore.addCategory(name.trim());
    message.success('类别已添加');
    
    // 成功后关闭模态框
    showAddCategoryModal.value = false;
  } catch (error) {
    message.error('添加类别失败');
    console.error(error);
    // 发生错误时保持模态框打开
  }
}

// 显示工具表单模态框
function showToolFormModal() {
  const query = selectedCategory.value !== 'all' ? { categoryId: selectedCategory.value } : {};
  router.push({ path: '/tools/edit', query });
}

// 确认删除工具
function confirmDeleteTool(tool: Tool) {
  toolToDelete.value = tool;
  showDeleteToolModal.value = true;
}

// 删除工具
async function deleteTool() {
  if (!toolToDelete.value) return;
  
  try {
    const success = await toolStore.removeTool(toolToDelete.value.id);
    if (success) {
      message.success('工具已删除');
    } else {
      message.error('删除工具失败');
    }
  } catch (error) {
    message.error('删除工具时发生错误');
  } finally {
    showDeleteToolModal.value = false;
    toolToDelete.value = null;
  }
}

// 编辑工具
function handleEditTool(tool: Tool) {
  router.push(`/tools/edit/${tool.id}`);
}

// 获取工具图标
function getToolIcon(tool: Tool) {
  if (tool.iconId) {
    return iconStore.getIconById(tool.iconId);
  }
  return null;
}

// 监听需要刷新状态，如果需要则重新加载数据
watch(() => toolStore.needRefresh, (needRefresh) => {
  if (needRefresh) {
    console.log("需要刷新工具列表，正在重新加载...");
    // 简化为只调用 fetchAllTools
    toolStore.fetchAllTools();
  }
}, { immediate: true }); 

// 在组件挂载时加载数据
onMounted(async () => {
  console.log('Tools view mounted');
  await categoryStore.fetchCategories();
  
  // 只获取所有工具数据，不再按分类获取
  await toolStore.fetchAllTools();
});
</script>

<style scoped>
.tools-view {
  height: 100%;
  width: 100%;
}

.category-sider {
  top: 0;
  bottom: 0;
  left: 0;
  overflow: hidden;
}

.category-list {
  padding: 8px 0;
  --n-divider-color: transparent;
  --n-border-color: transparent;
}

.category-item {
  cursor: pointer;
  transition: all 0.2s;
  margin: 2px 0;
}

.category-content {
  padding: 8px 16px;
  margin: 0 4px;
  font-size: 14px;
  color: var(--text-color);
  position: relative;
}

.category-item:hover .category-content {
  background-color: rgba(128, 128, 128, 0.08);
}

.category-item.active .category-content {
  background-color: rgba(24, 160, 88, 0.12);
  color: var(--primary-color);
  font-weight: 500;
}

.tools-content {
  padding: 16px;
  overflow: auto;
  top: 0;
  bottom: 0;
}

.tools-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.tools-header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.tools-search {
  width: 240px;
}

.tools-cards {
  margin-top: 20px;
}

.tool-card {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
  min-height: 200px;
}

.tool-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
}

.tool-card-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.tool-card-title {
  font-size: 16px;
  font-weight: 500;
}

.tool-card-description {
  color: var(--text-color-secondary);
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
  height: 3em; /* 约等于两行文字的高度 */
}

.tool-card-footer {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.tool-card-date {
  font-size: 12px;
  color: var(--text-color-secondary);
}

.tool-card-actions {
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
  padding-bottom: 8px;
}

:deep(.n-card-footer) {
  padding-top: 8px;
}

.category-item-wrapper {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.category-name {
  flex-grow: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 60px;
  cursor: pointer;
}

.category-delete-btn {
  flex-shrink: 0;
  margin-left: 4px;
  visibility: hidden;
}

.category-item:hover .category-delete-btn {
  visibility: visible;
}

.add-category {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--primary-color);
  font-weight: 500;
}

.add-category:hover {
  background-color: rgba(24, 160, 88, 0.08);
}

.empty-state-container {
  display: flex;
  align-items: center;
  justify-content: center;
  height: calc(100vh - 200px); /* 设置适当的高度使内容垂直居中 */
  width: 100%;
}

@media (max-width: 768px) {
  .tools-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
  }
  
  .tools-header-left {
    width: 100%;
  }
  
  .tools-search {
    width: 100%;
  }
}

</style>
