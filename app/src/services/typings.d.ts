// @ts-ignore
/* eslint-disable */

/*
const ProviderApiCategory = ['OpenAI', "Gemini", "Anthropic", "Xai", "Ollama", "DeepSeek"];
*/

// 模型名, 标签包含 [推理,向量,图片]
export interface Model {
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

  models?: Model[];
}

// 模型
export interface ProviderModel {
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
  id: number;
  categoryId: number;

  // 图标Id // 通过stores/iconsStore.ts中的icons获取 getIconById(agent.iconId)
  iconId?: number;

  // 名称
  name: string;

  // 描述
  description?: string;

  // 模型通过providerId和name获取
  model?: ProviderModel;

  // 提示词
  prompt: string;

  // 模型生成文本的随机程度。值越大，回复内容越赋有多样性、创造性、随机性；设为 0 根据事实回答。日常聊天建议设置为 0.7
  // 默认1, 0 - 2
  temperature: number;

  // 最高概率采样，值越大，回复内容越赋有多样性、创造性、随机性；设为 0 根据事实回答。日常聊天建议设置为 0.9
  topP: number;

  // 最大消息长度, 默认0, 0 不限制
  maxTokens: number;

  // 要保留在上下文中的消息数量，数值越大，上下文越长，消耗的 token 越多。普通聊天建议 5-10
  // 默认 5
  contextSize: number;

  // 上下文是否保留扩展数据
  contextExtend: boolean;

  // 自定义参数
  params?: ModelParam[];

  // 工具集合
  tools?: number[];

  // 自定义问题
  // 1. 自定义问题, 例如: 你是谁? 你能做什么?
  customQuestions?: string[];

  createdAt: i64;
  updatedAt?: i64;
}

// 智能体类别
export interface AgentCategory {
  id: number;
  name: string;
  createdAt: i64;
}



// 工具类别
export interface ToolCategory {
  id: number;
  name: string;
  createdAt: number;
}

// 工具参数
export interface Param {
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

// tool
export interface McpTool {
  name: string;
  description: string;
  inputSchema?: Object;
}

// js
export interface ToolJavaScript {
  type: 'javaScript';
  param?: Param[];
  code: string;
}

// mcp-io
export interface ToolMcpIo {
  type: 'mcpIo'
  path: string;
}

// mcp-sse
export interface ToolMcpSse {
  type: 'mcpSse';
  url: string;
  tools: McpTool[];
}

// 工具
export interface Tool {
  id: number;
  categoryId: number;

  // 图标Id // 通过stores/iconsStore.ts中的icons获取 getIconById(agent.iconId)
  iconId?: number;

  name: string;
  description: string;

  // 工具类型
  data: ToolJavaScript | ToolMcpIo | ToolMcpSse;

  createdAt: i64;
  updatedAt?: i64;
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

export interface Attachment {
  name: string;
  size: number;
  // 文件内容,经过base64编码
  data: string;
}

// 工具执行结果
export interface ToolResult {
  // 工具id
  id: string;
  // 工具名称
  name: string;
  // json格式
  arguments: string;
  // json格式
  result: string;
}

// 聊天信息
export interface ChatMessage {
  // 消息ID
  id: number;
  
  // 会话ID
  sessionId: number;

  role: 'user' | 'assistant' | 'system';
  content: string;

  //工具消息结果
  tools?: ToolResult[];
  
  // 附件
  attachments?: Attachment[];

  status: string;

  // 反馈 默认 = 0, 赞 = 1, 踩 = 2
  feedback?: number;

  // 耗时
  cost?: number;

  // totalTokens > 0 时显示
  promptTokens?: number;
  completionTokens?: number;
  totalTokens?: number;
  
  createdAt: number;
  updatedAt?: number;
}

// 会话输入状态
export interface ChatInput {
  time: boolean;
  search: boolean;
  stream: boolean;
}

// 聊天会话信息
export interface ChatSession {
  // 会话id
  id: number;

  // 智能体id
  agentId: number;

  // 会话主题
  topic: string;

  input: ChatInput;
  
  createdAt: number;
  updatedAt?: number;
}

export interface SearchTavily {
  name: 'Tavily';
  apiKey: string;
}

export interface SearchBaidu {
  name: 'Baidu';
  apiKey: string;
}

/// 搜索选项
export interface Search {
  // 服务类型
  type: SearchTavily | SearchBaidu,

  // 模式 先搜索, 工具智能搜索
  mode: number,

  // 保留几条搜索结果
  resultCount: u32
}

// API 响应格式
export interface ApiResponse<T = any> {
  code: number;
  message: string;
  data?: T;
}