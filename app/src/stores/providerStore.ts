import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { Provider } from '../services/typings';
import { 
  getAllProviders, getProviderById, addProvider, 
  updateProvider, deleteProvider 
} from '../services/api';

// 添加模型提供商 API 类别常量, 先支持OpenAI 'OpenAI', "Gemini", "Anthropic", "Xai", "Ollama", "DeepSeek"
export const ProviderApiCategory = ['OpenAI'];

export const useProviderStore = defineStore('provider', () => {
  // 状态
  const providers = ref<Provider[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const searchKeyword = ref<string>('');
  
  // 计算属性：按搜索关键词过滤后的提供商
  const displayProviders = computed(() => {
    if (!searchKeyword.value) {
      return providers.value;
    }
    
    const keyword = searchKeyword.value.toLowerCase();
    return providers.value.filter(provider => 
      provider.name.toLowerCase().includes(keyword) || 
      provider.apiCategory.toLowerCase().includes(keyword)
    );
  });
  
  // 获取所有提供商
  async function fetchAllProviders() {
    loading.value = true;
    error.value = null;
    
    try {
      if (providers.value.length === 0) {
        providers.value = await getAllProviders();
      }
    } catch (err) {
      console.error('Failed to fetch providers:', err);
      error.value = '获取模型提供商失败';
    } finally {
      loading.value = false;
    }
  }
  
  // 获取单个提供商详情
  // 先从providers.value中查找，如果没有再调用API
  async function fetchProviderById(id: number) {
    loading.value = true;
    error.value = null;
    
    try {
      const provider = providers.value.find(p => p.id === id);
      if (provider) {
        return provider;
      }
      return await getProviderById(id);
    } catch (err) {
      console.error(`Failed to fetch provider with id ${id}:`, err);
      error.value = '获取模型提供商详情失败';
      return null;
    } finally {
      loading.value = false;
    }
  }
  
  // 添加提供商
  async function createProvider(providerData: Omit<Provider, 'id'>) {
    loading.value = true;
    error.value = null;
    
    try {
      const newProvider = await addProvider(providerData);
      providers.value.push(newProvider);
      return newProvider;
    } catch (err) {
      console.error('Failed to create provider:', err);
      error.value = '创建模型提供商失败';
      throw err;
    } finally {
      loading.value = false;
    }
  }
  
  // 更新提供商
  async function modifyProvider(providerData: Provider) {
    loading.value = true;
    error.value = null;
    
    try {
      const updatedProvider = await updateProvider(providerData);
      const index = providers.value.findIndex(p => p.id === updatedProvider.id);
      if (index !== -1) {
        providers.value[index] = updatedProvider;
      }
      return updatedProvider;
    } catch (err) {
      console.error(`Failed to update provider with id ${providerData.id}:`, err);
      error.value = '更新模型提供商失败';
      throw err;
    } finally {
      loading.value = false;
    }
  }
  
  // 删除提供商
  async function removeProvider(id: number) {
    loading.value = true;
    error.value = null;
    
    try {
      const success = await deleteProvider(id);
      if (success) {
        providers.value = providers.value.filter(p => p.id !== id);
      }
      return success;
    } catch (err) {
      console.error(`Failed to delete provider with id ${id}:`, err);
      error.value = '删除模型提供商失败';
      return false;
    } finally {
      loading.value = false;
    }
  }
  
  // 设置搜索关键词
  function setSearchKeyword(keyword: string) {
    searchKeyword.value = keyword;
  }
  
  return {
    providers,
    loading,
    error,
    searchKeyword,
    displayProviders,
    fetchAllProviders,
    fetchProviderById,
    createProvider,
    modifyProvider,
    removeProvider,
    setSearchKeyword
  };
});
