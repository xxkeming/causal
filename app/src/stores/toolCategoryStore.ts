import { defineStore } from 'pinia';
import { ref } from 'vue';
import { ToolCategory } from '../services/typings';
import { getToolCategories, addToolCategory, deleteToolCategory } from '../services/api';

export const useToolCategoryStore = defineStore('toolCategory', () => {
  // 状态
  const categories = ref<ToolCategory[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  
  // 获取所有工具类别
  async function fetchCategories() {
    loading.value = true;
    error.value = null;
    
    try {
      // 移除可能的条件判断，始终获取最新数据
      if (categories.value.length === 0) {
        categories.value = await getToolCategories();
      }
    } catch (err) {
      console.error('Failed to fetch tool categories:', err);
      error.value = '获取工具类别失败';
    } finally {
      loading.value = false;
    }
  }
  
  // 添加工具类别
  async function addCategory(name: string) {
    loading.value = true;
    error.value = null;
    
    try {
      // 调用API添加类别
      const newCategory = await addToolCategory({ name });
      // 直接添加到本地数组，确保界面立即更新
      categories.value.push(newCategory);
      return newCategory;
    } catch (err) {
      console.error('Failed to add tool category:', err);
      error.value = '添加工具类别失败';
      throw err;
    } finally {
      loading.value = false;
    }
  }
  
  // 删除工具类别
  async function removeCategory(categoryId: number) {
    loading.value = true;
    error.value = null;
    
    try {
      const success = await deleteToolCategory(categoryId);
      if (success) {
        categories.value = categories.value.filter(c => c.id !== categoryId);
      }
      return success;
    } catch (err) {
      console.error('Failed to delete tool category:', err);
      error.value = '删除工具类别失败';
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
