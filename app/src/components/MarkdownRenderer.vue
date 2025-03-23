<template>
  <div class="markdown-body" v-html="renderedContent"></div>
</template>

<script setup lang="ts">
import { onMounted, watch, ref, toRaw } from 'vue';
import MarkdownIt from 'markdown-it';
import hljs from 'highlight.js';
import katex from 'katex';
import mermaid from 'mermaid';

import 'katex/dist/katex.min.css';

// 组件接口定义
interface Props {
  content: string;
}

const props = defineProps<Props>();
const renderedContent = ref<string>('');

// 初始化markdown-it实例，添加类型注解
const md: MarkdownIt = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,
  breaks: true,
  highlight: function (str: string, lang: string): string {
    // 特殊处理 mermaid 代码块
    if (lang === 'mermaid') {
      // 返回一个特殊标记，稍后会被处理
      return `<div class="mermaid-placeholder" data-diagram="${encodeURIComponent(str)}"></div>`;
    }
    
    // 处理其他语言的代码块
    if (lang && hljs.getLanguage(lang)) {
      try {
        return `<div class="code-block"><span class="code-language">${lang}</span><pre class="hljs"><code>${hljs.highlight(str, { language: lang, ignoreIllegals: true }).value}</code></pre></div>`;
      } catch (e) {
        console.error(e);
      }
    }
    return `<div class="code-block"><span class="code-language">plaintext</span><pre class="hljs"><code>${md.utils.escapeHtml(str)}</code></pre></div>`;
  }
});

// 初始化 Mermaid
onMounted(() => {
  mermaid.initialize({
    startOnLoad: false,
    theme: 'default',
    securityLevel: 'loose',
    fontFamily: 'sans-serif'
  });
});

// 处理数学公式
function renderMathFormulas(html: string): string {
  // 处理块级公式 - 匹配 $$...$$
  html = html.replace(/\$\$([\s\S]+?)\$\$/g, (_match, formula) => {
    try {
      return `<div class="katex-block">${katex.renderToString(formula, { 
        displayMode: true,
        throwOnError: false
      })}</div>`;
    } catch (error) {
      console.error('KaTeX 渲染错误:', error);
      return `<div class="katex-error">公式渲染错误: ${formula}</div>`;
    }
  });

  // 处理行内公式 - 匹配 $...$ 但不匹配 \$ 转义字符
  html = html.replace(/(?<!\\\$)\$(?!\$)(.*?)(?<!\\\$)\$(?!\$)/g, (_match, formula) => {
    try {
      return katex.renderToString(formula, { 
        displayMode: false,
        throwOnError: false
      });
    } catch (error) {
      console.error('KaTeX 渲染错误:', error);
      return `<span class="katex-error">${formula}</span>`;
    }
  });

  return html;
}

// 处理 Mermaid 图表
async function renderMermaidDiagrams(html: string): Promise<string> {
  const mermaidPattern = /<div class="mermaid-placeholder" data-diagram="([^"]+)"><\/div>/g;
  const matches = html.matchAll(mermaidPattern);
  
  for (const match of matches) {
    try {
      const id = `mermaid-diagram-${Date.now()}-${Math.floor(Math.random() * 10000)}`;
      const diagram = decodeURIComponent(match[1]);
      
      // 使用 Mermaid API 渲染图表
      const { svg } = await mermaid.render(id, diagram);
      
      // 在 HTML 中替换 Mermaid 占位符
      html = html.replace(
        match[0],
        `<div class="mermaid-diagram">${svg}</div>`
      );
    } catch (error) {
      console.error('Mermaid 渲染错误:', error);
      html = html.replace(
        match[0],
        `<div class="mermaid-error">流程图渲染错误: <pre>${decodeURIComponent(match[1])}</pre></div>`
      );
    }
  }
  
  return html;
}

