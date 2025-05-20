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
              :class="{ active: selectedCategory === 0 }"
              @click="handleSelectCategory(0)"
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
              @click="handleSelectCategory(category.id)"
            >
              <div class="category-content category-item-wrapper">
                <n-tooltip placement="right" trigger="hover">
                  <template #trigger>
                    <span class="category-name">
                      {{ category.name }}
                    </span>
                  </template>
                  {{ category.name }}
                </n-tooltip>
                <div class="category-actions">
                  <!-- 添加编辑按钮 -->
                  <n-button 
                    circle 
                    size="tiny" 
                    quaternary 
                    class="category-action-btn"
                    @click.stop="openEditCategory(category)"
                  >
                    <template #icon>
                      <n-icon size="14"><CreateOutline /></n-icon>
                    </template>
                  </n-button>
                  <n-button 
                    circle 
                    size="tiny" 
                    quaternary 
                    class="category-action-btn"
                    @click.stop="confirmDeleteCategory(category)"
                  >
                    <template #icon>
                      <n-icon size="14"><CloseOutline /></n-icon>
                    </template>
                  </n-button>
                </div>
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
          <n-dropdown 
            trigger="hover"
            :options="createOptions" 
            @select="showToolFormModal"
            placement="bottom-start"
          >
            <n-button type="primary" class="create-button">
              <template #icon>
                <n-icon><AddOutline /></n-icon>
              </template>
              创建工具
            </n-button>
          </n-dropdown>
        </div>
    
        <n-spin :show="globalStore.isLoading">
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
                      <!-- 添加工具类型标签 -->
                      <div class="tool-type-tag">
                        <n-tag size="small" :type="getTypeTagColor(tool.data.type)">
                          {{ getTypeDisplay(tool.data.type) }}
                        </n-tag>
                      </div>
                    </div>
                  </template>
              
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
            <div v-if="filteredTools.length === 0 && !globalStore.isLoading" class="empty-state-container">
              <n-empty :description="emptyDescription" />
            </div>
          </div>
        </n-spin>
      </n-layout>
    </n-layout>
    
    <!-- 添加/编辑类别对话框 -->
    <tool-category-form-modal
      v-model:visible="showAddCategoryModal"
      :category="categoryToEditForModal"
      @submit="addNewCategory"
      @update="updateCategory"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, h, type Component } from 'vue';
import { useRouter } from 'vue-router'
import { 
  NLayout, NLayoutSider, NList, NListItem, NCard, NAvatar, 
  NButton, NIcon, NGrid, NGridItem, NInput, NScrollbar,
  NEmpty, NSpace, NSpin, NTag, NTooltip, NDropdown,
  useDialog, type DropdownOption 
} from 'naive-ui';
import { 
  AddOutline, SearchOutline, BuildOutline, CreateOutline, 
  TrashBinOutline, CloseOutline, ServerOutline, ExtensionPuzzleOutline
} from '@vicons/ionicons5';
import { useMessage } from 'naive-ui';
import { ToolCategory, Tool } from '../../services/typings';
import { useGlobalStore } from '../../stores/globalStore';
import { useToolStore } from '../../stores/toolStore';
import { useToolCategoryStore } from '../../stores/toolCategoryStore';
import { useIconStore } from '../../stores/iconStore';
// 导入拆分的组件
import ToolCategoryFormModal from './components/ToolCategoryFormModal.vue';

const router = useRouter()

const message = useMessage();
const dialog = useDialog(); // 添加 dialog

// Store
const globalStore = useGlobalStore();
const toolStore = useToolStore();
const categoryStore = useToolCategoryStore();
const iconStore = useIconStore();

// 本地状态
const searchKeyword = ref<string>('');
const selectedCategory = ref<number>(0);
const showAddCategoryModal = ref<boolean>(false);
const categoryToEdit = ref<ToolCategory | null>(null);

// Computed property to handle null-to-undefined conversion
const categoryToEditForModal = computed(() => categoryToEdit.value || undefined);

