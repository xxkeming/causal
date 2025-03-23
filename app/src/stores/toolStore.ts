import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { Tool } from '../services/typings';
import { 
  getAllTools, getToolsByCategory, getToolById,
  addTool, updateTool, deleteTool 
} from '../services/api';

export const useToolStore = defineStore('tool', () => {
  // 状态
  const tools = ref<Tool[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const selectedCategoryId = ref<number | 'all'>('all');
  const searchKeyword = ref<string>('');
  const needRefresh = ref(false);
  
  // 计算属性：按搜索关键词过滤后的工具
  const displayTools = computed(() => {
    if (!searchKeyword.value) {
      return tools.value;
    }
    
    const keyword = searchKeyword.value.toLowerCase();
    return tools.value.filter(tool => 
      tool.name.toLowerCase().includes(keyword) || 
      (tool.description && tool.description.toLowerCase().includes(keyword))
    );
  });
  
  // 获取所有工具
  async function fetchAllTools() {
    loading.value = true;
    error.value = null;
    
    try {
      tools.value = await getAllTools();
      needRefresh.value = false;
    } catch (err) {
      console.error('Failed to fetch tools:', err);
      error.value = '获取工具失败';
    } finally {
      loading.value = false;
    }
  }
  
  // 按分类获取工具
  async function fetchToolsByCategory(categoryId: number | 'all') {
    loading.value = true;
    error.value = null;
    selectedCategoryId.value = categoryId;
    
    try {
      if (categoryId === 'all') {
        tools.value = await getAllTools();
      } else {
        tools.value = await getToolsByCategory(categoryId);
      }
      needRefresh.value = false;
    } catch (err) {
      console.error(`Failed to fetch tools for category ${categoryId}:`, err);
      error.value = '按分类获取工具失败';
    } finally {
      loading.value = false;
    }
  }
  
  // 获取单个工具详情
  async function fetchToolById(id: number) {
    loading.value = true;
    error.value = null;
    
    try {
      return await getToolById(id);
    } catch (err) {
      console.error(`Failed to fetch tool with id ${id}:`, err);
      error.value = '获取工具详情失败';
      return null;
    } finally {
      loading.value = false;
    }
  }
  
  // 创建工具
  async function createTool(toolData: Omit<Tool, 'id' | 'createdAt'>) {
    loading.value = true;
    error.value = null;
    
    try {
      const newTool = await addTool(toolData);
      
      // 如果当前显示的是全部或者对应分类，则添加到列表中
      if (selectedCategoryId.value === 'all' || selectedCategoryId.value === newTool.categoryId) {
        tools.value.push(newTool);
      }
      
      return newTool;
    } catch (err) {
      console.error('Failed to create tool:', err);
      error.value = '创建工具失败';
      throw err;
    } finally {
      loading.value = false;
    }
  }
  
  // 更新工具
  async function modifyTool(toolData: Tool) {
    loading.value = true;
    error.value = null;
    
    try {
      const updatedTool = await updateTool(toolData);
      
      // 更新本地列表中的对应项
      const index = tools.value.findIndex(t => t.id === updatedTool.id);
      if (index !== -1) {
        tools.value[index] = updatedTool;
      }
      
      return updatedTool;
    } catch (err) {
      console.error(`Failed to update tool with id ${toolData.id}:`, err);
      error.value = '更新工具失败';
      throw err;
    } finally {
      loading.value = false;
    }
  }
  
  // 删除工具
  async function removeTool(id: number) {
    loading.value = true;
    error.value = null;
    
    try {
      const success = await deleteTool(id);
      
      if (success) {
        // 从本地列表中移除
        tools.value = tools.value.filter(t => t.id !== id);
      }
      
      return success;
    } catch (err) {
      console.error(`Failed to delete tool with id ${id}:`, err);
      error.value = '删除工具失败';
      return false;
    } finally {
      loading.value = false;
    }
  }
  
  // 设置搜索关键词
  function setSearchKeyword(keyword: string) {
    searchKeyword.value = keyword;
  }
  
  // 设置已选分类
  function selectCategory(categoryId: number | 'all') {
    selectedCategoryId.value = categoryId;
  }
  
  // 设置需要刷新标志
  function setNeedRefresh() {
    needRefresh.value = true;
  }
  
  return {
    tools,
    loading,
    error,
    searchKeyword,
    selectedCategoryId,
    needRefresh,
    displayTools,
    fetchAllTools,
    fetchToolsByCategory,
    fetchToolById,
    createTool,
    modifyTool,
    removeTool,
    setSearchKeyword,
    selectCategory,
    setNeedRefresh
  };
});
