# Causal - 智能对话平台

Causal 是一个基于 Tauri 2 和 Vue 3 构建的现代化智能对话平台，提供丰富的功能和优雅的用户体验。

## 效果预览

![应用预览](/app/public/app.jpg)
![应用预览](/app/public/tool.jpg)

## 主要功能

- 🤖 多智能体对话支持
- 💬 会话管理和历史记录
- 📝 Markdown 渲染引擎
- ➗ KaTeX 数学公式支持
- 📊 Mermaid 流程图渲染
- 🌈 代码语法高亮
- 📎 上下文文件支持
- 🔄 流式输出控制
- 🎨 响应式设计，适配各种设备

## 特色功能

### 智能体管理
- 自定义智能体配置
- 灵活的提示词系统
- 多种模型供应商支持
- 智能体分类管理

### 会话功能
- 实时流式响应
- 会话历史管理
- 富文本消息渲染
- 文件上传和解析
- 会话状态同步

### Markdown增强
- 代码块语法高亮
- 数学公式渲染
- 流程图自动生成
- 表格样式优化

## 技术栈

- 🦀 Tauri 2 - 构建轻量级跨平台应用
- 🟢 Vue 3 - 响应式前端框架
- 🔷 TypeScript - 类型安全的开发体验
- 🎯 Naive UI - 高质量 Vue 组件库
- 📝 Markdown-it - Markdown 渲染引擎
- ➗ KaTeX - 数学公式排版
- 📊 Mermaid - 图表和流程图
- ✨ Highlight.js - 代码语法高亮

## 开发指南

### 环境要求

- Node.js 20+
- Rust 最新稳定版
- Visual Studio Code (推荐)

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri dev
```

### 构建生产版本

```bash
npm run tauri build
```

## 项目结构

```
app/
├── src/
│   ├── components/     # 通用组件
│   ├── services/      # API 服务
│   ├── stores/        # 状态管理
│   └── views/         # 页面组件
│       ├── agents/    # 智能体管理
│       ├── chat/      # 对话界面
│       └── tools/     # 工具管理
├── public/           # 静态资源
└── package.json      # 项目配置
```

## 许可证

MIT © 2024 Present