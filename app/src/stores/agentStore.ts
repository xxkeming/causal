import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { Agent } from '../services/typings';
import { 
  getAllAgents, getAgentsByCategory, getAgentById,
  addAgent, updateAgent, deleteAgent 
} from '../services/api';

export const useAgentStore = defineStore('agent', () => {
  // 状态
  const agents = ref<Agent[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const selectedCategoryId = ref<string>('all');
  const searchKeyword = ref<string>('');
  const needRefresh = ref(false);
  
  // 计算属性：按搜索关键词过滤后的智能体
  const displayAgents = computed(() => {
    if (!searchKeyword.value) {
      return agents.value;
    }
    
    const keyword = searchKeyword.value.toLowerCase();
    return agents.value.filter(agent => 
      agent.name.toLowerCase().includes(keyword) || 
      (agent.description && agent.description.toLowerCase().includes(keyword))
    );
  });
  
  // 获取所有智能体
  async function fetchAllAgents() {
    loading.value = true;
    error.value = null;
    
    try {
      agents.value = await getAllAgents();
      needRefresh.value = false;
    } catch (err) {
      console.error('Failed to fetch agents:', err);
      error.value = '获取智能体失败';
    } finally {
      loading.value = false;
    }
  }
  
  // 按分类获取智能体
  async function fetchAgentsByCategory(categoryId: string) {
    loading.value = true;
    error.value = null;
    selectedCategoryId.value = categoryId;
    
    try {
      agents.value = await getAgentsByCategory(categoryId);
      needRefresh.value = false;
    } catch (err) {
      console.error(`Failed to fetch agents for category ${categoryId}:`, err);
      error.value = '按分类获取智能体失败';
    } finally {
      loading.value = false;
    }
  }
  
  // 获取单个智能体详情
  async function fetchAgentById(id: string) {
    loading.value = true;
    error.value = null;
    
    try {
      const agent = await getAgentById(id);
      return agent;
    } catch (err) {
      console.error(`Failed to fetch agent with id ${id}:`, err);
      error.value = '获取智能体详情失败';
      return null;
    } finally {
      loading.value = false;
    }
  }
  
  // 创建智能体
  async function createAgent(agentData: Omit<Agent, 'id' | 'createdAt'>) {
    loading.value = true;
    error.value = null;
    
    try {
      // 检查是否同名
      const exists = agents.value.some(a => a.name === agentData.name);
      if (exists) {
        throw('已存在同名智能体');
      }

      const newAgent = await addAgent(agentData);
      agents.value.push(newAgent);
      
      return newAgent;
    } catch (err) {
      console.error('Failed to create agent:', err);
      error.value = '创建智能体失败';
      throw err;
    } finally {
      loading.value = false;
    }
  }
  
  // 更新智能体
  async function modifyAgent(agentData: Agent) {
    loading.value = true;
    error.value = null;
    
    try {
      const updatedAgent = await updateAgent(agentData);
      
      // 更新本地列表中的对应项
      const index = agents.value.findIndex(a => a.id === updatedAgent.id);
      if (index !== -1) {
        agents.value[index] = updatedAgent;
      }
      
      return updatedAgent;
    } catch (err) {
      console.error(`Failed to update agent with id ${agentData.id}:`, err);
      error.value = '更新智能体失败';
      throw err;
    } finally {
      loading.value = false;
    }
  }
  
  // 删除智能体
  async function removeAgent(id: string) {
    loading.value = true;
    error.value = null;
    
    try {
      const success = await deleteAgent(id);
      
      if (success) {
        // 从本地列表中移除
        agents.value = agents.value.filter(a => a.id !== id);
      }
      
      return success;
    } catch (err) {
      console.error(`Failed to delete agent with id ${id}:`, err);
      error.value = '删除智能体失败';
      return false;
    } finally {
      loading.value = false;
    }
  }
  
  // 设置搜索关键词
  function setSearchKeyword(keyword: string) {
    searchKeyword.value = keyword;
  }
  
  // 设置已选分类
  function selectCategory(categoryId: string) {
    selectedCategoryId.value = categoryId;
  }
  
  // 设置需要刷新标志
  function setNeedRefresh() {
    needRefresh.value = true;
  }
  
  return {
    agents,
    loading,
    error,
    searchKeyword,
    selectedCategoryId,
    needRefresh,
    displayAgents,
    fetchAllAgents,
    fetchAgentsByCategory,
    fetchAgentById,
    createAgent,
    modifyAgent,
    removeAgent,
    setSearchKeyword,
    selectCategory,
    setNeedRefresh
  };
});
