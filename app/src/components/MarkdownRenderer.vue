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
    let highlightedCode = '';
    if (lang && hljs.getLanguage(lang)) {
      try {
        highlightedCode = hljs.highlight(str, { language: lang, ignoreIllegals: true }).value;
      } catch (e) {
        console.error(e);
        highlightedCode = md.utils.escapeHtml(str);
      }
    } else {
      // 默认处理
      highlightedCode = md.utils.escapeHtml(str);
    }

    // 当代码块是markdown时，确保特殊语法不被渲染
    if (lang === 'markdown' || lang === 'md') {
      // 特殊处理markdown代码块，确保数学公式和mermaid代码不被渲染
      str = str
        .replace(/\$\$([\s\S]+?)\$\$/g, (match) => match) // 保留数学公式原样
        .replace(/\$([^\$]+?)\$/g, (match) => match) // 保留行内公式原样
        .replace(/```mermaid([\s\S]+?)```/g, (match) => match); // 保留mermaid代码原样
    }

    // 优化代码块结构，移除多余的空白字符
    return `<div class="code-block"><div class="code-header"><span class="code-language">${lang || 'plaintext'}</span><button class="code-copy-btn" title="复制代码" onclick="navigator.clipboard.writeText(decodeURIComponent('${encodeURIComponent(str)}')); this.classList.add('copied'); setTimeout(() => this.classList.remove('copied'), 2000);"><svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="copy-icon"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg><svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="check-icon"><polyline points="20 6 9 17 4 12"></polyline></svg></button></div><pre class="hljs"><code>${highlightedCode}</code></pre></div>`;
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
  // 检查是否已经包含已渲染的KaTeX内容，如果有则不重复渲染
  if (html.includes('class="katex"')) {
    return html;
  }

  // 保护代码块内的公式不被渲染
  const codeBlocks: string[] = [];
  html = html.replace(/<pre class="hljs"><code>[\s\S]*?<\/code><\/pre>/g, (match) => {
    const index = codeBlocks.length;
    codeBlocks.push(match);
    return `<!--CODE_BLOCK_${index}-->`;
  });

  // 更精确的块级公式匹配 - 匹配独立行的 $$...$$，避免匹配内联代码或代码块中的公式
  html = html.replace(/(?<![`\\])\$\$([\s\S]+?)(?<![\\])\$\$(?!`)/g, (_match, formula) => {
    try {
      return `<span class="math-inline-block">
                ${katex.renderToString(formula.trim(), { 
                  displayMode: true,
                  throwOnError: false
                })}
              </span>`;
    } catch (error) {
      console.error('KaTeX 渲染错误:', error);
      // 渲染错误时保留原始公式内容
      return `<span class="math-raw-content">$$${formula}$$</span>`;
    }
  });

  // 更精确的行内公式匹配 - 匹配行内的单 $...$ 但避免匹配货币符号和其他误匹配情况
  // 1. 不匹配 \$ 转义字符
  // 2. 不匹配 $5 这类货币表示
  // 3. 确保公式内容不为空
  // 4. 确保公式内容两侧有正确的空白符边界
  html = html.replace(/(?<![\\a-zA-Z0-9])\$(?!\s)([^\n$]+?)(?<!\s|\\)\$(?![0-9a-zA-Z])/g, (_match, formula) => {
    // 排除可能的货币符号错误匹配
    if (/^\d+(\.\d+)?$/.test(formula.trim()) || 
        /^\s*$/.test(formula) || 
        formula.includes('\\$')) {
      return `$${formula}$`; // 返回原始文本，避免误渲染
    }
    
    try {
      return katex.renderToString(formula.trim(), { 
        displayMode: false,
        throwOnError: false
      });
    } catch (error) {
      console.error('KaTeX 渲染错误:', error);
      return `<span class="math-raw-content-inline">$${formula}$</span>`;
    }
  });

  // 恢复代码块
  html = html.replace(/<!--CODE_BLOCK_(\d+)-->/g, (_match, index) => {
    return codeBlocks[parseInt(index)];
  });

  return html;
}

