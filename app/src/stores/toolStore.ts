import { defineStore } from 'pinia';
import { ref } from 'vue';
import * as api from '../services/api';
import { Tool } from '../services/typings';

export const useToolStore = defineStore('tool', () => {
  const tools = ref<Tool[]>([]);
  
  // 获取所有工具
  async function fetchAllTools() {
    try {
      if (tools.value.length == 0) {
        tools.value = await api.getAllTools();
        console.log('fetchAllTools', tools.value[0]);
      }
    } catch (error) {
      console.error('Failed to fetch tools:', error);
    }
  }
  
  // 获取单个工具
  async function fetchToolById(id: number): Promise<Tool | null> {
    try {
      const toolIndex = tools.value.findIndex(t => t.id === id);
      if (toolIndex !== -1) {
        return tools.value[toolIndex];
      }
      const tool = await api.getToolById(id);
      return tool;
    } catch (error) {
      console.error(`Failed to fetch tool ${id}:`, error);
      throw error;
    }
  }
  
  // 创建工具
  async function createTool(tool: Omit<Tool, 'id' | 'createdAt' | 'updatedAt'>) {
    try {
      await api.addTool(tool);
      tools.value = await api.getAllTools();
    } catch (error) {
      console.error('Failed to create tool:', error);
      throw error;
    }
  }
  
  // 更新工具
  async function modifyTool(tool: Tool) {
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
    }
  }
  
  // 删除工具
  async function removeTool(id: number) {
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
    }
  }
  
  // 根据类别删除工具
  async function removeToolByCategory(categoryId: number) {
    try {
      const result = await api.deleteToolByCategory(categoryId);
      if (result) {
        tools.value = tools.value.filter(tool => tool.categoryId !== categoryId);
      }
      return result;
    } catch (error) {
      console.error(`Failed to delete tools by category ${categoryId}:`, error);
      return false;
    }
  }

  return {
    tools,
    fetchAllTools,
    fetchToolById,
    createTool,
    modifyTool,
    removeTool,
    removeToolByCategory
  };
});
