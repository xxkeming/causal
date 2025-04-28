<template>
  <div class="tool-selector">
    <div class="tools-filter">
      <n-input
        v-model:value="searchKeyword"
        placeholder="搜索工具..."
        clearable
        size="small"
        class="search-input"
      >
        <template #prefix>
          <n-icon><SearchOutline /></n-icon>
        </template>
      </n-input>
      
      <div class="category-tabs">
        <n-tabs
          v-model:value="currentCategory"
          type="line"
          size="small"
          @update:value="handleCategoryChange"
        >
          <n-tab-pane 
            name="all" 
            :tab="() => renderTabWithCounter('全部', props.modelValue.length)"
          />
          <n-tab-pane
            v-for="category in categories"
            :key="category.id"
            :name="category.id.toString()"
            :tab="() => renderTabWithCounter(category.name, getCategorySelectedCount(category.id))"
          />
        </n-tabs>
      </div>
    </div>
    
    <div class="tools-list">
      <div v-if="filteredTools.length === 0" class="empty-tools">
        <n-empty size="small" description="没有找到匹配的工具" />
      </div>
      
      <n-scrollbar style="max-height: 240px" v-slot:default>
        <div class="tools-selection">
          <!-- 将n-space替换为n-grid，设置为2列 -->
          <n-grid :cols="2" :x-gap="8" :y-gap="8">
            <n-grid-item v-for="tool in filteredTools" :key="tool.id">
              <div
                class="tool-item"
                :class="{ selected: isSelected(tool.id) }"
                @click="toggleToolSelection(tool)"
              >
                <div class="tool-icon">
                  <n-avatar
                    round
                    :size="28"
                    :color="getToolIcon(tool)?.color || '#d9d9d9'"
                  >
                    <n-icon v-if="getToolIcon(tool)">
                    <component :is="getToolIcon(tool)?.icon" />
                    </n-icon>
                    <n-icon v-else><BuildOutline /></n-icon>
                  </n-avatar>
                </div>
                <div class="tool-info">
                  <div class="tool-name">{{ tool.name }}</div>
                  <div class="tool-description">{{ tool.description }}</div>
                </div>
              </div>
            </n-grid-item>
          </n-grid>
        </div>
      </n-scrollbar>
    </div>

    <div class="selected-tools-footer">
      <div class="filter-switch">
        <n-switch 
          v-model:value="showOnlySelected" 
          size="small"
        >
          <template #checked>只显示已选择</template>
          <template #unchecked>显示全部</template>
        </n-switch>
      </div>
      <div class="selected-tools-counter">
        已选择 {{ modelValue.length }} 个工具
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, h } from 'vue';
import { 
  NInput, NIcon, NTabs, NTabPane, NEmpty, NScrollbar, 
  NAvatar, NSwitch, NGrid, NGridItem, NBadge 
} from 'naive-ui';
import { SearchOutline, BuildOutline } from '@vicons/ionicons5';
import { useToolStore } from '../stores/toolStore';
import { useToolCategoryStore } from '../stores/toolCategoryStore';
import { useIconStore } from '../stores/iconStore';
import { Tool } from '../services/typings';

