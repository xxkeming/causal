import { KnowledgeBase } from '../typings';

// 知识库类别是固定的，不需要模拟数据
// 类别1: "1" - 通用知识库
// 类别2: "2" - 站点同步

// 知识库模拟数据
export const mockKnowledgeBases: KnowledgeBase[] = [
  {
    id: "1",
    categoryId: "1", // 通用知识库
    iconId: 12, // 使用BuildOutline图标
    name: "产品文档库",
    description: "包含产品手册、使用指南和常见问题解答的知识库",
    createdAt: "2023-09-01",
    updatedAt: "2023-10-15"
  },
  {
    id: "2",
    categoryId: "1", // 通用知识库
    iconId: 16, // 使用ColorPaletteOutline图标
    name: "设计规范库",
    description: "UI/UX设计规范、设计系统组件和样式指南",
    createdAt: "2023-09-05",
    updatedAt: "2023-10-10"
  },
  {
    id: "3",
    categoryId: "2", // 站点同步
    iconId: 19, // 使用EarthOutline图标
    name: "公司官网同步",
    description: "从公司官方网站自动同步的内容知识库",
    site: "https://www.example.com",
    createdAt: "2023-09-10",
    updatedAt: "2023-10-20"
  },
  {
    id: "4",
    categoryId: "1", // 通用知识库
    iconId: 31, // 假设使用某个图标
    name: "研发技术文档",
    description: "包含技术架构、API文档和开发规范的知识库",
    createdAt: "2023-08-15",
    updatedAt: "2023-10-25"
  },
  {
    id: "5",
    categoryId: "2", // 站点同步
    iconId: 18, // 使用CompassOutline图标
    name: "帮助中心同步",
    description: "从在线帮助中心自动同步的常见问题和解决方案",
    site: "https://help.example.com",
    createdAt: "2023-09-20",
    updatedAt: "2023-10-28"
  },
  {
    id: "6",
    categoryId: "1", // 通用知识库
    iconId: 23, // 使用FlashOutline图标
    name: "营销资料库",
    description: "市场营销策略、宣传材料和活动计划的集合",
    createdAt: "2023-08-25"
  },
  {
    id: "7",
    categoryId: "1", // 通用知识库
    iconId: 2, // 使用HeartOutline图标
    name: "客户案例库",
    description: "成功客户案例和项目实施经验的知识库",
    createdAt: "2023-09-15",
    updatedAt: "2023-10-05"
  },
  {
    id: "8",
    categoryId: "2", // 站点同步
    iconId: 6, // 使用RocketOutline图标
    name: "博客内容同步",
    description: "从公司博客自动同步的技术文章和行业洞察",
    site: "https://blog.example.com",
    createdAt: "2023-09-25",
    updatedAt: "2023-10-30"
  }
];
