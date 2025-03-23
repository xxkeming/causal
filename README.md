# Causal - 智能对话平台

Causal 是一个基于 Vue 3 和 TypeScript 构建的智能对话平台，支持富文本渲染、数学公式、代码高亮和流程图等高级功能。

## 主要功能

- 🤖 多智能体对话支持
- 💬 会话管理和历史记录
- 📝 Markdown 渲染引擎
- ➗ KaTeX 数学公式支持
- 📊 Mermaid 流程图渲染
- 🌈 代码语法高亮
- 🎨 响应式设计，适配各种设备

## 技术栈

- Tauri 2
- Vue 3
- TypeScript
- Naive UI
- Markdown-it
- KaTeX
- Mermaid
- Highlight.js

## 特色组件

### Markdown渲染器

提供了完整的 Markdown 渲染功能，支持：

- 标准 Markdown 语法
- 自动代码高亮（多种语言）
- 数学公式渲染（行内和块级）
- 流程图和图表渲染
- 表格、列表等高级格式化

### 聊天界面

- 左侧会话列表，支持多会话管理
- 右侧对话区域，支持富媒体显示
- 智能体信息展示
- 消息状态管理（发送中、成功、失败）
- 错误处理和重试机制

## 开发指南

### 环境要求

- Node.js 20+

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