// 计算属性 - 同时处理分类筛选和关键词搜索
const filteredTools = computed(() => {
  // 拷贝toolStore.tools
  let result = toolStore.tools;

  // 先按分类筛选
  if (selectedCategory.value !== 0) {
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

  console.log('Filtered tools:', result);
  return result;
});

// 空状态描述
const emptyDescription = computed(() => {
  if (searchKeyword.value) {
    return '没有找到匹配的工具';
  }
  return selectedCategory.value === 0 ? '暂无工具' : '该分类下暂无工具';
});

// 处理分类选择 - 简化为仅更新本地状态
function handleSelectCategory(categoryId: number) {
  selectedCategory.value = categoryId;
}

// 格式化日期
function formatDate(dateString: string): string {
  if (!dateString) return '未知时间';
  const date = new Date(dateString);
  return date.toLocaleDateString('zh-CN');
}

// 修改确认删除分类函数
async function confirmDeleteCategory(category: ToolCategory) {
  try {
    globalStore.setLoadingState(true);

    await dialog.warning({
      title: '确认删除',
      content: `是否删除类别"${category.name}"？该类别下的所有工具也将被删除，此操作不可恢复。`,
      positiveText: '确认',
      negativeText: '取消',
      style: {
        position: 'relative',
        marginTop: '20vh'
      },
      onPositiveClick: async () => {
        const success = await categoryStore.removeCategory(category.id);
        if (success) {
          await toolStore.removeToolByCategory(category.id);
          
          // 如果当前选中的是被删除的类别，则切换到"所有工具"
          if (selectedCategory.value === category.id) {
            selectedCategory.value = 0;
          }
          message.success('类别已删除');
        } else {
          throw new Error('删除类别失败');
        }
      }
    });
  } catch (error) {
    message.error('删除类别失败');
  } finally {
    globalStore.setLoadingState(false);
  }
}

// 添加新类别
async function addNewCategory(name: string) {
  if (!name || !name.trim()) {
    message.error('请输入有效的分类名称');
    return; // 提前返回，不关闭模态框
  }
  
  try {
    globalStore.setLoadingState(true);

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
  } finally {
    globalStore.setLoadingState(false);
  }
}

// 添加工具类型菜单选项
const createOptions: DropdownOption[] = [
  {
    label: 'JavaScript',
    key: 'js',
    icon: renderIcon(BuildOutline),
    disabled: true,
  },
  {
    label: 'MCP-SSE',
    key: 'mcp-sse',
    icon: renderIcon(ServerOutline),
    disabled: false
  },
  {
    label: 'MCP-STDIO',
    key: 'mcp-io',
    icon: renderIcon(ExtensionPuzzleOutline),
    disabled: false
  }
];

// 添加图标渲染函数
function renderIcon(icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) });
}

// 修改显示工具表单模态框函数
function showToolFormModal(key: string) {
  const query = selectedCategory.value !== 0 ? 
    { categoryId: selectedCategory.value, type: key } : 
    { type: key };

  console.log('showToolFormModal', query);

  router.push({ path: '/tools/edit', query });
}

// 修改确认删除工具函数
async function confirmDeleteTool(tool: Tool) {
  try {
    globalStore.setLoadingState(true);

    await dialog.warning({
      title: '确认删除',
      content: `是否删除工具"${tool.name}"？此操作不可恢复。`,
      positiveText: '确认',
      negativeText: '取消',
      style: {
        position: 'relative',
        marginTop: '20vh'
      },
      onPositiveClick: async () => {
        const success = await toolStore.removeTool(tool.id);
        if (success) {
          message.success('工具已删除');
        } else {
          throw new Error('删除工具失败');
        }
      }
    });
  } catch (error) {
    console.error('删除工具出错:', error);
    message.error('删除工具时发生错误');
  } finally {
    globalStore.setLoadingState(false);
  }
}

// 编辑工具
function handleEditTool(tool: Tool) {
  console.log('Editing tool:', toolStore.tools);
  router.push(`/tools/edit/${tool.id}`);
}

// 获取工具图标
function getToolIcon(tool: Tool) {
  if (tool.iconId) {
    return iconStore.getIconById(tool.iconId);
  }
  return null;
}

// 获取工具类型显示文本
function getTypeDisplay(type: string): string {
  switch(type) {
    case 'js':
      return 'JavsScript';
    case 'mcpIo':
      return 'MCP-STDIO';
    case 'mcpSse':
      return 'MCP-SSE';
    default:
      return type.toUpperCase();
  }
}

