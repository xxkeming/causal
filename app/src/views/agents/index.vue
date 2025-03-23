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
              :class="{ active: selectedCategory === 'all' }"
              @click="handleSelectCategory('all')"
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
    
      <!-- 添加类别对话框 -->
      <agent-category-add-modal
        v-model:visible="showAddCategoryModal"
        @submit="addNewCategory"
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
  import { ref, computed, onMounted, watch } from 'vue';
  import { 
    NLayout, NLayoutSider, NList, NListItem, NCard, NAvatar, 
    NButton, NIcon, NGrid, NGridItem, NInput, NScrollbar,
    NEmpty, NSpace, NSpin
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
  const selectedCategory = ref<string>('all');
  const showAddCategoryModal = ref<boolean>(false);
  const showDeleteCategoryModal = ref<boolean>(false);
  const categoryToDelete = ref<AgentCategory | null>(null);

  // 计算属性 - 同时处理分类筛选和关键词搜索
  const filteredAgents = computed(() => {
    let result = agentStore.agents;
    
    // 先按分类筛选
    if (selectedCategory.value !== 'all') {
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
    return selectedCategory.value === 'all' ? '暂无智能体' : '该分类下暂无智能体';
  });

  // 处理分类选择 - 简化为仅更新本地状态
  async function handleSelectCategory(categoryId: string) {
    selectedCategory.value = categoryId;
  }

  // 格式化日期字符串
  function formatDate(dateString: string): string {
    if (!dateString) return '未知时间';
    const date = new Date(dateString);
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
        // 如果当前选中的是被删除的类别，则切换到"所有智能体"
        if (selectedCategory.value === categoryId) {
          selectedCategory.value = 'all';
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
      await categoryStore.addCategory(name.trim());
      message.success('类别已添加');
      // 移除重新加载类别数据的调用，避免重复添加
      showAddCategoryModal.value = false;
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

      // 刷新智能体列表
      // await agentStore.fetchAllAgents();
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

  // 监听需要刷新状态，简化为只获取全部数据
  watch(() => agentStore.needRefresh, (needRefresh) => {
    if (needRefresh) {
      agentStore.fetchAllAgents();
    }
  }, { immediate: false }); 

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
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    transition: all 0.3s ease;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
    min-height: 200px; /* 确保卡片有一个最小高度，保持美观 */
  }

  .agent-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
  }

  .agent-card-header {
    display: flex;
    align-items: center;
    gap: 12px;
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
    height: 3em; /* 约等于两行文字的高度 */
  }

  .agent-card-footer {
    display: flex;
    flex-direction: column;
    gap: 8px;
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
  </style>