// 处理 Mermaid 图表
async function renderMermaidDiagrams(html: string): Promise<string> {
  // 检查是否已经包含已渲染的Mermaid内容，如果有则不重复渲染
  if (html.includes('class="mermaid-diagram"')) {
    return html;
  }
  
  // 保护代码块内的mermaid代码不被渲染
  const codeBlocks: string[] = [];
  html = html.replace(/<pre class="hljs"><code>[\s\S]*?<\/code><\/pre>/g, (match) => {
    const index = codeBlocks.length;
    codeBlocks.push(match);
    return `<!--CODE_BLOCK_${index}-->`;
  });
  
  const mermaidPattern = /<div class="mermaid-placeholder" data-diagram="([^"]+)"><\/div>/g;
  const matches = html.matchAll(mermaidPattern);
  
  for (const match of matches) {
    try {
      const id = `mermaid-diagram-${Date.now()}-${Math.floor(Math.random() * 10000)}`;
      const diagram = decodeURIComponent(match[1]);
      
      // 使用 Mermaid API 渲染图表
      const { svg } = await mermaid.render(id, diagram);
      
      // 使用紧凑结构渲染Mermaid图表，减少空白
      html = html.replace(
        match[0],
        `<div class="code-block mermaid-container"><div class="code-header"><span class="code-language">mermaid</span><button class="code-copy-btn" title="复制图表源码" onclick="navigator.clipboard.writeText(decodeURIComponent('${encodeURIComponent(diagram)}')); this.classList.add('copied'); setTimeout(() => this.classList.remove('copied'), 2000);"><svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="copy-icon"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg><svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="check-icon"><polyline points="20 6 9 17 4 12"></polyline></svg></button></div><div class="mermaid-diagram">${svg.replace(/\s+/g, ' ').replace(/>\s+</g, '><')}</div></div>`
      );
    } catch (error) {
      console.error('Mermaid 渲染错误:', error);
      html = html.replace(
        match[0],
        `<div class="code-block mermaid-error-container">
          <div class="code-header">
            <span class="code-language">mermaid (错误)</span>
          </div>
          <div class="mermaid-error">
            <div>流程图渲染错误:</div>
            <pre>${decodeURIComponent(match[1])}</pre>
            <pre class="error-message">${error}</pre>
          </div>
        </div>`
      );
    }
  }
  
  // 恢复代码块
  html = html.replace(/<!--CODE_BLOCK_(\d+)-->/g, (_match, index) => {
    return codeBlocks[parseInt(index)];
  });
  
  return html;
}

// 处理 <think> 标签，将其转换为可折叠区域
function processThinkTags(html: string): { html: string; thinkBlocks: string[] } {
  // 在处理公式和流程图之前先提取和保护思考块的内容
  const thinkBlocks: string[] = [];
  
  // 提取完整的思考块
  html = html.replace(/<think>([\s\S]*?)<\/think>/g, (_match, content) => {
    // 检查内容是否只包含空白字符
    if (!content || /^\s*$/.test(content)) {
      return '';
    }
    
    const index = thinkBlocks.length;
    thinkBlocks.push(content);
    return `<!--THINK_BLOCK_${index}-->`;
  });
  
  // 提取不完整的思考块（流式传输的情况）
  const thinkStartTagPattern = /<think>([\s\S]*)$/;
  const startMatch = html.match(thinkStartTagPattern);
  
  if (startMatch) {
    // 找到了开始标签但没有结束标签
    const partialContent = startMatch[1];
    
    // 只有非空内容才处理
    if (partialContent && !/^\s*$/.test(partialContent)) {
      const index = thinkBlocks.length;
      thinkBlocks.push(partialContent);
      html = html.replace(thinkStartTagPattern, `<!--THINK_BLOCK_ONGOING_${index}-->`);
    }
  }
  
  // 现在可以安全地返回 html，由渲染流程中的其他函数处理
  return { html, thinkBlocks };
}

// 恢复思考块内容
function restoreThinkBlocks(html: string, thinkBlocks: string[]): string {
  // 恢复完整的思考块
  html = html.replace(/<!--THINK_BLOCK_(\d+)-->/g, (_match, index) => {
    const content = thinkBlocks[parseInt(index)];
    return `
      <div class="thinking-block">
        <details>
          <summary class="thinking-summary">思考过程</summary>
          <div class="thinking-content">${content}</div>
        </details>
      </div>
    `;
  });
  
  // 恢复正在进行的思考块
  html = html.replace(/<!--THINK_BLOCK_ONGOING_(\d+)-->/g, (_match, index) => {
    const content = thinkBlocks[parseInt(index)];
    return `
      <div class="thinking-block">
        <details open>
          <summary class="thinking-summary">思考过程</summary>
          <div class="thinking-content thinking-ongoing">${content}</div>
        </details>
      </div>
    `;
  });
  
  return html;
}

