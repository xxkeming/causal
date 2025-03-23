import { defineStore } from 'pinia';
import { ref } from 'vue';
import { KnowledgeBaseCategory } from '../services/typings';

export const useKnowledgeBaseCategoryStore = defineStore('knowledgeBaseCategory', () => {
  const categories: KnowledgeBaseCategory[] = [
    {
      id: "1",
      name: "通用知识库"
    },
    {
      id: "2",
      name: "站点同步"
    }
  ];
  const loading = ref(false);
  const error = ref<string | null>(null);
  
  return {
    categories,
    loading,
    error
  };
});
