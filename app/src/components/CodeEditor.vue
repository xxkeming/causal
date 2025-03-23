<template>
  <div class="code-editor-wrapper" :style="{ height: height }">
    <codemirror
      :model-value="modelValue"
      @update:model-value="onChange"
      :placeholder="placeholder"
      :style="{ height: '100%' }"
      :autofocus="autofocus"
      :indent-with-tab="true"
      :tab-size="tabSize"
      :extensions="resolvedExtensions"
      @blur="$emit('blur', $event)"
      @focus="$emit('focus', $event)"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { Codemirror } from 'vue-codemirror';
import { javascript } from '@codemirror/lang-javascript';
import { oneDark } from '@codemirror/theme-one-dark';
import { EditorView } from '@codemirror/view';
import { Extension } from '@codemirror/state'; // 导入 Extension 类型
import { useThemeStore } from '../stores/themeStore';

const props = defineProps({
  modelValue: {
    type: String,
    required: true,
  },
  language: {
    type: String,
    default: 'javascript',
    validator: (value: string) => ['javascript', 'typescript', 'json', 'html', 'css'].includes(value),
  },
  height: {
    type: String,
    default: '100%',
  },
  placeholder: {
    type: String,
    default: '',
  },
  autofocus: {
    type: Boolean,
    default: false,
  },
  tabSize: {
    type: Number,
    default: 2,
  },
  customExtensions: {
    type: Array as () => Extension[], // 明确指定类型为 Extension[]
    default: () => [],
  },
  formatOnBlur: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['update:modelValue', 'change', 'blur', 'focus']);

const themeStore = useThemeStore();
const isDark = computed(() => themeStore.theme === 'dark');

// 创建亮色主题
const lightTheme = EditorView.theme({
  "&": {
    backgroundColor: "#ffffff",
    color: "#333333",
    height: "100%",
    fontFamily: "'Fira Code', 'Menlo', 'Monaco', 'Courier New', monospace",
    fontSize: "14px"
  },
  ".cm-content": {
    caretColor: "#2160c4", // 更改为蓝色系
    lineHeight: "1.6",
    fontFamily: "'Fira Code', 'Menlo', 'Monaco', 'Courier New', monospace",
    padding: "8px 12px"
  },
  // 增强选中文本的样式 - 亮色模式
  ".cm-selectionBackground": {
    backgroundColor: "#3870c4 !important", // 更深、更饱和的蓝色
    color: "#ffffff !important", // 白色文本增加可读性
    borderRadius: "2px"
  },
  "&.cm-focused .cm-selectionBackground": {
    backgroundColor: "#3870c4 !important", // 保持一致的深蓝色
    color: "#ffffff !important",
    outline: "1px solid #2160c4", // 添加轮廓增强可见性
  },
  ".cm-selectionMatch": {
    backgroundColor: "rgba(56, 112, 196, 0.5)", // 更明显的匹配高亮
    borderRadius: "2px"
  },
  ".cm-activeLine": {
    backgroundColor: "#f0f5fb" // 更改为蓝色系的浅色
  },
  ".cm-gutters": {
    backgroundColor: "#f5f5f5",
    borderRight: "1px solid #ddd"
  },
  ".cm-activeLineGutter": {
    backgroundColor: "#e8f0fb" // 更改为蓝色系的浅色
  },
  // 语法高亮颜色保持不变
  ".cm-keyword": { color: "#d73a49" },
  ".cm-function": { color: "#6f42c1" },
  ".cm-variable": { color: "#24292e" },
  ".cm-string": { color: "#032f62" },
  ".cm-number": { color: "#005cc5" },
  ".cm-comment": { color: "#6a737d" },
  ".cm-property": { color: "#005cc5" },
});

// 创建暗色主题覆盖
const darkTheme = EditorView.theme({
  "&": {
    backgroundColor: "#1e1e1e",
    color: "#d4d4d4"
  },
  ".cm-content": {
    caretColor: "#569cd6" // 更改为VS Code风格的蓝色
  },
  // 增强选中文本的样式 - 暗色模式
  ".cm-selectionBackground": {
    backgroundColor: "#264f78 !important", // 使用更明显的蓝色
    color: "#ffffff !important",
    borderRadius: "2px"
  },
  "&.cm-focused .cm-selectionBackground": {
    backgroundColor: "#375e87 !important", // 聚焦时使用更亮的蓝色增加可见性
    color: "#ffffff !important",
    outline: "1px solid #4d7baa", // 添加亮色轮廓
  },
  ".cm-selectionMatch": {
    backgroundColor: "rgba(56, 112, 196, 0.6)", // 更明显的匹配高亮
    borderRadius: "2px"
  },
  // 其他暗色模式特定样式
  ".cm-activeLine": {
    backgroundColor: "#2c3136"
  },
  ".cm-gutters": {
    backgroundColor: "#1e1e1e",
    borderRight: "1px solid #333"
  },
  ".cm-activeLineGutter": {
    backgroundColor: "#2c3136"
  }
}, { dark: true });

// 根据语言配置语言扩展
const languageExtension = computed(() => {
  switch (props.language) {
    case 'typescript':
      return javascript({ jsx: false, typescript: true });
    case 'javascript':
    default:
      return javascript({ jsx: false, typescript: false });
  }
});

// 计算所有扩展
const resolvedExtensions = computed((): Extension[] => [
  languageExtension.value,
  EditorView.lineWrapping,
  isDark.value ? [oneDark, darkTheme] : lightTheme,
  ...(props.customExtensions as Extension[]) // 添加类型断言
]);

// 处理内容变化
const onChange = (value: string) => {
  emit('update:modelValue', value);
  emit('change', value);
};

// 格式化代码
const formatCode = (code: string): string => {
  try {
    return code
      .replace(/}\s*else/g, '} else')
      .replace(/{\s*/g, '{\n  ')
      .replace(/\s*}/g, '\n}')
      .replace(/;\s*/g, ';\n  ')
      .replace(/\n\s*\n\s*\n/g, '\n\n');
  } catch (e) {
    console.error('Error formatting code:', e);
    return code;
  }
};

