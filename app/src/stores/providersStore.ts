import { defineStore } from 'pinia';
import { ref, computed } from 'vue';


// 定义提供商类型
export interface ProviderApiCategory {
  id: number;
  name: string;
}

/*
    default.svg 默认
    openai.svg OpenAI
    qwen.svg 阿里通义
    deepseek.svg 深度求索
    wenxin.svg 文心一言
    hunyuan.svg 腾讯混元
    chatglm.svg ChatGLM
    claude.svg Claude
    doubao.svg 豆包
    gemini.svg Gemini
    gemma.svg Gemma
    grok.svg Grok
    meta.svg Meta
    mistral.svg Mistral
    moonshot.svg 月之暗面
*/
export interface ModelIcon {
  id: number;
  name: string;
  path: string; // SVG图标的路径
  color: string; // 图标关联的颜色
  displayName: string; // 显示名称（中文或其他友好名称）
}

// 预定义的颜色集合
const colors = [
  '#2080f0', // 蓝色
  '#d03050', // 红色
  '#f0a020', // 橙色
  '#18a058', // 绿色
  '#8a2be2', // 紫色
  '#ff1493', // 深粉色
  '#1e90ff', // 道奇蓝
  '#ff6347', // 番茄红
  '#3cb371', // 中海绿
  '#7b68ee'  // 中紫色
];

// 获取颜色函数
const getColor = (index: number) => colors[index % colors.length];

// assets/model-icons 目录中的模型图标列表
const modelIconsList: ModelIcon[] = [
  { id: 1, name: 'default', path: '/assets/model-icons/default.svg', color: getColor(0), displayName: '默认图标' },
  { id: 2, name: 'openai', path: '/assets/model-icons/openai.svg', color: getColor(1), displayName: 'OpenAI' },
  { id: 3, name: 'qwen', path: '/assets/model-icons/qwen.svg', color: getColor(2), displayName: '阿里通义' },
  { id: 4, name: 'deepseek', path: '/assets/model-icons/deepseek.svg', color: getColor(3), displayName: '深度求索' },
  { id: 5, name: 'wenxin', path: '/assets/model-icons/wenxin.svg', color: getColor(4), displayName: '文心一言' },
  { id: 6, name: 'hunyuan', path: '/assets/model-icons/hunyuan.svg', color: getColor(5), displayName: '腾讯混元' },
  { id: 7, name: 'chatglm', path: '/assets/model-icons/chatglm.svg', color: getColor(6), displayName: 'ChatGLM' },
  { id: 8, name: 'claude', path: '/assets/model-icons/claude.svg', color: getColor(7), displayName: 'Claude' },
  { id: 9, name: 'doubao', path: '/assets/model-icons/doubao.svg', color: getColor(8), displayName: '豆包' },
  { id: 10, name: 'gemini', path: '/assets/model-icons/gemini.svg', color: getColor(9), displayName: 'Gemini' },
  { id: 11, name: 'gemma', path: '/assets/model-icons/gemma.svg', color: getColor(0), displayName: 'Gemma' },
  { id: 12, name: 'grok', path: '/assets/model-icons/grok.svg', color: getColor(1), displayName: 'Grok' },
  { id: 13, name: 'meta', path: '/assets/model-icons/meta.svg', color: getColor(2), displayName: 'Meta' },
  { id: 14, name: 'mistral', path: '/assets/model-icons/mistral.svg', color: getColor(3), displayName: 'Mistral' },
  { id: 15, name: 'moonshot', path: '/assets/model-icons/moonshot.svg', color: getColor(4), displayName: '月之暗面' },
];

export const useModelIconStore = defineStore('modelIcon', () => {
  // 图标列表
  const icons = ref<ModelIcon[]>(modelIconsList);

  // 返回所有图标
  const allIcons = computed(() => icons.value);
  
  // 通过ID获取图标
  function getIconById(id?: number): ModelIcon | undefined {
    if (!id) return undefined;
    return icons.value.find(icon => icon.id === id);
  }
  
  // 通过名称获取图标
  function getIconByName(name: string): ModelIcon | undefined {
    if (!name) return undefined;
    return icons.value.find(icon => icon.name.toLowerCase() === name.toLowerCase());
  }
  
  // 搜索图标
  function searchIcons(keyword: string): ModelIcon[] {
    if (!keyword) return icons.value;
    const lowerKeyword = keyword.toLowerCase();
    return icons.value.filter(icon => 
      icon.name.toLowerCase().includes(lowerKeyword) ||
      icon.displayName.toLowerCase().includes(lowerKeyword)
    );
  }
  
  // 为特定图标设置颜色
  function setIconColor(id: number, color: string): void {
    const icon = icons.value.find(icon => icon.id === id);
    if (icon) {
      icon.color = color;
    }
  }

  // 添加新图标
  function addIcon(name: string, path: string, displayName: string): ModelIcon {
    const maxId = Math.max(0, ...icons.value.map(icon => icon.id));
    const newIcon: ModelIcon = {
      id: maxId + 1,
      name,
      path,
      color: getColor(maxId + 1),
      displayName
    };
    icons.value.push(newIcon);
    return newIcon;
  }
  
  return {
    icons,
    allIcons,
    getIconById,
    getIconByName,
    searchIcons,
    setIconColor,
    addIcon
  };
});
