import { Provider } from '../typings';

// 模型提供商模拟数据
export const mockProviders: Provider[] = [
  {
    id: 1,
    name: "自定义OpenAI",
    apiCategory: "OpenAI",
    url: "https://api.openai.com/v1",
    apiKey: "sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    models: [
      {
        name: "gpt-3.5-turbo",
        tags: ["推理"]
      },
      {
        name: "gpt-4",
        tags: ["推理"]
      },
      {
        name: "dall-e-3",
        tags: ["图片"]
      },
      {
        name: "text-embedding-ada-002",
        tags: ["向量"]
      }
    ]
  },
  {
    id: 2,
    name: "阿里通义",
    apiCategory: "Qwen",
    url: "https://dashscope.aliyuncs.com/api/v1",
    apiKey: "sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    models: [
      {
        name: "qwen-turbo",
        tags: ["推理"]
      },
      {
        name: "qwen-plus",
        tags: ["推理"]
      },
      {
        name: "qwen-max",
        tags: ["推理"]
      },
      {
        name: "text-embedding-v1",
        tags: ["向量"]
      }
    ]
  }
];