// 暴露格式化方法给父组件
defineExpose({
  formatCode
});
</script>

<style scoped>
.code-editor-wrapper {
  position: relative;
  width: 100%;
  overflow: hidden;
  border-radius: 4px;
}

:deep(.cm-editor) {
  height: 100%;
  width: 100%;
  font-family: 'Fira Code', 'Menlo', 'Monaco', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.6;
}

:deep(.cm-scroller) {
  overflow: auto;
}

:deep(.cm-content) {
  padding: 8px 12px;
  font-family: 'Fira Code', 'Menlo', 'Monaco', 'Courier New', monospace;
}

:deep(.cm-editor.cm-focused) {
  outline: none;
}

:deep(.cm-line) {
  padding: 0 4px 0 4px;
}

:deep(.cm-gutters) {
  border-right: 1px solid var(--border-color);
}

/* 添加特定的选中样式覆盖，确保不被其他样式覆盖 */
:deep(.cm-selectionBackground) {
  opacity: 0.9;
}

:deep(.cm-editor.cm-focused .cm-selectionBackground) {
  opacity: 1;
}

/* 重新设置选中样式，解决可能的样式冲突 */
:deep(.cm-selectionBackground),
:deep(.cm-content ::selection) {
  background-color: rgba(33, 96, 196, 0.4);
  color: #000000;
  text-shadow: none;
}

/* 大幅增强选中样式，解决可能的样式冲突 */
:deep(.cm-selectionBackground),
:deep(.cm-content ::selection) {
  background-color: #3870c4 !important;
  color: white !important;
  text-shadow: none;
  border-radius: 2px;
}

:deep(.cm-editor.cm-focused .cm-selectionBackground) {
  opacity: 1;
  outline: 1px solid #2160c4 !important; 
  border-radius: 2px !important;
  box-shadow: 0 0 0 1px rgba(33, 96, 196, 0.2);
}

/* 暗色主题下的选中样式 */
html.dark :deep(.cm-selectionBackground),
html.dark :deep(.cm-content ::selection) {
  background-color: #375e87 !important;
  color: white !important;
  text-shadow: 0 0 1px rgba(255, 255, 255, 0.5);
}

html.dark :deep(.cm-editor.cm-focused .cm-selectionBackground) {
  outline: 1px solid #4d7baa !important;
  box-shadow: 0 0 0 1px rgba(77, 123, 170, 0.3);
}

/* 确保选中状态的文本有足够的对比度 */
:deep(.cm-selectionBackground) {
  opacity: 1 !important;
}
</style>
