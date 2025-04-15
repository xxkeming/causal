<template>
  <div class="agents-view">
    <n-layout has-sider position="absolute">
      <!-- 左侧分类列表 - 使用列表组件 -->
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
            <!-- 所有智能体选项 -->
            <n-list-item 
              class="category-item"
              :class="{ active: selectedCategory === 0 }"
              @click="handleSelectCategory(0)"
            >
              <div class="category-content">
                所有智能体
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
        <n-layout position="absolute" style="left: 120px; right: 0;" class="agents-content">
          <div class="agents-header">
            <div class="agents-header-left">
              <n-input
                v-model:value="searchKeyword"
                placeholder="搜索智能体..."
                clearable
                class="agents-search"
              >
                <template #prefix>
                  <n-icon><SearchOutline /></n-icon>
                </template>
              </n-input>
            </div>
            <n-button type="primary" class="create-button" @click="showAgentFormModal">
              <template #icon>
                <n-icon><AddOutline /></n-icon>
              </template>
              创建智能体
            </n-button>
          </div>
        
          <n-spin :show="agentStore.loading || categoryStore.loading">
            <div class="agents-cards">
              <n-grid :cols="4" :x-gap="16" :y-gap="16" responsive="screen">
                <n-grid-item v-for="agent in filteredAgents" :key="agent.id">
                  <n-card hoverable class="agent-card" :bordered="false">
                    <template #header>
                      <div class="agent-card-header">
                        <n-avatar round :size="32" :color="getAgentIcon(agent)?.color || '#d9d9d9'">
                          <n-icon size="24" v-if="getAgentIcon(agent)">
                            <component :is="getAgentIcon(agent)?.icon" />
                          </n-icon>
                          <n-icon size="24" v-else><ServerOutline /></n-icon>
                        </n-avatar>
                        <div class="agent-card-title">{{ agent.name }}</div>
                      </div>
                    </template>
                  
                    <div class="agent-card-description">
                      {{ agent.description }}
                    </div>
                  
                    <template #footer>
                      <div class="agent-card-footer">
                        <!-- 日期信息 -->
                        <div class="agent-card-date">
                          更新于: {{ formatDate(agent.updatedAt || agent.createdAt) }}
                        </div>
                      
                        <!-- 操作按钮 -->
                        <div class="agent-card-actions">
                          <n-space>
                            <n-button circle tertiary size="small" type="info" title="编辑" @click="showEditAgentModal(agent)">
                              <template #icon>
                                <n-icon><CreateOutline /></n-icon>
                              </template>
                            </n-button>
                            <n-button circle tertiary size="small" type="error" title="删除" @click="confirmDeleteAgent(agent)">
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
            
              <!-- 修改空状态组件，使用过滤后的代理列表检查 -->
              <div v-if="filteredAgents.length === 0 && !agentStore.loading" class="empty-state-container">
                <n-empty :description="emptyDescription" />
              </div>
            </div>
          </n-spin>
        </n-layout>
      </n-layout>
    
      <!-- 添加/编辑类别对话框 -->
      <agent-category-add-modal
        v-model:visible="showAddCategoryModal"
        :category="categoryToEdit || undefined"
        @submit="addNewCategory"
        @update="updateCategory"
      />
    
      <!-- 删除类别确认对话框 -->
      <agent-category-delete-modal
        v-model:visible="showDeleteCategoryModal"
        :category="categoryToDelete"
        @confirm="deleteCategory"
      />
    
      <!-- 智能体表单模态框 -->
      <agent-form-modal
        v-model:visible="showAgentModal"
        :is-edit="isEditingAgent"
        :agent-data="currentAgent"
        :default-category-id="selectedCategory"
        @submit="handleAgentFormSubmit"
        @cancel="handleAgentFormCancel"
      />

      <!-- 替换为智能体删除确认组件 -->
      <agent-delete-modal
        v-model:visible="showDeleteAgentModal"
        :agent="agentToDelete"
        @confirm="deleteAgent"
        @cancel="cancelDeleteAgent"
      />
    </div>
  </template>

  <script setup lang="ts">
  import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
  import { 
    NLayout, NLayoutSider, NList, NListItem, NCard, NAvatar, 
    NButton, NIcon, NGrid, NGridItem, NInput, NScrollbar,
    NEmpty, NSpace, NSpin, NTooltip
  } from 'naive-ui';
  import { 
    ServerOutline, AddOutline, SearchOutline, 
    CreateOutline, TrashBinOutline, CloseOutline,
  } from '@vicons/ionicons5';
  import { Agent, AgentCategory } from '../../services/typings';
  import { useMessage } from 'naive-ui';
  import { useAgentStore } from '../../stores/agentStore';
  import { useAgentCategoryStore } from '../../stores/agentCategoryStore';
  // 导入组件
  import AgentFormModal from './components/AgentFormModal.vue';
  import AgentCategoryAddModal from './components/AgentCategoryAddModal.vue';
  import AgentCategoryDeleteModal from './components/AgentCategoryDeleteModal.vue';
  import AgentDeleteModal from './components/AgentDeleteModal.vue'; // 导入智能体删除组件
  import { useIconStore } from '../../stores/iconStore';

  const message = useMessage();

  // 使用Store
  const agentStore = useAgentStore();
  const categoryStore = useAgentCategoryStore();
  const iconStore = useIconStore();

  // 本地状态
  const searchKeyword = ref<string>('');
  const selectedCategory = ref<number>(0);
  const showAddCategoryModal = ref<boolean>(false);
  const showDeleteCategoryModal = ref<boolean>(false);
  const categoryToDelete = ref<AgentCategory | null>(null);
  const categoryToEdit = ref<AgentCategory | null>(null);

  // 计算属性 - 同时处理分类筛选和关键词搜索
  const filteredAgents = computed(() => {
    let result = agentStore.agents;
    
    // 先按分类筛选
    if (selectedCategory.value !== 0) {
      result = result.filter(agent => agent.categoryId === selectedCategory.value);
    }
    
    // 再按关键词搜索
    if (searchKeyword.value) {
      const keyword = searchKeyword.value.toLowerCase();
      result = result.filter(agent => 
        agent.name.toLowerCase().includes(keyword) || 
        (agent.description && agent.description.toLowerCase().includes(keyword))
      );
    }
    
    return result;
  });

  // 空状态描述信息
  const emptyDescription = computed(() => {
    if (searchKeyword.value) {
      return '没有找到匹配的智能体';
    }
    return selectedCategory.value === 0 ? '暂无智能体' : '该分类下暂无智能体';
  });

  // 处理分类选择 - 简化为仅更新本地状态
  async function handleSelectCategory(categoryId: number) {
    selectedCategory.value = categoryId;
  }

  // 格式化日期字符串
  function formatDate(timesamp: number): string {
    const date = new Date(timesamp);
    return date.toLocaleDateString('zh-CN');
  }

  // 显示删除类别确认对话框
  function confirmDeleteCategory(category: AgentCategory) {
    categoryToDelete.value = category;
    showDeleteCategoryModal.value = true;
  }

  // 删除类别
  async function deleteCategory() {
    if (!categoryToDelete.value) return;
  
    const categoryId = categoryToDelete.value.id;
  
    try {
      const success = await categoryStore.removeCategory(categoryId);
      if (success) {
        await agentStore.removeAgentByCategory(categoryId);

        // 如果当前选中的是被删除的类别，则切换到"所有智能体"
        if (selectedCategory.value === categoryId) {
          selectedCategory.value = 0;
        }
        message.success('类别已删除');
      } else {
        message.error('删除类别失败');
      }
    } catch (error) {
      message.error('删除类别时发生错误');
    } finally {
      showDeleteCategoryModal.value = false;
      categoryToDelete.value = null;
    }
  }

  // 添加新类别
  async function addNewCategory(name: string) {
    if (!name.trim()) {
      message.error('请输入类别名称');
      return;
    }
  
    try {
      // 先检查是否已存在同名类别
      const existingCategory = categoryStore.categories.find(cat => 
        cat.name.toLowerCase() === name.trim().toLowerCase()
      );
      
      if (existingCategory) {
        message.warning('类别名称已存在');
        return; // 不关闭对话框
      }
      
      await categoryStore.addCategory(name.trim());
      message.success('类别已添加');
      showAddCategoryModal.value = false;
      // 清除可能的编辑状态
      categoryToEdit.value = null;
    } catch (error) {
      message.error('添加类别失败');
      console.error(error);
    }
  }

  // 智能体表单模态框相关
  const showAgentModal = ref(false);
  const isEditingAgent = ref(false);
  const currentAgent = ref<Agent | null>(null);

  // 显示创建智能体模态框
  function showAgentFormModal() {
    currentAgent.value = null;
    isEditingAgent.value = false;
    showAgentModal.value = true;
  }

  // 显示编辑智能体模态框
  function showEditAgentModal(agent: Agent) {
    currentAgent.value = { ...agent }; // 创建一个副本避免直接修改原对象
    isEditingAgent.value = true;
    showAgentModal.value = true;
  }

  // 处理智能体表单提交
  async function handleAgentFormSubmit(agent: Agent) {
    try {
      if (isEditingAgent.value && agent.id) {
        console.log('Editing agent:', agent);
        // 编辑现有智能体
        await agentStore.modifyAgent(agent);
        message.success('智能体已更新');
      } else {
        // 创建新智能体
        await agentStore.createAgent(agent);
        message.success('智能体已创建');
      }

      // 成功时才关闭窗口
      showAgentModal.value = false;
    } catch (error) {
      console.error('Failed to submit agent form:', error);
      message.error(isEditingAgent.value ? '更新智能体失败:' + error : '创建智能体失败:' + error);
    }
  }

  // 处理智能体表单取消
  function handleAgentFormCancel() {
    showAgentModal.value = false;
  }

  // 获取智能体图标
  function getAgentIcon(agent: Agent) {
    if (agent.iconId) {
      return iconStore.getIconById(agent.iconId);
    }
    return null;
  }

  // 在组件挂载时加载数据 - 只获取所有智能体数据
  onMounted(async () => {
    await categoryStore.fetchCategories();
    await agentStore.fetchAllAgents();
  });

  // 添加智能体删除相关状态
  const showDeleteAgentModal = ref(false);
  const agentToDelete = ref<Agent | null>(null);

  // 确认删除智能体 - 更新为使用组件
  function confirmDeleteAgent(agent: Agent) {
    agentToDelete.value = agent;
    showDeleteAgentModal.value = true;
  }

  // 执行删除智能体
  async function deleteAgent() {
    if (!agentToDelete.value || !agentToDelete.value.id) {
      message.error('删除失败：无效的智能体');
      return;
    }

    try {
      const success = await agentStore.removeAgent(agentToDelete.value.id);
      if (success) {
        message.success('智能体已删除');
      } else {
        message.error('删除智能体失败');
      }
    } catch (error) {
      console.error('删除智能体出错:', error);
      message.error('删除智能体时发生错误');
    } finally {
      // 重置状态
      showDeleteAgentModal.value = false;
      agentToDelete.value = null;
    }
  }

  // 取消删除智能体
  function cancelDeleteAgent() {
    showDeleteAgentModal.value = false;
    agentToDelete.value = null;
  }

  // 打开编辑分类对话框
  function openEditCategory(category: AgentCategory) {
    categoryToEdit.value = category;
    showAddCategoryModal.value = true;
  }

  // 编辑分类
  async function updateCategory(category: AgentCategory) {
    try {
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
    }
  }

  // 组件卸载时清理状态
  onBeforeUnmount(() => {
    categoryToEdit.value = null;
    categoryToDelete.value = null;
  });

  </script>

  <style scoped>
  .agents-view {
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

  .agents-content {
    padding: 16px;
    overflow: auto;
    top: 0;
    bottom: 0;
  }

  .agents-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  .agents-header-left {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .agents-search {
    width: 240px;
  }

  .agents-cards {
    margin-top: 20px;
  }

  .agent-card {
    height: 180px; /* 固定卡片高度 */
    display: flex;
    flex-direction: column;
    overflow: hidden;
    transition: all 0.3s ease;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
  }

  .agent-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
  }

  .agent-card-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 12px; /* 增加底部间距 */
  }

  .agent-card-title {
    font-size: 16px;
    font-weight: 500;
  }

  .agent-card-description {
    color: var(--text-color-secondary);
    line-height: 1.5;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1; /* 让描述区域自动填充剩余空间 */
    font-size: 13px; /* 稍微减小字体大小 */
  }

  .agent-card-footer {
    margin-top: auto; /* 将页脚推到底部 */
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 12px;
    border-top: 1px solid rgba(0, 0, 0, 0.05);
  }

  .agent-card-date {
    font-size: 12px;
    color: var(--text-color-secondary);
  }

  .agent-card-actions {
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

  /* 添加响应式调整 */
  @media (max-width: 768px) {
    .agents-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 16px;
    }
  
    .agents-header-left {
      width: 100%;
      flex-direction: column;
      align-items: flex-start;
    }
  
    .agents-search {
      width: 100%;
      margin-top: 8px;
    }
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

  /* 添加响应式布局 */
  @media (max-width: 1400px) {
    .n-grid { grid-template-columns: repeat(3, 1fr) !important; }
  }

  @media (max-width: 1100px) {
    .n-grid { grid-template-columns: repeat(2, 1fr) !important; }
  }

  @media (max-width: 768px) {
    .n-grid { grid-template-columns: repeat(1, 1fr) !important; }
  }
  </style>
