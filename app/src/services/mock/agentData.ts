import { Agent, AgentCategory } from '../typings';

// 智能体分类模拟数据
export const mockCategories: AgentCategory[] = [
  {
    id: 1,
    name: "办公助手",
    createdAt: Date.now()
  },
  {
    id: 2,
    name: "生产力",
    createdAt: Date.now()
  },
  {
    id: 3, 
    name: "创意写作",
    createdAt: Date.now()
  },
  {
    id: 4,
    name: "数据分析",
    createdAt: Date.now()
  }
];

// 智能体模拟数据，按照新的Model结构
export const mockAgents: Agent[] = [
  {
    id: 1,
    name: "文档助手",
    description: "帮助你撰写、编辑和改进文档内容",
    iconId: 15, // 对应图标库中的图标ID
    categoryId: 1,
    model: {
      id: 1, // 对应提供商ID
      name: "gpt-3.5-turbo" // 对应模型名称
    },
    prompt: "你是一个专业的文档编辑助手。你会帮助用户改进文档内容，提供修改建议，纠正语法错误，优化文档结构。请保持专业、简洁的沟通风格。",
    temperature: 0.7,
    maxTokens: 2000,
    contextSize: 8,
    params: [
      {
        name: "stylePreference",
        type: "string",
        value: "professional"
      }
    ],
    tools: [4, 8],
    createdAt: "2023-10-05T08:30:00Z"
  },
  {
    id: 2,
    name: "数据分析师",
    description: "分析数据并提供可视化建议",
    iconId: 33,
    categoryId: 4,
    model: {
      id: 1,
      name: "gpt-4"
    },
    prompt: "你是一个数据分析专家。你擅长分析各类数据，提取洞察，并以清晰的方式呈现结果。请在回答中使用专业的数据分析术语，并尽可能提供可视化建议。",
    temperature: 0.5,
    maxTokens: 4000,
    contextSize: 10,
    params: [
      {
        name: "dataFormat",
        type: "string",
        value: "csv"
      },
      {
        name: "includeVisualizations",
        type: "boolean",
        value: "true"
      }
    ],
    tools: [3, 8, 9],
    createdAt: "2023-11-12T14:45:00Z"
  },
  {
    id: 3,
    name: "创意写作助手",
    description: "帮助创作故事、诗歌和各类创意内容",
    iconId: 3,
    categoryId: 3,
    model: {
      id: 2,
      name: "qwen-plus"
    },
    prompt: "你是一个富有创造力的写作助手。你擅长创作引人入胜的故事、富有感染力的诗歌和其他创意内容。请使用生动、丰富的语言，根据用户的需求提供创意帮助。",
    temperature: 1.2,
    maxTokens: 3000,
    contextSize: 6,
    params: [
      {
        name: "genre",
        type: "string",
        value: "fantasy"
      },
      {
        name: "tone",
        type: "string",
        value: "imaginative"
      }
    ],
    tools: [],
    createdAt: "2023-12-03T09:20:00Z"
  },
  {
    id: 4,
    name: "会议记录助手",
    description: "帮助记录和整理会议内容",
    iconId: 28,
    categoryId: 1,
    model: {
      id: 1,
      name: "gpt-3.5-turbo"
    },
    prompt: "你是一个会议记录助手。你的任务是帮助用户整理会议内容，提取关键信息，组织成结构化的会议纪要。请使用简洁、清晰的格式，重点突出重要决策和后续行动项。",
    temperature: 0.4,
    maxTokens: 2500,
    contextSize: 12,
    params: [
      {
        name: "includeActionItems",
        type: "boolean",
        value: "true"
      }
    ],
    tools: [4],
    createdAt: "2024-01-15T10:30:00Z"
  },
  {
    id: 5,
    name: "代码助手",
    description: "提供编程帮助和代码建议",
    iconId: 31,
    categoryId: 2,
    model: {
      id: 1,
      name: "gpt-4"
    },
    prompt: "你是一个专业的编程助手。你会帮助用户解决编程问题、编写代码、解释技术概念，并提供最佳实践建议。请使用正确的代码格式，并在必要时提供详细的解释。",
    temperature: 0.5,
    maxTokens: 4000,
    contextSize: 15,
    params: [
      {
        name: "preferredLanguage",
        type: "string",
        value: "typescript"
      },
      {
        name: "includeExamples",
        type: "boolean",
        value: "true"
      }
    ],
    tools: [3, 5, 6],
    createdAt: "2024-02-07T15:15:00Z"
  }
];
