import { defineStore } from 'pinia';
import { ref } from 'vue';
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
      await addProvider(providerData);
      providers.value = await getAllProviders();
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
      await updateProvider(providerData);
      const index = providers.value.findIndex(p => p.id === providerData.id);
      if (index !== -1) {
        providers.value[index] = providerData;
      }
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
  
  return {
    providers,
    loading,
    error,
    fetchAllProviders,
    fetchProviderById,
    createProvider,
    modifyProvider,
    removeProvider
  };
});
