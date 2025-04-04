import { Agent, AgentCategory, Tool, ToolCategory } from './typings';
import { mockAgents, mockCategories } from './mock/agentData';
import { mockTools, mockToolCategories } from './mock/toolData';
import { mockKnowledgeBases } from './mock/knowledgeData';
import { KnowledgeBase, KnowledgeBaseCategory } from './typings';
import { Provider } from './typings';

// 导入Tauri API
import * as tauriApi from './tauriApi';

// 导入模型提供商数据
import { mockProviders } from './mock/providerData';

// 获取所有智能体分类
export async function getAgentCategories(): Promise<AgentCategory[]> {
  return new Promise((resolve) => {
    resolve(structuredClone(mockCategories));
  });
}

// 根据分类获取智能体
export async function getAgentsByCategory(categoryId: string): Promise<Agent[]> {
  return new Promise((resolve) => {
    if (categoryId === 'all') {
      resolve(mockAgents);
    } else {
      const filteredAgents = mockAgents.filter(agent => agent.categoryId === categoryId);
      resolve(filteredAgents);
    }
  });
}

// 获取所有智能体
export async function getAllAgents(): Promise<Agent[]> {
  return new Promise((resolve) => {
    resolve(mockAgents);
  });
}

// 获取单个智能体详情
export async function getAgentById(id: string): Promise<Agent | null> {
  return new Promise((resolve) => {
    const agent = mockAgents.find(a => a.id === id);
    resolve(agent ? agent : null);
  });
}

// 添加智能体
export async function addAgent(agent: Omit<Agent, 'id' | 'createdAt'>): Promise<Agent> {
  return new Promise((resolve) => {
    const newId = String(Math.max(...mockAgents.map(a => parseInt(a.id))) + 1);
    const createdAt = new Date().toISOString();
    
    const newAgent: Agent = {
      id: newId,
      createdAt,
      ...agent
    };
    
    mockAgents.push(newAgent);
    resolve(newAgent);
  });
}

// 更新智能体
export async function updateAgent(agent: Agent): Promise<Agent> {
  return new Promise((resolve, reject) => {
    const index = mockAgents.findIndex(a => a.id === agent.id);
    
    if (index === -1) {
      reject(new Error('Agent not found'));
      return;
    }
    
    const updatedAgent = {
      ...mockAgents[index],
      ...agent,
      updatedAt: new Date().toISOString()
    };
    
    mockAgents[index] = updatedAgent;
    resolve(updatedAgent);
  });
}

// 删除智能体
export async function deleteAgent(id: string): Promise<boolean> {
  return new Promise((resolve) => {
    // const initialLength = mockAgents.length;
    const index = mockAgents.findIndex(a => a.id === id);
    
    if (index !== -1) {
      mockAgents.splice(index, 1);
      resolve(true);
    } else {
      resolve(false);
    }
  });
}

// 添加智能体类别
export async function addAgentCategory(category: Pick<AgentCategory, 'name'>): Promise<AgentCategory> {
  return new Promise((resolve) => {
    // 确保ID是唯一的
    const newCategory: AgentCategory = {
      id: `${mockCategories.length + 1}`,
      name: category.name
    };
    // 添加到模拟数据中
    mockCategories.push(newCategory);
    resolve(structuredClone(newCategory));
  });
}

// 删除智能体类别
export async function deleteAgentCategory(categoryId: string): Promise<boolean> {
  return new Promise((resolve) => {
    const index = mockCategories.findIndex(cat => cat.id === categoryId);
    if (index !== -1) {
      mockCategories.splice(index, 1);
      resolve(true);
    } else {
      resolve(false);
    }
  });
}

// 工具相关API接口
// 获取所有工具类别
export async function getToolCategories(): Promise<ToolCategory[]> {
  return new Promise((resolve) => {
    resolve(structuredClone(mockToolCategories));
  });
}

// 根据分类获取工具
export async function getToolsByCategory(categoryId: number): Promise<Tool[]> {
  return new Promise((resolve) => {
    const filteredTools = mockTools.filter(tool => tool.categoryId === categoryId);
    resolve(filteredTools);
  });
}

// 获取所有工具
export async function getAllTools(): Promise<Tool[]> {
  return new Promise((resolve) => {
    resolve(mockTools);
  });
}

// 通过ID获取特定工具
export async function getToolById(id: number): Promise<Tool | null> {
  return new Promise((resolve) => {
    const tool = mockTools.find(tool => tool.id === id);
    resolve(tool || null);
  });
}

// 添加工具类别
export async function addToolCategory(category: Pick<ToolCategory, 'name'>): Promise<ToolCategory> {
  return new Promise((resolve) => {
    // 确保ID是唯一的，使用毫秒时间戳 + 随机数确保唯一性
    const timestamp = Date.now();
    const random = Math.floor(Math.random() * 1000);
    const newId = timestamp * 1000 + random;
    
    // 先检查是否已存在同名类别
    const existingCategory = mockToolCategories.find(cat => cat.name === category.name);
    if (existingCategory) {
      // 如果存在同名类别，直接返回现有类别
      resolve(existingCategory);
      return;
    }
    
    // 如果不存在同名类别，创建新类别
    const newCategory: ToolCategory = {
      id: newId,
      name: category.name
    };
    
    // 添加到模拟数据中
    mockToolCategories.push(newCategory);
    resolve(newCategory);
  });
}

// 删除工具类别
export async function deleteToolCategory(categoryId: number): Promise<boolean> {
  return new Promise((resolve) => {
    const index = mockToolCategories.findIndex(cat => cat.id === categoryId);
    if (index !== -1) {
      mockToolCategories.splice(index, 1);
      resolve(true);
    } else {
      resolve(false);
    }
  });
}

