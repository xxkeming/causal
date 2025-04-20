import { defineStore } from 'pinia';
import { ref } from 'vue';
import { useGlobalStore } from './globalStore';
import { KnowledgeBaseCategory } from '../services/typings';

export const useKnowledgeBaseCategoryStore = defineStore('knowledgeBaseCategory', () => {
  const globalStore = useGlobalStore();
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
  const error = ref<string | null>(null);
  
  return {
    categories,
    loading: globalStore.isLoading,
    error
  };
});
