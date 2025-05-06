import { defineStore } from 'pinia';
import { ref } from 'vue';
import * as api from '../services/api';
import { ToolCategory } from '../services/typings';

export const useToolCategoryStore = defineStore('toolCategory', () => {
  const categories = ref<ToolCategory[]>([]);
  const error = ref<string | null>(null);
  
  // 获取所有工具类别
  async function fetchCategories() {
    error.value = null;
    
    try {
      if (categories.value.length === 0) {
        categories.value = await api.getToolCategories();
      }
    } catch (err) {
      console.error('Failed to fetch tool categories:', err);
      error.value = '获取工具类别失败';
    }
  }
  
  // 添加工具类别
  async function addCategory(name: string) {
    error.value = null;
    
    try {
      await api.addToolCategory({ name });
      categories.value = await api.getToolCategories();
    } catch (err) {
      console.error('Failed to add tool category:', err);
      error.value = '添加工具类别失败';
      throw err;
    }
  }
  
  // 删除工具类别
  async function removeCategory(id: number) {
    error.value = null;
    
    try {
      const result = await api.deleteToolCategory(id);
      if (result) {
        const index = categories.value.findIndex(c => c.id === id);
        if (index !== -1) {
          categories.value.splice(index, 1);
        }
      }
      return result;
    } catch (err) {
      console.error(`Failed to delete tool category ${id}:`, err);
      error.value = '删除工具类别失败';
      return false;
    }
  }

  /**
   * 更新工具类别
   */
  async function updateCategory(category: ToolCategory): Promise<boolean> {
    try {
      // 调用API更新类别
      const result = await api.updateToolCategory(category);
      
      if (result) {
        // 更新本地状态
        const index = categories.value.findIndex(c => c.id === category.id);
        if (index !== -1) {
          categories.value[index] = category;
        }
      }
      
      return result;
    } catch (error) {
      console.error('Failed to update category:', error);
      return false;
    }
  }
  
  return {
    categories,
    error,
    fetchCategories,
    addCategory,
    removeCategory,
    updateCategory // 导出新函数
  };
});
