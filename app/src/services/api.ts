import { Agent, AgentCategory, Tool, ToolCategory } from './typings';
import { mockKnowledgeBases } from './mock/knowledgeData';
import { KnowledgeBase, KnowledgeBaseCategory } from './typings';
import { Provider } from './typings';

// 导入Tauri API
import * as tauriApi from './tauriApi';

// 获取所有智能体分类
export async function getAgentCategories(): Promise<AgentCategory[]> {
  return tauriApi.fetch_local('agent.category.list', null) as Promise<AgentCategory[]>;
}

// 添加智能体类别
export async function addAgentCategory(category: Pick<AgentCategory, 'name'>): Promise<boolean> {
  const timestamp = Date.now();
  const random = Math.floor(Math.random() * 1000);
  const newId = timestamp * 1000 + random;
  
  const newCategory: AgentCategory = {
    id: newId,
    name: category.name,
    createdAt: timestamp,
  };

  return tauriApi.fetch_local('agent.category.add', newCategory) as Promise<boolean>;
}

// 删除智能体类别
export async function deleteAgentCategory(categoryId: number): Promise<boolean> {
  return tauriApi.fetch_local('agent.category.delete', categoryId) as Promise<boolean>;
}

// 获取所有智能体
export async function getAllAgents(): Promise<Agent[]> {
  return tauriApi.fetch_local('agent.list', null) as Promise<Agent[]>;
}

// 添加智能体
export async function addAgent(agent: Omit<Agent, 'id' | 'createdAt'>): Promise<boolean> {
  const timestamp = Date.now();
  const random = Math.floor(Math.random() * 1000);
  const newId = timestamp * 1000 + random;
  
  const newAgent: Agent = {
    id: newId,
    createdAt: timestamp,
    ...agent
  };
  return tauriApi.fetch_local('agent.add', newAgent) as Promise<boolean>;
}

// 更新智能体
export async function updateAgent(agent: Agent): Promise<boolean> {
  const timestamp = Date.now();
  const updatedAgent = {
    ...agent,
    updatedAt: timestamp
  };
  return tauriApi.fetch_local('agent.update', updatedAgent) as Promise<boolean>;
}

// 删除智能体
export async function deleteAgent(id: number): Promise<boolean> {
  return tauriApi.fetch_local('agent.delete', id) as Promise<boolean>;
}

// 通过类别删除智能体
export async function deleteAgentByCategory(categoryId: number): Promise<boolean> {
  return tauriApi.fetch_local('agent.delete.by.category', categoryId) as Promise<boolean>;
}

// 工具相关API接口
// 获取所有工具类别
export async function getToolCategories(): Promise<ToolCategory[]> {
  return tauriApi.fetch_local('tool.category.list', null) as Promise<ToolCategory[]>;
}

// 获取所有工具
export async function getAllTools(): Promise<Tool[]> {
  return tauriApi.fetch_local('tool.list', null) as Promise<Tool[]>;
}

// 通过ID获取特定工具
export async function getToolById(id: number): Promise<Tool | null> {
  return tauriApi.fetch_local('tool.get', id) as Promise<Tool | null>;
}

// 添加工具类别
export async function addToolCategory(category: Pick<ToolCategory, 'name'>): Promise<boolean> {
  const timestamp = Date.now();
  const random = Math.floor(Math.random() * 1000);
  const newId = timestamp * 1000 + random;
  
  const newCategory: ToolCategory = {
    id: newId,
    name: category.name,
    createdAt: timestamp
  };
  
  return tauriApi.fetch_local('tool.category.add', newCategory) as Promise<boolean>;
}

// 删除工具类别
export async function deleteToolCategory(categoryId: number): Promise<boolean> {
  return tauriApi.fetch_local('tool.category.delete', categoryId) as Promise<boolean>;
}

// 添加工具
export async function addTool(tool: Omit<Tool, 'id' | 'createdAt' | 'updatedAt'>): Promise<boolean> {
  const timestamp = Date.now();
  const random = Math.floor(Math.random() * 1000);
  const newId = timestamp * 1000 + random;
  
  const newTool: Tool = {
    ...tool,
    id: newId,
    createdAt: timestamp
  };
  
  return tauriApi.fetch_local('tool.add', newTool) as Promise<boolean>;
}

// 更新工具
export async function updateTool(tool: Tool): Promise<boolean> {
  const timestamp = Date.now();
  const updatedTool = {
    ...tool,
    updatedAt: timestamp
  };
  
  return tauriApi.fetch_local('tool.update', updatedTool) as Promise<boolean>;
}

// 删除工具
export async function deleteTool(id: number): Promise<boolean> {
  return tauriApi.fetch_local('tool.delete', id) as Promise<boolean>;
}

// 根据类别删除工具
export async function deleteToolByCategory(categoryId: number): Promise<boolean> {
  return tauriApi.fetch_local('tool.delete.by.category', categoryId) as Promise<boolean>;
}

// 测试工具
export async function testTool(tool: Tool): Promise<string> {
  return tauriApi.fetch_local('tool.test', tool) as Promise<string>;
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
  return tauriApi.fetch_local('provider.list', null) as Promise<Provider[]>;
}

// 获取单个模型提供商详情
export async function getProviderById(id: number): Promise<Provider | null> {
  return tauriApi.fetch_local('provider.get', {id}) as Promise<Provider>;
}

// 添加模型提供商
export async function addProvider(providerData: Omit<Provider, 'id'>): Promise<boolean> {
  const timestamp = Date.now();
  const random = Math.floor(Math.random() * 1000);
  const newId = timestamp * 1000 + random;

  const newProvider = {
    id: newId,
    createdAt: timestamp,
    ...providerData
  };
  return tauriApi.fetch_local('provider.add', newProvider) as Promise<boolean>;
}

// 更新模型提供商
export async function updateProvider(providerData: Provider): Promise<boolean> {
  return tauriApi.fetch_local('provider.update', providerData) as Promise<boolean>;
}

// 删除模型提供商
export async function deleteProvider(id: number): Promise<boolean> {
  return tauriApi.fetch_local('provider.delete', id) as Promise<boolean>;
}

// 模拟流式输出的聊天接口（逐字输出，间隔10毫秒）
export function simulateChatStream(input: string, onData: (chunk: string) => void, delay = 100): Promise<void> {
  const response = `作为AI助手，我收到了您的消息："${input}"。让我来为您解答。这是一个模拟的流式输出示例。希望这个回答对您有帮助！`;

  // 延迟2秒后开始输出
  return new Promise(async (resolve) => {
    setTimeout(() => {
      for (const char of response) {
        // 模拟逐字延迟
        setTimeout(() => {
          onData(char);
        }, delay);
        delay += 10; // 每个字符延迟10毫秒
      }
      resolve();
    }, 2000); // 延迟2秒后开始输出
  });
}

// 导出Tauri相关API
export const isTauriAvailable = tauriApi.isTauriAvailable;
export const safeTauriCall = tauriApi.safeTauriCall;