// 渲染Markdown内容
async function renderMarkdown(content: string): Promise<string> {
  if (!content) return '';
  
  try {
    // 确保我们使用的是原始值，避免Vue的响应式代理
    const rawContent = toRaw(content);
    
    // 先渲染Markdown
    let html = md.render(rawContent);
    
    // 处理数学公式
    html = renderMathFormulas(html);
    
    // 处理 Mermaid 图表
    html = await renderMermaidDiagrams(html);
    
    return html;
  } catch (error) {
    console.error('Markdown渲染错误:', error);
    return `<div class="markdown-error">渲染错误: ${error instanceof Error ? error.message : String(error)}</div>`;
  }
}

// 更新渲染内容
async function updateRenderedContent() {
  renderedContent.value = await renderMarkdown(toRaw(props.content));
}

// 监听内容变化
watch(() => props.content, updateRenderedContent, { immediate: true });1

// 组件挂载后初始化
onMounted(() => {
  if (props.content) {
    updateRenderedContent();
  }
});
</script>

<style scoped>
/* Markdown样式 */
.markdown-body {
  line-height: 1.5; /* 略微减小行高 */
  word-wrap: break-word;
  font-size: 15px; /* 设置基础字体大小 */
}

.markdown-body :deep(h1),
.markdown-body :deep(h2),
.markdown-body :deep(h3),
.markdown-body :deep(h4),
.markdown-body :deep(h5),
.markdown-body :deep(h6) {
  margin-top: 20px; /* 从24px减小 */
  margin-bottom: 12px; /* 从16px减小 */
  font-weight: 600;
  line-height: 1.25;
}

.markdown-body :deep(h1) {
  font-size: 1.9em;
}

.markdown-body :deep(h2) {
  font-size: 1.45em;
}

.markdown-body :deep(h3) {
  font-size: 1.2em;
}

.markdown-body :deep(p) {
  margin: 0px;
}

.markdown-body :deep(a) {
  color: #0366d6;
  text-decoration: none;
}

.markdown-body :deep(a:hover) {
  text-decoration: underline;
}

.markdown-body :deep(strong) {
  font-weight: 600;
}

.markdown-body :deep(em) {
  font-style: italic;
}

.markdown-body :deep(ul),
.markdown-body :deep(ol) {
  padding-left: 1.8em; /* 略微减小缩进 */
  margin-bottom: 12px; /* 从16px减小 */
}

.markdown-body :deep(li+li) {
  margin-top: 0.2em; /* 从0.25em减小 */
}

.markdown-body :deep(code) {
  font-family: SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
  padding: 0.2em 0.4em;
  margin: 0;
  font-size: 95%; /* 从85%增大 */
  border-radius: 3px;
  background-color: rgba(27, 31, 35, 0.05);
}

.markdown-body :deep(pre) {
  font-family: SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
  padding: 12px; /* 从16px减小 */
  overflow: auto;
  font-size: 95%; /* 从85%增大 */
  line-height: 1.45;
  border-radius: 6px;
  background-color: #f6f8fa;
}

.markdown-body :deep(pre code) {
  font-size: 100%; /* 增大代码块中的代码字体大小 */
  background-color: transparent;
  padding: 0;
}

.markdown-body :deep(blockquote) {
  padding: 0 0.8em; /* 从1em减小 */
  color: #6a737d;
  border-left: 0.25em solid #dfe2e5;
  margin: 0 0 12px 0; /* 从16px减小 */
}

.markdown-body :deep(table) {
  border-collapse: collapse;
  width: 100%;
  margin-bottom: 12px; /* 从16px减小 */
}

.markdown-body :deep(table th),
.markdown-body :deep(table td) {
  padding: 6px 13px;
  border: 1px solid #dfe2e5;
}

.markdown-body :deep(table tr) {
  background-color: #fff;
  border-top: 1px solid #c6cbd1;
}

.markdown-body :deep(table tr:nth-child(2n)) {
  background-color: #f6f8fa;
}

.markdown-body :deep(img) {
  max-width: 100%;
  box-sizing: border-box;
}