// 获取工具类型标签颜色
function getTypeTagColor(type: string): "error" | "info" | "success" | "warning" | "default" | "primary" {
  switch(type) {
    case 'javaScript':
      return 'success';
    case 'mcpIo':
      return 'info';
    case 'mcpSse':
      return 'warning';
    default:
      return 'default';
  }
}

// 打开编辑分类对话框
function openEditCategory(category: ToolCategory) {
  categoryToEdit.value = category;
  showAddCategoryModal.value = true;
}

// 编辑分类
async function updateCategory(category: ToolCategory) {
  try {
    globalStore.setLoadingState(true);

    // 检查类别名称是否存在
    const duplicatedCategory = categoryStore.categories.find(
      c => c.name.toLowerCase() === category.name.toLowerCase() && 
           c.id !== category.id
    );
    
    if (duplicatedCategory) {
      message.warning('分类名称已存在');
      return; // 不关闭对话框
    }
    
    const success = await categoryStore.updateCategory(category);
    
    if (success) {
      message.success('分类已更新');
      showAddCategoryModal.value = false;
      categoryToEdit.value = null;
    } else {
      message.error('更新分类失败');
    }
  } catch (error) {
    console.error('更新分类时出错:', error);
    message.error('更新分类时发生错误');
  } finally {
    globalStore.setLoadingState(false);
  }
}

// 在组件挂载时加载数据
onMounted(async () => {
  console.log('Tools view mounted');
  try {
    globalStore.setLoadingState(true);
    await categoryStore.fetchCategories();
  
    // 只获取所有工具数据，不再按分类获取
    await toolStore.fetchAllTools();
  
  } catch (error) {
    console.error('加载工具数据时出错:', error);
  } finally {
    globalStore.setLoadingState(false);
  }
});

// 组件卸载时清理状态
onBeforeUnmount(() => {
  categoryToEdit.value = null;
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
  min-height: unset;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  transition: all 0.3s ease;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.05);
  padding: 4px;
}

.tool-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 12px rgba(0, 0, 0, 0.08);
  background-color: rgba(24, 160, 88, 0.02);
}

/* 工具类型标签样式 */
.tool-type-tag {
  position: absolute;
  top: 8px;
  right: 8px;
  z-index: 1;
}

.tool-card-header {
  display: flex;
  align-items: center;
  gap: 4px;
}

.tool-card-title {
  font-size: 14px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tool-card-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-top: 8px;
  border-top: 1px solid rgba(0, 0, 0, 0.05);
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

.category-item-wrapper {
  display: flex;
  align-items: center;
  width: 100%;
  position: relative;
  min-height: 24px; /* 确保有足够的高度容纳按钮 */
}

.category-name {
  flex-grow: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 75px; /* 增加最大宽度，但保留空间给按钮 */
  cursor: pointer;
}

/* 分类操作按钮容器 - 改为绝对定位，使其悬浮，添加透明效果 */
.category-actions {
  position: absolute;
  right: 0px;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  visibility: hidden;
  background-color: transparent; /* 透明背景 */
  border-radius: 4px;
  z-index: 2;
  padding: 0 6px;
  transition: all 0.2s ease;
}

/* 确保在暗黑模式下也有正确的背景色 */
html.dark .category-actions {
  background-color: transparent; /* 透明背景 */
}

.category-item:hover .category-actions {
  visibility: visible;
}

.category-action-btn {
  opacity: 0.6;
  transition: all 0.2s;
  margin: 0 1px; /* 减小按钮间距 */
  padding: 0; /* 移除内边距 */
  width: 18px; /* 固定按钮宽度 */
  height: 18px; /* 固定按钮高度 */
  min-width: unset; /* 移除最小宽度限制 */
  background-color: transparent; /* 按钮透明背景 */
}

.category-action-btn:hover {
  opacity: 1;
  background-color: rgba(0, 0, 0, 0.1); /* 悬停时轻微背景 */
}

/* 暗色模式悬停效果 */
html.dark .category-action-btn:hover {
  background-color: rgba(255, 255, 255, 0.1); /* 暗模式下悬停背景 */
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