// 渲染Markdown内容
async function renderMarkdown(content: string): Promise<string> {
  if (!content) return '';
  
  try {
    // 确保我们使用的是原始值，避免Vue的响应式代理
    const rawContent = toRaw(content);
    
    // 预处理：标记可能被误认为公式的内容
    const preprocessedContent = rawContent
      // 保护单行代码中的 $ 符号
      .replace(/`([^`]*\$[^`]*)`/g, (match) => match)
      // 保护可能被误认为公式的货币表示
      .replace(/\$\d+(\.\d+)?/g, (match) => match)
      // 保护转义的美元符号
      .replace(/\\\$/g, '\\\\$');
    
    // 先渲染Markdown
    let html = md.render(preprocessedContent);
    
    // 提取并保护思考块
    const processedResult = processThinkTags(html);
    const htmlWithProtectedThinks = processedResult.html;
    const thinkBlocks = processedResult.thinkBlocks;
    
    // 处理数学公式 (已经保护了思考块)
    html = renderMathFormulas(htmlWithProtectedThinks);
    
    // 处理 Mermaid 图表
    html = await renderMermaidDiagrams(html);
    
    // 恢复并处理思考过程标签
    html = restoreThinkBlocks(html, thinkBlocks);
    
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
watch(() => props.content, updateRenderedContent, { immediate: true });

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
  line-height: 1.5; 
  word-wrap: break-word;
  font-size: 15px; 
}

.markdown-body :deep(h1),
.markdown-body :deep(h2),
.markdown-body :deep(h3),
.markdown-body :deep(h4),
.markdown-body :deep(h5),
.markdown-body :deep(h6) {
  margin-top: 16px; /* 进一步减小 */
  margin-bottom: 10px; /* 进一步减小 */
  font-weight: 600;
  line-height: 1.25;
}

.markdown-body :deep(h1) {
  font-size: 1.8em;
}

.markdown-body :deep(h2) {
  font-size: 1.4em;
}

.markdown-body :deep(h3) {
  font-size: 1.15em;
}

.markdown-body :deep(p) {
  margin: 0px 0px 8px;
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
  padding-left: 1.6em; /* 减小缩进 */
  margin-bottom: 10px; /* 减小间距 */
}

.markdown-body :deep(li+li) {
  margin-top: 0.15em; /* 减小间距 */
}

.markdown-body :deep(code) {
  font-family: SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
  padding: 0.2em 0.4em;
  margin: 0;
  font-size: 95%;
  border-radius: 3px;
  background-color: rgba(27, 31, 35, 0.05);
}

/* 简化代码块样式 */
.markdown-body :deep(.code-block) {
  margin: 10px 0; /* 设置垂直间距 */
  border-radius: 4px;
  overflow: hidden;
  border: 1px solid #e8e8e8;
  background-color: #f6f8fa;
  line-height: 1.3; /* 减小行高 */
}

.markdown-body :deep(.code-header) {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 2px 8px;
  background-color: #f1f3f5;
  border-bottom: 1px solid #e0e0e0;
  line-height: 1;
}

.markdown-body :deep(.code-language) {
  font-size: 12px;
  color: #666;
  font-weight: 500;
  font-family: SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
}

/* 优化代码块内部样式，减少间距 */
.markdown-body :deep(.hljs) {
  background: #f6f8fa;
  font-size: 14px;
  line-height: 1.3; /* 减小行高 */
  padding: 8px 10px;
  margin: 0; /* 移除外边距 */
  overflow-x: auto;
}

.markdown-body :deep(pre) {
  margin: 0; /* 重要：确保pre没有外边距 */
  padding: 0; /* 移除内边距 */
  font-family: SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
  overflow: visible; /* 修改为visible */
  line-height: 1.3; /* 确保行高一致 */
}

.markdown-body :deep(pre code) {
  font-size: 14px;
  background-color: transparent;
  padding: 0; /* 移除内边距 */
  margin: 0; /* 移除外边距 */
  line-height: 1.3; /* 确保行高一致 */
  white-space: pre; /* 保留空白字符，但允许换行 */
}

/* 解决markdown-body中可能存在的全局间距问题 */
.markdown-body :deep(p) {
  margin: 0 0 8px; /* 确保段落之间有适当的间距 */
}

.markdown-body :deep(p:last-child) {
  margin-bottom: 0; /* 最后一个段落不要底部间距 */
}

.markdown-body :deep(p + .code-block),
.markdown-body :deep(.code-block + p) {
  margin-top: 10px; /* 确保代码块和段落之间有正确的间距 */
}

/* 优化复制按钮样式 */
.markdown-body :deep(.code-copy-btn) {
  background: none;
  border: none;
  padding: 2px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #666;
  border-radius: 3px;
  transition: background-color 0.2s;
  height: 22px;
  width: 22px;
}

.markdown-body :deep(.code-copy-btn:hover) {
  background-color: rgba(0, 0, 0, 0.1);
}

.markdown-body :deep(.code-copy-btn .copy-icon) {
  display: inline-block;
}

.markdown-body :deep(.code-copy-btn .check-icon) {
  display: none;
}

.markdown-body :deep(.code-copy-btn.copied .copy-icon) {
  display: none;
}

.markdown-body :deep(.code-copy-btn.copied .check-icon) {
  display: inline-block;
  color: #22c55e;
}

.markdown-body :deep(.code-copy-btn.copied) {
  color: #22c55e;
}

/* 优化代码块内部样式 */
.markdown-body :deep(.hljs) {
  background: #f6f8fa;
  font-size: 14px;
  line-height: 1.4;
  padding: 0px 10px;
  margin: 0;
  overflow-x: auto;
}

.markdown-body :deep(pre) {
  margin: 0;
  font-family: SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
  overflow: auto;
}

.markdown-body :deep(pre code) {
  font-size: 14px;
  background-color: transparent;
  padding: 0;
}

.markdown-body :deep(blockquote) {
  padding: 0 0.6em;
  color: #6a737d;
  border-left: 0.25em solid #dfe2e5;
  margin: 0 0 10px 0;
}

/* 表格样式优化 */
.markdown-body :deep(table) {
  border-collapse: collapse;
  width: 100%;
  margin-bottom: 10px;
  font-size: 14px;
}

.markdown-body :deep(table th) {
  background-color: #f1f3f4;
  font-weight: 600;
  text-align: left;
  padding: 6px 10px;
  border: 1px solid #dfe2e5;
}

.markdown-body :deep(table td) {
  padding: 5px 10px;
  border: 1px solid #dfe2e5;
}

.markdown-body :deep(table tr:nth-child(2n)) {
  background-color: #f6f8fa;
}

.markdown-body :deep(img) {
  max-width: 100%;
  box-sizing: border-box;
}

/* Mermaid 样式优化 */
.markdown-body :deep(.mermaid-container) {
  margin: 8px 0; /* 减小垂直间距 */
}

.markdown-body :deep(.mermaid-diagram) {
  text-align: center;
  padding: 0; /* 移除内边距 */
  background-color: #f6f8fa;
  overflow-x: auto;
  line-height: 1; /* 减小行高 */
  font-size: 0; /* 消除文本节点可能带来的间距 */
}

.markdown-body :deep(.mermaid-diagram svg) {
  max-width: 100%; /* 确保SVG不会超出容器宽度 */
  height: auto; /* 保持宽高比 */
  display: inline-block; /* 处理可能的内联元素间距 */
  vertical-align: top; /* 防止底部出现空白 */
}

.markdown-body :deep(.mermaid-diagram > *) {
  margin: 0; /* 移除所有子元素的外边距 */
  padding: 0; /* 移除所有子元素的内边距 */
}

/* 修复SVG内部可能的间距问题 */
.markdown-body :deep(.mermaid-diagram svg text) {
  font-size: 14px; /* 恢复文本字体大小 */
}

/* 修复SVG内部边距问题 */
.markdown-body :deep(.mermaid-diagram svg g) {
  transform-origin: top left; /* 调整变换原点 */
}

/* 处理mermaid特有的样式问题 */
.markdown-body :deep(.mermaid-diagram .label) {
  font-family: sans-serif;
  font-size: 14px;
}

.markdown-body :deep(.mermaid-diagram .cluster rect) {
  stroke-width: 1px;
  stroke: #e0e0e0;
}

/* 覆盖Mermaid默认样式中可能导致间距的部分 */
.markdown-body :deep(.mermaid-diagram .node) {
  font-size: 14px;
  margin: 0;
  padding: 0;
}

/* Mermaid 样式优化 */
.markdown-body :deep(.mermaid-container) {
  margin: 8px 0; /* 减小垂直间距 */
}

.markdown-body :deep(.mermaid-diagram) {
  text-align: center;
  padding: 5px; /* 减小内边距 */
  background-color: #f6f8fa;
  overflow-x: auto;
  line-height: 1.1; /* 减小行高 */
}

.markdown-body :deep(.mermaid-diagram svg) {
  max-width: 100%; /* 确保SVG不会超出容器宽度 */
  height: auto; /* 保持宽高比 */
}

.markdown-body :deep(.mermaid-error-container) {
  margin: 10px 0;
}

.markdown-body :deep(.mermaid-error) {
  padding: 8px 10px;
  color: #d03050;
  font-size: 14px;
}

.markdown-body :deep(.mermaid-error pre) {
  margin: 5px 0;
  padding: 5px;
  background-color: rgba(255, 255, 255, 0.7);
  border-radius: 3px;
  overflow-x: auto;
  font-size: 13px;
  line-height: 1.3;
}

.markdown-body :deep(.mermaid-error .error-message) {
  color: #e53935;
  border-left: 3px solid #e53935;
  padding-left: 5px;
}

/* 优化KaTeX公式样式 */
.markdown-body :deep(.math-inline-block) {
  display: block;
  overflow-x: auto;
  margin: 4px 0;
  padding: 0;
  background-color: transparent;
  text-align: center;
  font-size: 1.05em;
}

/* 行内公式样式调整 */
.markdown-body :deep(.katex) {
  font-size: 1.05em;
}

/* 修复行内公式对齐问题 */
.markdown-body :deep(p .katex) {
  vertical-align: -0.25em; /* 微调与文本的对齐 */
}

/* 处理数学公式的错误样式 */
.markdown-body :deep(.math-error) {
  color: #cc0000;
  font-size: 14px;
  background-color: rgba(204, 0, 0, 0.05);
  padding: 2px 4px;
  border-radius: 2px;
}

.markdown-body :deep(.math-error-inline) {
  color: #cc0000;
  border-bottom: 1px dotted #cc0000;
  padding: 0 2px;
}

/* 移除旧的KaTeX块样式，使用新的行内风格 */
.markdown-body :deep(.katex-block) {
  margin: 0;
  padding: 0;
  border: none;
  border-radius: 0;
  background-color: transparent;
  text-align: center;
  overflow-x: auto;
  overflow-y: hidden;
  max-width: 100%;
}

.markdown-body :deep(.katex-display) {
  margin: 0.5em 0;
  padding: 0;
  overflow-x: auto;
  overflow-y: hidden;
  text-align: center;
}

/* 修正多行公式的滚动问题 */
.markdown-body :deep(.katex-display > .katex) {
  max-width: 100%;
  overflow-x: auto;
  overflow-y: hidden;
  padding: 0.25em 0;
}

/* 确保公式和周围文本有适当间距 */
.markdown-body :deep(p + .math-inline-block) {
  margin-top: 8px;
}

.markdown-body :deep(.math-inline-block + p) {
  margin-top: 8px;
}

/* 不同上下文中的公式样式调整 */
.markdown-body :deep(li .math-inline-block) {
  margin: 2px 0;
}

.markdown-body :deep(blockquote .math-inline-block) {
  font-size: 0.98em;
}

/* 移除旧的样式 */
.markdown-body :deep(.mermaid-actions),
.markdown-body :deep(.mermaid-copy-btn) {
  display: none;
}

/* 思考过程样式优化 */
.markdown-body :deep(.thinking-block) {
  margin: 6px 0;
  border-radius: 2px;
  border-left: 1px solid #6b7280;
  background-color: #f9fafb;
}

.markdown-body :deep(.thinking-summary) {
  padding: 3px 5px;
  font-weight: 600;
  color: #4b5563;
  cursor: pointer;
  user-select: none;
  display: flex;
  align-items: center;
  font-size: 14px;
}

.markdown-body :deep(.thinking-content) {
  padding: 4px 5px;
  border-top: 1px solid #e5e7eb;
  font-size: 0.95em;
  color: #4b5563;
}

/* 增强表格样式 */
.markdown-body :deep(.table-container) {
  overflow-x: auto;
  margin-bottom: 16px;
}

.markdown-body :deep(table) {
  border-collapse: collapse;
  width: 100%;
  margin-bottom: 12px;
  border: 1px solid #dfe2e5;
  border-radius: 3px;
  overflow: hidden;
}

.markdown-body :deep(table th) {
  background-color: #f1f3f4;
  font-weight: 600;
  text-align: left;
  color: #24292e;
  padding: 8px 13px;
  border: 1px solid #dfe2e5;
}

.markdown-body :deep(table td) {
  padding: 6px 13px;
  border: 1px solid #dfe2e5;
}

.markdown-body :deep(table tr) {
  background-color: #fff;
  border-top: 1px solid #c6cbd1;
  transition: background-color 0.2s ease;
}

.markdown-body :deep(table tr:nth-child(2n)) {
  background-color: #f6f8fa;
}

.markdown-body :deep(table tr:hover) {
  background-color: rgba(0, 120, 215, 0.05);
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

/* 正在进行中的思考过程样式 */
.markdown-body :deep(.thinking-ongoing) {
  border-left: 2px solid #6366f1;
  position: relative;
}

.markdown-body :deep(.thinking-ongoing::after) {
  content: "";
  display: inline-block;
  width: 6px;
  height: 6px;
  background-color: #6366f1;
  border-radius: 50%;
  margin-left: 6px;
  animation: thinking-pulse 1.5s infinite;
  vertical-align: middle;
}

@keyframes thinking-pulse {
  0% { opacity: 1; }
  50% { opacity: 0.3; }
  100% { opacity: 1; }
}

/* 修复嵌套问题：禁用嵌套的Markdown容器中的某些样式 */
.markdown-body :deep(.markdown-body) {
  /* 避免嵌套的markdown-body容器应用重复的样式 */
  padding: 0;
  margin: 0;
  border: none;
  box-shadow: none;
  background: transparent;
}

/* 处理嵌套Markdown中的标题样式 */
.markdown-body :deep(.markdown-body h1),
.markdown-body :deep(.markdown-body h2),
.markdown-body :deep(.markdown-body h3),
.markdown-body :deep(.markdown-body h4),
.markdown-body :deep(.markdown-body h5),
.markdown-body :deep(.markdown-body h6) {
  margin-top: 8px;
  margin-bottom: 5px;
}

/* 添加对markdown代码块中特殊语法的样式 */
.markdown-body :deep(.hljs .katex),
.markdown-body :deep(.hljs .mermaid-diagram) {
  display: inline; /* 确保在代码块内正确显示，不被当作特殊元素处理 */
  background: inherit;
  border: none;
  font-family: inherit;
  font-size: inherit;
  padding: 0;
  margin: 0;
}

/* 确保代码块内的特殊语法不被样式化 */
.markdown-body :deep(pre.hljs code .katex),
.markdown-body :deep(pre.hljs code .mermaid-placeholder) {
  font-family: SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
  white-space: pre;
}

/* 增强代码块内的语法识别 */
.markdown-body :deep(pre.hljs code .token.math),
.markdown-body :deep(pre.hljs code .token.mermaid) {
  color: inherit;
  background: inherit;
}

/* 修改数学公式渲染错误的样式 - 保留原始内容 */
.markdown-body :deep(.math-raw-content) {
  display: block;
  font-family: SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
  padding: 6px 10px;
  margin: 8px 0;
  background-color: rgba(0, 0, 0, 0.03);
  border-radius: 3px;
  white-space: pre-wrap;
  word-break: break-word;
  line-height: 1.4;
  color: #333;
  text-align: left; /* 保证原始公式文本左对齐更易读 */
}

.markdown-body :deep(.math-raw-content-inline) {
  font-family: SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
  padding: 0 4px;
  background-color: rgba(0, 0, 0, 0.03);
  border-radius: 3px;
  color: #333;
  display: inline-block; /* 确保行内显示但可以有内边距 */
  white-space: nowrap; /* 行内公式不换行 */
}

/* 移除之前的错误样式 */
.markdown-body :deep(.math-error),
.markdown-body :deep(.math-error-inline) {
  display: none;
}

/* 确保思考块内的内容不被其他样式影响 */
.markdown-body :deep(.thinking-content) {
  padding: 4px 5px;
  border-top: 1px solid #e5e7eb;
  font-size: 0.95em;
  color: #4b5563;
}

.markdown-body :deep(.thinking-content .katex) {
  font-size: inherit;
}

.markdown-body :deep(.thinking-content pre) {
  margin: 0.5em 0;
}

.markdown-body :deep(.thinking-content code) {
  white-space: pre-wrap;
}

/* 思考块内保持原始格式 */
.markdown-body :deep(.thinking-content .math-inline-block),
.markdown-body :deep(.thinking-content .katex-display) {
  text-align: left;
  margin: 0.25em 0;
}
</style>
