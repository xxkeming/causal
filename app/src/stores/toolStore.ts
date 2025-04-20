import { defineStore } from 'pinia';
import { ref } from 'vue';
import { useGlobalStore } from './globalStore';
import * as api from '../services/api';
import { Tool } from '../services/typings';

export const useToolStore = defineStore('tool', () => {
  const globalStore = useGlobalStore();
  const tools = ref<Tool[]>([]);
  
  // 获取所有工具
  async function fetchAllTools() {
    globalStore.setLoadingState(true);
    try {
      if (tools.value.length === 0) {
        tools.value = await api.getAllTools();
      }
    } catch (error) {
      console.error('Failed to fetch tools:', error);
    } finally {
      globalStore.setLoadingState(false);
    }
  }
  
  // 获取单个工具
  async function fetchToolById(id: number): Promise<Tool | null> {
    globalStore.setLoadingState(true);
    try {
      const tool = await api.getToolById(id);
      return tool;
    } catch (error) {
      console.error(`Failed to fetch tool ${id}:`, error);
      return null;
    } finally {
      globalStore.setLoadingState(false);
    }
  }
  
  // 创建工具
  async function createTool(tool: Omit<Tool, 'id' | 'createdAt' | 'updatedAt'>) {
    globalStore.setLoadingState(true);
    try {
      await api.addTool(tool);
      tools.value = await api.getAllTools();
    } catch (error) {
      console.error('Failed to create tool:', error);
      throw error;
    } finally {
      globalStore.setLoadingState(false);
    }
  }
  
  // 更新工具
  async function modifyTool(tool: Tool) {
    globalStore.setLoadingState(true);
    try {
      const index = tools.value.findIndex(t => t.id === tool.id);
      if (index === -1) {
        throw new Error(`Tool with ID ${tool.id} not found`);
      }
      const toolData: Tool = {
        ... tools.value[index],
        ...tool,
        updatedAt: new Date()
      }
      await api.updateTool(toolData);

      tools.value[index] = toolData;

    } catch (error) {
      console.error(`Failed to update tool ${tool.id}:`, error);
      throw error;
    } finally {
      globalStore.setLoadingState(false);
    }
  }
  
  // 删除工具
  async function removeTool(id: number) {
    globalStore.setLoadingState(true);
    try {
      const result = await api.deleteTool(id);
      if (result) {
        const index = tools.value.findIndex(t => t.id === id);
        if (index !== -1) {
          tools.value.splice(index, 1);
        }
      }
      return result;
    } catch (error) {
      console.error(`Failed to delete tool ${id}:`, error);
      return false;
    } finally {
      globalStore.setLoadingState(false);
    }
  }
  
  // 根据类别删除工具
  async function removeToolByCategory(categoryId: number) {
    globalStore.setLoadingState(true);
    try {
      const result = await api.deleteToolByCategory(categoryId);
      if (result) {
        tools.value = tools.value.filter(tool => tool.categoryId !== categoryId);
      }
      return result;
    } catch (error) {
      console.error(`Failed to delete tools by category ${categoryId}:`, error);
      return false;
    } finally {
      globalStore.setLoadingState(false);
    }
  }
  
  // 设置加载状态
  function setTestState(testing: boolean) {
    globalStore.isLoading = testing;
  }

  return {
    tools,
    loading: globalStore.isLoading,
    setTestState,
    fetchAllTools,
    fetchToolById,
    createTool,
    modifyTool,
    removeTool,
    removeToolByCategory
  };
});