const props = defineProps({
  modelValue: {
    type: Array as () => number[],
    default: () => []
  },
  disabled: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['update:modelValue', 'change']);

// 加载Store
const toolStore = useToolStore();
const categoryStore = useToolCategoryStore();
const iconStore = useIconStore();

// 本地状态
const searchKeyword = ref('');
const currentCategory = ref('all');
const tools = ref<Tool[]>([]);
const showOnlySelected = ref(false); // 添加过滤开关状态

// 获取所有工具分类
const categories = computed(() => categoryStore.categories);

// 新增计算属性：计算每个分类已选择的工具数量
const getCategorySelectedCount = (categoryId: number) => {
  if (!tools.value || props.modelValue.length === 0) return 0;
  
  // 获取指定分类下的所有工具
  const categoryTools = tools.value.filter(tool => tool.categoryId === categoryId);
  
  // 计算这些工具中有多少被选中了
  const selectedCount = categoryTools.filter(tool => 
    props.modelValue.includes(tool.id)
  ).length;
  
  return selectedCount;
};

// 按分类和搜索关键字过滤工具
const filteredTools = computed(() => {
  if (!tools.value) return [];
  
  // 先按分类过滤
  let result = tools.value;
  
  if (currentCategory.value !== 'all') {
    const categoryId = parseInt(currentCategory.value);
    result = result.filter(tool => tool.categoryId === categoryId);
  }
  
  // 如果开启了"只显示已选择"，则过滤出已选择的工具
  if (showOnlySelected.value) {
    result = result.filter(tool => isSelected(tool.id));
  }
  
  // 再按搜索关键词过滤
  const keyword = searchKeyword.value.trim().toLowerCase();
  if (keyword) {
    result = result.filter(tool => 
      tool.name.toLowerCase().includes(keyword) || 
      tool.description.toLowerCase().includes(keyword)
    );
  }
  
  return result;
});

// 检查工具是否被选中
const isSelected = (toolId: number) => {
  return props.modelValue.includes(toolId);
};

// 切换工具选择状态
const toggleToolSelection = (tool: Tool) => {
  if (props.disabled) return;
  
  const index = props.modelValue.indexOf(tool.id);
  const newSelection = [...props.modelValue];
  
  if (index === -1) {
    newSelection.push(tool.id);
  } else {
    newSelection.splice(index, 1);
  }
  
  emit('update:modelValue', newSelection);
  emit('change', newSelection);
};

// 处理分类变化
const handleCategoryChange = (categoryId: string) => {
  currentCategory.value = categoryId;
};

// 获取工具图标
const getToolIcon = (tool: Tool) => {
  if (tool.iconId) {
    return iconStore.getIconById(tool.iconId);
  }
  return null;
};

// 添加渲染标签页的函数，包含选中计数器
const renderTabWithCounter = (label: string, count: number) => {
  return h(NBadge, { 
          value: count, 
          // color: '#18a058',
          offset: [10, 5]
        }, {
          default: () => h('span', {}, label)
        });
};

// 初始化加载工具数据
const loadToolsData = async () => {
  // 加载分类
  if (categoryStore.categories.length === 0) {
    await categoryStore.fetchCategories();
  }
  
  // 加载所有工具
  await toolStore.fetchAllTools();
  tools.value = toolStore.tools;
};

onMounted(() => {
  loadToolsData();
});
</script>

<style scoped>
.tool-selector {
  display: flex;
  flex-direction: column;
  width: 100%;
  /* 移除边框 */
  border-radius: 6px;
  overflow: hidden;
  max-height: 380px;
}

.tools-filter {
  padding: 1px;
  background-color: var(--card-color);
}

.search-input {
  margin-bottom: 12px;
}

.category-tabs {
  margin-top: 4px;
}

.tools-list {
  flex: 1;
  min-height: 120px; /* 减小最小高度 */
  padding: 8px 0;
  overflow: hidden; /* 确保内容不溢出 */
}

.empty-tools {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 120px; /* 减小空状态高度 */
}

.tool-item {
  display: flex;
  align-items: center;
  padding: 8px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
  /* 调整工具项的高度和布局，适应网格布局 */
  min-height: 64px;
  height: 100%;
}

.tool-item:hover {
  background-color: var(--hover-color-overlay);
}

.tool-item.selected {
  background-color: rgba(24, 160, 88, 0.08);
}

.tool-icon {
  margin-right: 12px;
  flex-shrink: 0;
}

.tool-info {
  flex: 1;
  min-width: 0;
  margin-right: 8px;
  /* 确保内容不会溢出 */
  overflow: hidden;
}

.tool-name {
  font-weight: 500;
  margin-bottom: 2px;
}

.tool-description {
  font-size: 12px;
  color: var(--text-color-3);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.selected-tools-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  font-size: 13px;
  color: var(--text-color-2);
  border-top: 1px solid var(--border-color);
  background-color: rgba(0, 0, 0, 0.02);
}

.selected-tools-counter {
  padding: 0;
}

.filter-switch {
  flex-shrink: 0;
}

/* 自定义滚动条样式 */
:deep(.n-scrollbar) {
  --n-scrollbar-width: 8px;
  --n-scrollbar-height: 8px;
}

/* 标签页中徽章的样式 */
.tab-label-wrapper {
  display: flex;
  align-items: center;
  gap: 5px;
}

:deep(.tab-badge .n-badge-sup) {
  padding: 2px 6px;
  font-size: 11px;
  font-weight: 500;
  transform: translateY(-2px) translateX(4px);
}

/* 让标签页内容垂直居中对齐 */
:deep(.n-tabs-tab__label) {
  display: flex;
  align-items: center;
}
</style>
