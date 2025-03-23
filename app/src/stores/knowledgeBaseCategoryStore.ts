import { defineStore } from 'pinia';
import { ref } from 'vue';
import { KnowledgeBaseCategory } from '../services/typings';
import { getKnowledgeBaseCategories } from '../services/api';

export const useKnowledgeBaseCategoryStore = defineStore('knowledgeBaseCategory', () => {
  // 状态
  const categories = ref<KnowledgeBaseCategory[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  
  // 获取知识库类别（固定的两种）
  async function fetchCategories() {
    loading.value = true;
    error.value = null;
    
    try {
      categories.value = await getKnowledgeBaseCategories();
    } catch (err) {
      console.error('Failed to fetch knowledge base categories:', err);
      error.value = '获取知识库类别失败';
    } finally {
      loading.value = false;
    }
  }
  
  return {
    categories,
    loading,
    error,
    fetchCategories
  };
});
