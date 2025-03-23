<template>
  <n-modal 
    v-model:show="showModal" 
    preset="card" 
    class="agent-selector-modal"
    :title="title"
    :style="{ width: '640px', maxWidth: '90vw' }"
    :transform-origin="'center'"
    style="margin-top: 80px;"
  >
    <div class="agent-selector-header">
      <n-input
        v-model:value="searchKeyword"
        placeholder="搜索智能体..."
        clearable
        class="search-input"
      >
        <template #prefix>
          <n-icon :component="Search" />
        </template>
      </n-input>
      <n-select
        v-model:value="selectedCategory"
        :options="categoryOptions"
        placeholder="全部分类"
        clearable
        class="category-select"
      />
    </div>
    
    <n-scrollbar style="max-height: 60vh" class="agent-list-scrollbar">
      <div class="agents-grid" v-if="filteredAgents.length > 0">
        <div 
          v-for="agent in filteredAgents" 
          :key="agent.id" 
          class="agent-card"
          :class="{ 'agent-card-selected': selectedAgentId === agent.id }"
          @click="confirmAgent(agent)"
        >
          <div class="agent-card-avatar">
            <n-avatar
              round
              :size="48"
              :color="getAgentIconColor(agent)"
              class="agent-avatar"
            >
              <n-icon size="28">
                <component :is="getAgentIconComponent(agent)" />
              </n-icon>
            </n-avatar>
          </div>
          <div class="agent-card-content">
            <div class="agent-card-name">{{ agent.name }}</div>
            <div class="agent-card-description" v-if="agent.description">
              {{ truncateText(agent.description, 60) }}
            </div>
            <div class="agent-card-model" v-if="agent.model">
              {{ agent.model.name }}
            </div>
          </div>
        </div>
      </div>
      
      <n-empty v-else description="没有找到匹配的智能体" />
    </n-scrollbar>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import {
  NModal, NInput, NSelect, NScrollbar, NEmpty, NAvatar,
  NIcon
} from 'naive-ui';
import { Search, ServerOutline } from '@vicons/ionicons5';
import { Agent } from '../services/typings';
import { useAgentStore } from '../stores/agentStore';
import { useAgentCategoryStore } from '../stores/agentCategoryStore';
import { useIconStore } from '../stores/iconStore';

const props = defineProps<{
  visible: boolean;
  currentAgentId?: string;
  title?: string;
}>();

const emit = defineEmits(['update:visible', 'select']);

// 内部状态
const selectedAgentId = ref<string | null>(null);
const searchKeyword = ref('');
const selectedCategory = ref<string>('all');
const showModal = ref(false);

// Store
const agentStore = useAgentStore();
const categoryStore = useAgentCategoryStore();
const iconStore = useIconStore();

// 监听visible属性
watch(() => props.visible, (val) => {
  showModal.value = val;
  if (val) {
    // 当弹窗显示时，预选当前智能体
    selectedAgentId.value = props.currentAgentId || null;
  }
});

// 监听modal的显示状态，同步回visible属性
watch(showModal, (val) => {
  emit('update:visible', val);
});

// 计算标题
const title = computed(() => props.title || '选择智能体');

// 计算筛选后的智能体列表
const filteredAgents = computed(() => {
  let result = agentStore.agents;
  
  // 按分类筛选
  if (selectedCategory.value && selectedCategory.value !== 'all') {
    result = result.filter(agent => agent.categoryId === selectedCategory.value);
  }
  
  // 按关键词搜索
  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase();
    result = result.filter(agent => 
      agent.name.toLowerCase().includes(keyword) || 
      (agent.description && agent.description.toLowerCase().includes(keyword))
    );
  }
  
  return result;
});

// 计算分类选项
const categoryOptions = computed(() => {
  const options = [{
    label: '全部分类',
    value: 'all'
  }];
  
  return options.concat(categoryStore.categories.map(cat => ({
    label: cat.name,
    value: cat.id
  })));
});

// 获取智能体图标 - 修复图标组件获取方法
const getAgentIconComponent = (agent: Agent) => {
  if (agent.iconId) {
    const icon = iconStore.getIconById(agent.iconId);
    // 直接返回图标组件，确保它是有效的Vue组件
    if (icon && icon.icon) {
      return icon.icon;
    }
  }
  // 默认返回ServerOutline作为后备图标
  return ServerOutline;
};

// 获取智能体图标颜色
const getAgentIconColor = (agent: Agent) => {
  if (agent.iconId) {
    const icon = iconStore.getIconById(agent.iconId);
    return icon?.color || '#18a058';
  }
  return '#18a058';
};

// 选择并立即确认智能体
const confirmAgent = (agent: Agent) => {
  if (agent.id === props.currentAgentId) return; // 如果是当前已选中的智能体，不做任何操作
  
  selectedAgentId.value = agent.id;
  emit('select', agent);
  showModal.value = false; // 直接关闭对话框
};

// 文本截断函数
const truncateText = (text: string, maxLength: number) => {
  if (text.length <= maxLength) return text;
  return text.slice(0, maxLength) + '...';
};

// 组件挂载时获取所需数据
onMounted(async () => {
  // 确保有智能体数据
  if (agentStore.agents.length === 0) {
    await agentStore.fetchAllAgents();
  }
  
  // 确保有分类数据
  if (categoryStore.categories.length === 0) {
    await categoryStore.fetchCategories();
  }
});
</script>

<style scoped>
.agent-selector-modal {
  max-width: 90vw;
}

/* 添加靠上显示的样式 */
:deep(.n-modal-container) {
  margin-top: 8vh !important;
  align-items: flex-start !important;
}

:deep(.n-modal-body-wrapper) {
  align-items: flex-start !important;
}

.agent-selector-header {
  display: flex;
  gap: 16px;
  margin-bottom: 16px;
}

.search-input {
  flex-grow: 1;
}

.category-select {
  width: 200px;
}

.agents-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
  padding: 8px 0;
}

.agent-card {
  display: flex;
  padding: 12px;
  border-radius: 8px;
  border: 1px solid var(--border-color, #eaeaea);
  cursor: pointer;
  transition: all 0.2s;
  background-color: transparent;
  position: relative;
  overflow: hidden;
}

.agent-card:hover {
  background-color: rgba(0, 0, 0, 0.03);
  border-color: var(--border-color-hover, #d9d9d9);
}

.agent-card-selected {
  border-color: var(--primary-color, #18a058);
  background-color: rgba(24, 160, 88, 0.05);
}

.agent-card-avatar {
  margin-right: 12px;
  display: flex;
  align-items: center;
}

.agent-avatar {
  display: flex;
  align-items: center;
  justify-content: center;
}

.agent-card-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
}

.agent-card-name {
  font-weight: 600;
  font-size: 15px;
  margin-bottom: 4px;
}

.agent-card-description {
  color: var(--text-color-secondary, #666);
  font-size: 13px;
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.agent-card-model {
  font-size: 12px;
  color: var(--text-color-3, #999);
}

.agent-list-scrollbar {
  margin: 0 -16px;
  padding: 0 16px;
}

/* 确保图标在头像中正确居中显示 */
.agent-avatar :deep(.n-icon) {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  width: 100%;
}

/* 修复avatar内部图标容器的样式，确保图标正确显示 */
.agent-avatar :deep(.n-avatar__icon) {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  width: 100%;
}
</style>
