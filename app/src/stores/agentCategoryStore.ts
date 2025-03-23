import { defineStore } from 'pinia';
import { ref } from 'vue';
import { AgentCategory } from '../services/typings';
import { getAgentCategories, addAgentCategory, deleteAgentCategory } from '../services/api';

export const useAgentCategoryStore = defineStore('agentCategory', () => {
  // 状态
  const categories = ref<AgentCategory[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  
  // 获取所有智能体类别
  async function fetchCategories() {
    loading.value = true;
    error.value = null;
    
    try {
      // 移除条件判断，始终获取最新数据
      if (categories.value.length === 0) {
        categories.value = await getAgentCategories();
      }
    } catch (err) {
      console.error('Failed to fetch agent categories:', err);
      error.value = '获取类别失败';
    } finally {
      loading.value = false;
    }
  }
  
  // 添加智能体类别
  async function addCategory(name: string) {
    loading.value = true;
    error.value = null;
    
    try {
      const newCategory = await addAgentCategory({ name });
      // 直接添加到本地数组，确保界面立即更新
      categories.value.push(newCategory);
      return newCategory;
    } catch (err) {
      console.error('Failed to add agent category:', err);
      error.value = '添加类别失败';
      throw err;
    } finally {
      loading.value = false;
    }
  }
  
  // 删除智能体类别
  async function removeCategory(categoryId: string) {
    loading.value = true;
    error.value = null;
    
    try {
      const success = await deleteAgentCategory(categoryId);
      if (success) {
        // 删除成功后更新本地数组
        categories.value = categories.value.filter(c => c.id !== categoryId);
      }
      return success;
    } catch (err) {
      console.error('Failed to delete agent category:', err);
      error.value = '删除类别失败';
      return false;
    } finally {
      loading.value = false;
    }
  }
  
  return {
    categories,
    loading,
    error,
    fetchCategories,
    addCategory,
    removeCategory
  };
});
