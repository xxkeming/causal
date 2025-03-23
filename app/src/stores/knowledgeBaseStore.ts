import { defineStore } from 'pinia';
import { ref } from 'vue';
import { KnowledgeBase } from '../services/typings';
import { 
  getAllKnowledgeBases, getKnowledgeBaseById,
  addKnowledgeBase, updateKnowledgeBase, deleteKnowledgeBase 
} from '../services/api';

export const useKnowledgeBaseStore = defineStore('knowledgeBase', () => {
  // 状态
  const knowledgeBases = ref<KnowledgeBase[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const needRefresh = ref(true);
  
  // 获取所有知识库
  async function fetchAllKnowledgeBases() {
    loading.value = true;
    error.value = null;
    
    try {
      if (needRefresh.value) {
        knowledgeBases.value = await getAllKnowledgeBases();
      }
      needRefresh.value = false;
    } catch (err) {
      console.error('Failed to fetch knowledge bases:', err);
      error.value = '获取知识库失败';
    } finally {
      loading.value = false;
    }
  }
  
  // 获取单个知识库详情
  async function fetchKnowledgeBaseById(id: string) {
    loading.value = true;
    error.value = null;
    
    try {
      const knowledgeBase = await getKnowledgeBaseById(id);
      return knowledgeBase;
    } catch (err) {
      console.error(`Failed to fetch knowledge base with id ${id}:`, err);
      error.value = '获取知识库详情失败';
      return null;
    } finally {
      loading.value = false;
    }
  }
  
  // 创建知识库
  async function createKnowledgeBase(knowledgeBaseData: Omit<KnowledgeBase, 'id' | 'createdAt'>) {
    loading.value = true;
    error.value = null;
    
    try {
      const newKnowledgeBase = await addKnowledgeBase(knowledgeBaseData);
      knowledgeBases.value.push(newKnowledgeBase);
      return newKnowledgeBase;
    } catch (err) {
      console.error('Failed to create knowledge base:', err);
      error.value = '创建知识库失败';
      throw err;
    } finally {
      loading.value = false;
    }
  }
  
  // 更新知识库
  async function modifyKnowledgeBase(knowledgeBaseData: KnowledgeBase) {
    loading.value = true;
    error.value = null;
    
    try {
      const updatedKnowledgeBase = await updateKnowledgeBase(knowledgeBaseData);
      const index = knowledgeBases.value.findIndex(kb => kb.id === updatedKnowledgeBase.id);
      if (index !== -1) {
        knowledgeBases.value[index] = updatedKnowledgeBase;
      }
      return updatedKnowledgeBase;
    } catch (err) {
      console.error(`Failed to update knowledge base with id ${knowledgeBaseData.id}:`, err);
      error.value = '更新知识库失败';
      throw err;
    } finally {
      loading.value = false;
    }
  }
  
  // 删除知识库
  async function removeKnowledgeBase(id: string) {
    loading.value = true;
    error.value = null;
    
    try {
      const success = await deleteKnowledgeBase(id);
      if (success) {
        knowledgeBases.value = knowledgeBases.value.filter(kb => kb.id !== id);
      }
      return success;
    } catch (err) {
      console.error(`Failed to delete knowledge base with id ${id}:`, err);
      error.value = '删除知识库失败';
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
    knowledgeBases,
    loading,
    error,
    needRefresh,
    fetchAllKnowledgeBases,
    fetchKnowledgeBaseById,
    createKnowledgeBase,
    modifyKnowledgeBase,
    removeKnowledgeBase,
    setNeedRefresh
  };
});
