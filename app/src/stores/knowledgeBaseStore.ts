import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { KnowledgeBase } from '../services/typings';
import { 
  getAllKnowledgeBases, getKnowledgeBasesByCategory, getKnowledgeBaseById,
  addKnowledgeBase, updateKnowledgeBase, deleteKnowledgeBase
} from '../services/api';

export const useKnowledgeBaseStore = defineStore('knowledgeBase', () => {
  // 状态
  const knowledgeBases = ref<KnowledgeBase[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const selectedCategoryId = ref<string | 'all'>('all');
  const searchKeyword = ref<string>('');
  // 添加刷新标志
  const needRefresh = ref(false);
  
  // 计算属性：按类别过滤后的知识库
  const filteredKnowledgeBases = computed(() => {
    if (selectedCategoryId.value === 'all') {
      return knowledgeBases.value;
    }
    return knowledgeBases.value.filter(kb => kb.categoryId === selectedCategoryId.value);
  });
  
  // 计算属性：按搜索关键词过滤后的知识库
  const displayKnowledgeBases = computed(() => {
    if (!searchKeyword.value) {
      return filteredKnowledgeBases.value;
    }
    
    const keyword = searchKeyword.value.toLowerCase();
    return filteredKnowledgeBases.value.filter(kb => 
      kb.name.toLowerCase().includes(keyword) || 
      (kb.description && kb.description.toLowerCase().includes(keyword))
    );
  });
  
  // 获取所有知识库
  async function fetchAllKnowledgeBases() {
    loading.value = true;
    error.value = null;
    
    try {
      console.log("正在获取所有知识库数据...");
      knowledgeBases.value = await getAllKnowledgeBases();
      console.log(`获取到 ${knowledgeBases.value.length} 个知识库`);
      // 重置刷新标志
      needRefresh.value = false;
    } catch (err) {
      console.error('Failed to fetch knowledge bases:', err);
      error.value = '获取知识库失败';
    } finally {
      loading.value = false;
    }
  }
  
  // 按类别获取知识库
  async function fetchKnowledgeBasesByCategory(categoryId: string | 'all') {
    loading.value = true;
    error.value = null;
    
    try {
      if (categoryId === 'all') {
        await fetchAllKnowledgeBases();
      } else {
        knowledgeBases.value = await getKnowledgeBasesByCategory(categoryId);
      }
      selectedCategoryId.value = categoryId;
    } catch (err) {
      console.error('Failed to fetch knowledge bases by category:', err);
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
  
  // 添加知识库
  async function createKnowledgeBase(kbData: Omit<KnowledgeBase, 'id' | 'createdAt' | 'updatedAt'>) {
    loading.value = true;
    error.value = null;
    
    try {
      const newKnowledgeBase = await addKnowledgeBase(kbData);
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
  async function modifyKnowledgeBase(kbData: KnowledgeBase) {
    loading.value = true;
    error.value = null;
    
    try {
      const updatedKnowledgeBase = await updateKnowledgeBase(kbData);
      const index = knowledgeBases.value.findIndex(kb => kb.id === updatedKnowledgeBase.id);
      if (index !== -1) {
        knowledgeBases.value[index] = updatedKnowledgeBase;
      }
      return updatedKnowledgeBase;
    } catch (err) {
      console.error(`Failed to update knowledge base with id ${kbData.id}:`, err);
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
  
  // 设置搜索关键词
  function setSearchKeyword(keyword: string) {
    searchKeyword.value = keyword;
  }
  
  // 设置选中的类别
  function selectCategory(categoryId: string | 'all') {
    selectedCategoryId.value = categoryId;
  }
  
  // 设置是否需要刷新
  function setNeedRefresh(value: boolean = true) {
    needRefresh.value = value;
  }
  
  return {
    knowledgeBases,
    loading,
    error,
    selectedCategoryId,
    searchKeyword,
    needRefresh,
    filteredKnowledgeBases,
    displayKnowledgeBases,
    fetchAllKnowledgeBases,
    fetchKnowledgeBasesByCategory,
    fetchKnowledgeBaseById,
    createKnowledgeBase,
    modifyKnowledgeBase,
    removeKnowledgeBase,
    setSearchKeyword,
    selectCategory,
    setNeedRefresh
  };
});