// 添加工具
export async function addTool(tool: Omit<Tool, 'id' | 'createdAt' | 'updatedAt'>): Promise<Tool> {
  return new Promise((resolve) => {
    const newTool: Tool = {
      ...tool,
      id: mockTools.length + 1,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString()
    };
    mockTools.push(newTool);
    resolve(newTool);
  });
}

// 更新工具
export async function updateTool(tool: Tool): Promise<Tool> {
  return new Promise((resolve, reject) => {
    const index = mockTools.findIndex(t => t.id === tool.id);
    if (index !== -1) {
      mockTools[index] = {
        ...tool,
        updatedAt: new Date().toISOString()
      };
      resolve(mockTools[index]);
    } else {
      reject(new Error('Tool not found'));
    }
  });
}

// 删除工具
export async function deleteTool(id: number): Promise<boolean> {
  return new Promise((resolve) => {
    const index = mockTools.findIndex(tool => tool.id === id);
    if (index !== -1) {
      mockTools.splice(index, 1);
      resolve(true);
    } else {
      resolve(false);
    }
  });
}

// 模拟固定的知识库类别
const mockKnowledgeBaseCategories: KnowledgeBaseCategory[] = [
  {
    id: "1",
    name: "通用知识库"
  },
  {
    id: "2",
    name: "站点同步"
  }
];

// 知识库API
// 获取所有知识库
export async function getAllKnowledgeBases(): Promise<KnowledgeBase[]> {
  
  return [...mockKnowledgeBases]; // 返回副本以避免引用问题
}

// 按类别获取知识库
export async function getKnowledgeBasesByCategory(categoryId: string): Promise<KnowledgeBase[]> {
  return mockKnowledgeBases.filter(kb => kb.categoryId === categoryId);
}

// 获取单个知识库详情
export async function getKnowledgeBaseById(id: string): Promise<KnowledgeBase> {
  const knowledgeBase = mockKnowledgeBases.find(kb => kb.id === id);
  
  if (!knowledgeBase) {
    throw new Error(`Knowledge base with ID ${id} not found`);
  }
  
  return { ...knowledgeBase }; // 返回副本以避免引用问题
}

// 添加知识库
export async function addKnowledgeBase(knowledgeBaseData: Omit<KnowledgeBase, 'id' | 'createdAt' | 'updatedAt'>): Promise<KnowledgeBase> {
  const newId = String(Math.max(...mockKnowledgeBases.map(kb => parseInt(kb.id))) + 1);
  const now = new Date().toISOString();
  
  const newKnowledgeBase: KnowledgeBase = {
    id: newId,
    ...knowledgeBaseData,
    createdAt: now
  };
  
  mockKnowledgeBases.push(newKnowledgeBase);
  
  return { ...newKnowledgeBase };
}

// 更新知识库
export async function updateKnowledgeBase(knowledgeBaseData: KnowledgeBase): Promise<KnowledgeBase> {
  const index = mockKnowledgeBases.findIndex(kb => kb.id === knowledgeBaseData.id);
  
  if (index === -1) {
    throw new Error(`Knowledge base with ID ${knowledgeBaseData.id} not found`);
  }
  
  const now = new Date().toISOString();
  const updatedKnowledgeBase = {
    ...mockKnowledgeBases[index],
    ...knowledgeBaseData,
    updatedAt: now
  };
  
  mockKnowledgeBases[index] = updatedKnowledgeBase;
  
  return { ...updatedKnowledgeBase };
}

// 删除知识库
export async function deleteKnowledgeBase(id: string): Promise<boolean> {
  const index = mockKnowledgeBases.findIndex(kb => kb.id === id);
  
  if (index !== -1) {
    mockKnowledgeBases.splice(index, 1);
    return true;
  }
  
  return false;
}

// 获取知识库类别（固定两种）
export async function getKnowledgeBaseCategories(): Promise<KnowledgeBaseCategory[]> {
  return [...mockKnowledgeBaseCategories]; // 返回副本以避免引用问题
}

// 模型提供商相关 API
// 获取所有模型提供商
export async function getAllProviders(): Promise<Provider[]> {
  return new Promise((resolve) => {
    resolve([...mockProviders]); // 返回副本以避免引用问题
  });
}

// 获取单个模型提供商详情
export async function getProviderById(id: number): Promise<Provider | null> {
  return new Promise((resolve) => {
    const provider = mockProviders.find(p => p.id === id);
    resolve(provider ? {...provider} : null);
  });
}

// 添加模型提供商
export async function addProvider(providerData: Omit<Provider, 'id'>): Promise<Provider> {
  return new Promise((resolve) => {
    const newId = Math.max(0, ...mockProviders.map(p => p.id)) + 1;
    const newProvider = {
      id: newId,
      ...providerData
    };
    mockProviders.push(newProvider);
    resolve({...newProvider});
  });
}

// 更新模型提供商
export async function updateProvider(providerData: Provider): Promise<Provider> {
  return new Promise((resolve, reject) => {
    const index = mockProviders.findIndex(p => p.id === providerData.id);
    if (index !== -1) {
      mockProviders[index] = {...providerData};
      resolve({...mockProviders[index]});
    } else {
      reject(new Error('Provider not found'));
    }
  });
}

// 删除模型提供商
export async function deleteProvider(id: number): Promise<boolean> {
  return new Promise((resolve) => {
    const index = mockProviders.findIndex(p => p.id === id);
    if (index !== -1) {
      mockProviders.splice(index, 1);
      resolve(true);
    } else {
      resolve(false);
    }
  });
}

// 导出Tauri相关API
export const greet = tauriApi.greet;
export const isTauriAvailable = tauriApi.isTauriAvailable;
export const safeTauriCall = tauriApi.safeTauriCall;
