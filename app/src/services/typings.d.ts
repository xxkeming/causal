// @ts-ignore
/* eslint-disable */

/*
const ProviderApiCategory = ['OpenAI', "Gemini", "Anthropic", "Xai", "Ollama", "DeepSeek"];
*/

// 模型名, 标签包含 [推理,向量,图片]
export interface ProviderModel {
  name: string;
  tags: string[];
}

// 模型提供商
export interface Provider {
  id: number;

  // 自定义名称
  name: string;

  // 对应ApiCategory里面的name
  apiCategory: string;

  url: string;
  apiKey?: string;

  models?: ProviderModel[];
}

// 模型
export interface Model {
  // id 为providerId
  id: number;
  // name 为模型名称
  name: string;
}

// 模型自定义参数
export interface ModelParam {
  // 参数名称
  name: string;
  // 参数类型 string, number, boolean, object
  type: string;
  // 参数值
  value: string;
}

// 智能体
export interface Agent {
  id: string;
  categoryId: string;

  // 图标Id // 通过stores/iconsStore.ts中的icons获取 getIconById(agent.iconId)
  iconId?: number;

  // 名称
  name: string;

  // 描述
  description?: string;

  // 模型通过providerId和name获取
  model?: Model;

  // 提示词
  prompt: string;

  // 模型生成文本的随机程度。值越大，回复内容越赋有多样性、创造性、随机性；设为 0 根据事实回答。日常聊天建议设置为 0.7
  // 默认1, 0 - 2
  temperature: number;

  // 值越小，AI 生成的内容越单调，也越容易理解；值越大，AI 回复的词汇围越大，越多样化
  // 默认1, 0 - 1
  topP: number;

  // 生成的回复中包含的 token 数量，值越大，回复内容越多，但可能会变得不连贯
  // 默认60
  topK: number;

  // 最大消息长度, 默认0, 0 不限制
  maxTokens: number;

  // 要保留在上下文中的消息数量，数值越大，上下文越长，消耗的 token 越多。普通聊天建议 5-10
  // 默认 5
  contextSize: number;

  // 自定义参数
  params?: ModelParam[];

  // 工具集合
  tools?: number[];

  createdAt: string;
  updatedAt?: string;
}

// 智能体类别
export interface AgentCategory {
  id: string;
  name: string;
}



// 工具类别
export interface ToolCategory {
  id: number;
  name: string;
}

// 工具参数
export interface ToolParam {
  // 参数名称
  name: string;
  // 参数类型 string, number, boolean, object
  type: string;
  // 参数描述
  description: string;
  // 是否必填, 默认否
  required: boolean;
  // 测试值
  testValue?: string;
}

// 工具
export interface Tool {
  id: number;
  categoryId?: number;

  // 图标Id // 通过stores/iconsStore.ts中的icons获取 getIconById(agent.iconId)
  iconId?: number;

  name: string;
  description: string;
  param?: ToolParam[];
  
  // js代码, 执行工具的代码, 需要编辑
  code: string;

  createdAt: string;
  updatedAt?: string;
}

// 知识库类型, 目前就固定的2个类型
// 1. 通用知识库
// 2. 站点同步
export interface KnowledgeBaseCategory {
  id: string;
  name: string;
}

// 知识库
export interface KnowledgeBase {
  id: string;
  categoryId: string;

  // 图标Id // 通过stores/iconsStore.ts中的icons获取 getIconById(agent.iconId)
  iconId?: number;

  name: string;
  description: string;
  
  // categoryId = 2, 才有站点地址数据
  site?: string;

  createdAt: string;
  updatedAt?: string;
}

// 知识库文档
export interface KnowledgeBaseDocument {
  id: string;
  knowledgeBaseId: string;
  title: string;
  content: string;
  createdAt: string;
  updatedAt?: string;
}

// 聊天信息
export interface ChatMessage {
  // 会话ID
  sessionId: number;

  role: 'user' | 'assistant' | 'system';
  content: string;
  status?: 'sending' | 'success' | 'error';
  timestamp?: Date;
}

// 聊天会话信息
export interface ChatSession {
  // 会话id
  id: number;

  // 智能体id
  agentId: string;

  // 会话主题
  topic: string;

  createdAt: string;
  updatedAt?: string;
}

// API 响应格式
export interface ApiResponse<T = any> {
  code: number;
  message: string;
  data?: T;
}