import { defineStore } from 'pinia';
import { ref } from 'vue';
import { Agent } from '../services/typings';
import { useGlobalStore } from './globalStore';
import { 
  getAllAgents,
  addAgent, updateAgent, deleteAgent, deleteAgentByCategory
} from '../services/api';

export const useAgentStore = defineStore('agent', () => {
  const globalStore = useGlobalStore();
  
  // 状态
  const agents = ref<Agent[]>([]);
  const error = ref<string | null>(null);
  
  // 获取所有智能体 - 作为主要的数据获取方法
  async function fetchAllAgents() {
    globalStore.setLoadingState(true);
    error.value = null;
    
    try {
      if (agents.value.length === 0) {
        agents.value = await getAllAgents();
      }
    } catch (err) {
      console.error('Failed to fetch agents:', err);
      error.value = '获取智能体失败';
    } finally {
      globalStore.setLoadingState(false);
    }
  }
  
  // 获取单个智能体详情
  async function fetchAgentById(id: number) {
    globalStore.setLoadingState(true);
    error.value = null;
    
    try {
      const agentIndex = agents.value.findIndex(a => a.id === id);
      if (agentIndex === -1) {
        // 如果已经在本地列表中，直接返回
        throw new Error(`Agent with id ${id} not found in local list`);
      }

      return agents.value[agentIndex];
    } catch (err) {
      console.error(`Failed to fetch agent with id ${id}:`, err);
      error.value = '获取智能体详情失败';
      return null;
    } finally {
      globalStore.setLoadingState(false);
    }
  }
  
  // 创建智能体
  async function createAgent(agentData: Omit<Agent, 'id' | 'createdAt'>) {
    globalStore.setLoadingState(true);
    error.value = null;
    
    try {
      // 检查是否同名
      const exists = agents.value.some(a => a.name === agentData.name);
      if (exists) {
        throw('已存在同名智能体');
      }

      await addAgent(agentData);
      agents.value = await getAllAgents();
    } catch (err) {
      console.error('Failed to create agent:', err);
      error.value = '创建智能体失败';
      throw err;
    } finally {
      globalStore.setLoadingState(false);
    }
  }
  
  // 更新智能体
  async function modifyAgent(agentData: Agent) {
    globalStore.setLoadingState(true);
    error.value = null;
    
    try {
      const index = agents.value.findIndex(a => a.id === agentData.id);
      if (index === -1) {
        throw new Error(`Agent with id ${agentData.id} not found in local list`);
      }

      const newAgentData = {
        ...agents.value[index],
        ...agentData
      };
      await updateAgent(newAgentData);
      
      // 更新本地列表中的对应项
      agents.value[index] = newAgentData;
    } catch (err) {
      console.error(`Failed to update agent with id ${agentData.id}:`, err);
      error.value = '更新智能体失败';
      throw err;
    } finally {
      globalStore.setLoadingState(false);
    }
  }
  
  // 删除智能体
  async function removeAgent(id: number) {
    globalStore.setLoadingState(true);
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
      globalStore.setLoadingState(false);
    }
  }
  
  // 通过类别删除智能体
  async function removeAgentByCategory(categoryId: number) {
    globalStore.setLoadingState(true);
    error.value = null;
    
    try {
      await deleteAgentByCategory(categoryId);
      
      // 从本地列表中移除
      agents.value = agents.value.filter(a => a.categoryId !== categoryId);
    } catch (err) {
      console.error(`Failed to delete agents by category ${categoryId}:`, err);
      error.value = '删除智能体失败';
    } finally {
      globalStore.setLoadingState(false);
    }
  }
  return {
    agents,
    loading: globalStore.isLoading,
    error,
    fetchAllAgents,
    fetchAgentById,
    createAgent,
    modifyAgent,
    removeAgent,
    removeAgentByCategory
  };
});
