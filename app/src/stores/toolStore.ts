import { defineStore } from 'pinia';
import { ref } from 'vue';
import { Tool } from '../services/typings';
import { 
  getAllTools, getToolById,
  addTool, updateTool, deleteTool 
} from '../services/api';

export const useToolStore = defineStore('tool', () => {
  // 状态
  const tools = ref<Tool[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const needRefresh = ref(false);
  
  // 获取所有工具 - 保留此方法作为唯一的数据获取入口
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
  
  // 移除 fetchToolsByCategory 方法
  
  // 获取单个工具详情
  async function fetchToolById(id: string | number) {
    loading.value = true;
    error.value = null;
    
    try {
      return await getToolById(id as number);
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
      tools.value.push(newTool);
      
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
  async function removeTool(id: string | number) {
    loading.value = true;
    error.value = null;
    
    try {
      const success = await deleteTool(id as number);
      
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
  
  // 设置需要刷新标志
  function setNeedRefresh() {
    needRefresh.value = true;
  }
  
  return {
    tools,
    loading,
    error,
    needRefresh,
    fetchAllTools,
    // 从导出中移除 fetchToolsByCategory
    fetchToolById,
    createTool,
    modifyTool,
    removeTool,
    setNeedRefresh
  };
});
