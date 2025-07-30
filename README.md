# Causal AI

<p align="center">
  <img src="/app/public/logo.png" alt="Causal Logo" width="200">
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-2.0-blue?style=flat-square" alt="Tauri 2.0">
  <img src="https://img.shields.io/badge/Vue-3-green?style=flat-square" alt="Vue 3">
  <img src="https://img.shields.io/badge/Rust-Stable-orange?style=flat-square" alt="Rust Stable">
  <img src="https://img.shields.io/github/license/hackfree/causal?style=flat-square" alt="License">
</p>

Causal 是一个基于 Tauri 2 和 Vue 3 构建的现代化智能对话平台，提供丰富的功能和优雅的用户体验。它支持多智能体对话、会话管理、Markdown 渲染、数学公式支持、流程图渲染等功能，并具有跨平台、轻量级的特点。

## 效果预览

<div style="display: flex; justify-content: space-around; flex-wrap: wrap;">
  <img src="/app/public/app.jpg" alt="应用主界面" width="45%">
  <img src="/app/public/tool.jpg" alt="工具界面" width="45%">
</div>

---

## 🌟 主要功能

- 🤖 多智能体对话支持
- 💬 会话管理和历史记录
- 📝 Markdown 渲染引擎
- ➗ KaTeX 数学公式支持
- 📊 Mermaid 流程图渲染
- 🌈 代码语法高亮
- 📎 上下文文件支持
- 🔄 流式输出控制
- 🎨 响应式设计，适配各种设备
- 🔧 可扩展的工具系统
- 🌐 多模型提供商支持（OpenAI、Anthropic等）

---

## 🚀 特色功能

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

### 工具系统
- 自定义工具开发
- MCP (Model Context Protocol) 支持
- JavaScript 工具支持
- 工具分类管理

---

## 🛠 技术栈

### 前端技术栈
- 🟢 Vue 3 - 响应式前端框架
- 🔷 TypeScript - 类型安全的开发体验
- 🎯 Naive UI - 高质量 Vue 组件库
- 📝 Markdown-it - Markdown 渲染引擎
- ➗ KaTeX - 数学公式排版
- 📊 Mermaid - 图表和流程图
- ✨ Highlight.js - 代码语法高亮
- 🧭 Vue Router - 路由管理
- 📦 Pinia - 状态管理

### 后端技术栈
- 🦀 Tauri 2 - 构建轻量级跨平台应用
- 🦀 Rust - 高性能系统编程语言
- 📦 Tokio - 异步运行时
- 🌐 async-openai - OpenAI API 客户端
- 🧠 RMCP - Model Context Protocol 实现

---

## 🚀 快速开始

### 环境要求

- Node.js 20+
- Rust 最新稳定版
- 推荐使用 Visual Studio Code 或其他支持 Rust 和 TypeScript 的 IDE

### 安装依赖
```bash
# 进入前端目录
cd app

# 安装前端依赖
npm install
```

### 开发模式运行
```bash
# 构建生产版本
npm run tauri build
```

### 📚 项目结构
```
causal/
├── app/                  # 前端应用
│   ├── src/              # Vue源码
│   │   ├── components/   # 组件
│   │   ├── views/        # 页面视图
│   │   ├── store/        # 状态管理
│   │   └── router/       # 路由配置
│   ├── src-tauri/        # Tauri后端
│   │   ├── src/          # Rust源码
│   │   ├── Cargo.toml    # Rust依赖配置
│   │   └── tauri.conf.json # Tauri配置
│   └── package.json      # 前端依赖配置
├── document/             # 文档目录
├── store/                # 数据存储
├── tools/                # 工具系统
└── README.md             # 项目说明
```

### 社区与支持
如有任何问题或建议，请加入我们的社区讨论组或在 GitHub 上提交 issue。我们期待您的反馈与合作。

### 📄 许可证
Apache 2.0 © 2025 Present
