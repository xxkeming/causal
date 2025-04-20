import { defineStore } from 'pinia';
import { ref } from 'vue';
import { AgentCategory } from '../services/typings';
import { getAgentCategories, addAgentCategory, updateAgentCategory, deleteAgentCategory } from '../services/api';
import { useGlobalStore } from './globalStore';

export const useAgentCategoryStore = defineStore('agentCategory', () => {
  const globalStore = useGlobalStore();
  
  // 状态
  const categories = ref<AgentCategory[]>([]);
  const error = ref<string | null>(null);
  
  // 获取所有智能体类别
  async function fetchCategories() {
    globalStore.setLoadingState(true);
    error.value = null;
    
    try {
      if (categories.value.length === 0) {
        categories.value = await getAgentCategories();
      }
    } catch (err) {
      console.error('Failed to fetch agent categories:', err);
      error.value = '获取类别失败';
    } finally {
      globalStore.setLoadingState(false);
    }
  }
  
  // 添加智能体类别
  async function addCategory(name: string) {
    globalStore.setLoadingState(true);
    error.value = null;
    
    try {
      await addAgentCategory({ name });
      categories.value = await getAgentCategories();
    } catch (err) {
      console.error('Failed to add agent category:', err);
      error.value = '添加类别失败';
      throw err;
    } finally {
      globalStore.setLoadingState(false);
    }
  }
  
  // 删除智能体类别
  async function removeCategory(categoryId: number) {
    globalStore.setLoadingState(true);
    error.value = null;
    
    try {
      const success = await deleteAgentCategory(categoryId);
      if (success) {
        categories.value = categories.value.filter(c => c.id !== categoryId);
      }
      return success;
    } catch (err) {
      console.error('Failed to delete agent category:', err);
      error.value = '删除类别失败';
      return false;
    } finally {
      globalStore.setLoadingState(false);
    }
  }

  async function updateCategory(category: AgentCategory): Promise<boolean> {
    globalStore.setLoadingState(true);
    try {
      const result = await updateAgentCategory(category);
      
      if (result) {
        const index = categories.value.findIndex(c => c.id === category.id);
        if (index !== -1) {
          categories.value[index] = category;
        }
      }
      
      return result;
    } catch (error) {
      console.error('Failed to update category:', error);
      return false;
    } finally {
      globalStore.setLoadingState(false);
    }
  }
  
  return {
    categories,
    loading: globalStore.isLoading,
    error,
    fetchCategories,
    addCategory,
    removeCategory,
    updateCategory
  };
});