.markdown-body :deep(.hljs) {
  background: #f6f8fa;
  border-radius: 6px;
  font-size: 15px; /* 明确设置高亮代码的字体大小 */
}

.markdown-body :deep(.mermaid) {
  text-align: center;
  margin: 16px 0;
}

/* KaTeX 特定的样式 */
.markdown-body :deep(.katex-display) {
  overflow-x: auto;
  overflow-y: hidden;
  padding: 8px 0;
}

.markdown-error {
  color: #d03050;
  padding: 8px;
  border-left: 3px solid #d03050;
  background-color: rgba(208, 48, 80, 0.1);
}

.markdown-body :deep(.code-block) {
  position: relative;
  margin-bottom: 12px; /* 从16px减小 */
}

.markdown-body :deep(.code-language) {
  position: absolute;
  top: 0;
  right: 0;
  background-color: rgba(0, 0, 0, 0.1);
  color: #666;
  padding: 1px 6px; /* 稍微减小填充 */
  font-size: 12px;
  border-bottom-left-radius: 3px;
  font-family: SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
  z-index: 1;
}

/* KaTeX 特定的样式 */
.markdown-body :deep(.katex-block) {
  overflow-x: auto;
  overflow-y: hidden;
  padding: 6px 0; /* 从8px减小 */
  text-align: center;
  margin: 12px 0; /* 从16px减小 */
}

.markdown-body :deep(.katex) {
  font-size: 1.1em;
}

.markdown-body :deep(.katex-error) {
  color: #cc0000;
  border-bottom: 1px dotted #cc0000;
}

/* Mermaid 样式 */
.markdown-body :deep(.mermaid-diagram) {
  text-align: center;
  margin: 12px 0; /* 从16px减小 */
  overflow-x: auto;
}

.markdown-body :deep(.mermaid-error) {
  color: #d03050;
  padding: 6px; /* 从8px减小 */
  border-left: 3px solid #d03050;
  background-color: rgba(208, 48, 80, 0.1);
  margin: 12px 0; /* 从16px减小 */
}

.markdown-body :deep(.mermaid-error pre) {
  font-family: SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
  padding: 6px; /* 从8px减小 */
  margin-top: 6px; /* 从8px减小 */
  overflow: auto;
  background-color: rgba(255, 255, 255, 0.7);
  border-radius: 3px;
}

/* 扩展的 Markdown 功能样式 */
/* 任务列表 */
.markdown-body :deep(.task-list-item) {
  list-style-type: none;
  margin-left: -1.5em;
}

.markdown-body :deep(.task-list-item input) {
  margin: 0 0.5em 0 0;
  vertical-align: middle;
}

/* 脚注 */
.markdown-body :deep(.footnotes) {
  margin-top: 20px;
  font-size: 0.85em;
}

.markdown-body :deep(.footnote-ref) {
  font-size: 0.8em;
  vertical-align: super;
  line-height: 0;
}

.markdown-body :deep(.footnote-backref) {
  font-size: 0.8em;
  text-decoration: none;
}

/* 定义列表 */
.markdown-body :deep(dl) {
  padding: 0;
  margin-bottom: 12px;
}

.markdown-body :deep(dt) {
  font-weight: bold;
  padding: 0;
  margin-top: 12px;
}

.markdown-body :deep(dd) {
  margin-left: 20px;
  padding: 0 12px;
}

/* 标记 */
.markdown-body :deep(mark) {
  background-color: #fff1a8;
  padding: 0.2em;
  border-radius: 2px;
}

/* 插入 */
.markdown-body :deep(ins) {
  text-decoration: underline;
  color: #22863a;
}

/* 缩写 */
.markdown-body :deep(abbr) {
  border-bottom: 1px dotted #666;
  cursor: help;
}

/* 上下标 */
.markdown-body :deep(sub),
.markdown-body :deep(sup) {
  font-size: 0.8em;
}
</style>
