<template>
  <div class="model-selector">
    <n-cascader
      v-model:value="selectedModel"
      :options="providerOptions"
      placeholder="选择模型提供商和模型"
      :filter="handleFilter"
      remote
      :on-load="handleLoad"
      :disabled="disabled"
      :clearable="false"
      @update:value="handleChange"
      class="custom-cascader"
      check-strategy="child"
      :default-value="defaultValue || modelValue"
    >
      <template #empty>{{ loading ? "加载中..." : "暂无模型" }}</template>
      <template #action v-if="showTagFilter">
        <div class="tag-filter">
          <div class="filter-tags">
            <n-tag
              v-for="tag in availableTags"
              :key="tag"
              :type="selectedTags.includes(tag) ? 'primary' : 'default'"
              :bordered="false"
              size="small"
              class="filter-tag"
              :style="{ cursor: 'pointer' }"
              @click.stop="toggleTagFilter(tag)"
            >
              {{ tag }}
            </n-tag>
          </div>
        </div>
      </template>
    </n-cascader>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import { NCascader, NTag } from "naive-ui";
import { useProviderStore } from "../stores/providerStore";
import type { CascaderOption } from "naive-ui";

const props = defineProps({
  modelValue: {
    type: String,
    default: () => null
  },
  // 添加默认值属性
  defaultValue: {
    type: String,
    default: () => null
  },
  size: {
    type: String,
    default: "medium"
  },
  disabled: {
    type: Boolean,
    default: false
  },
  showTagFilter: {
    type: Boolean,
    default: true
  },
  defaultTags: {
    type: Array as () => string[],
    default: () => ["推理"]
  }
});

const emit = defineEmits(["update:modelValue", "change"]);

// 从提供商store获取数据
const providerStore = useProviderStore();
const loading = ref(false);
const selectedModel = ref<string | null>(props.defaultValue?.length ? props.defaultValue : props.modelValue);
const selectedTags = ref<string[]>(props.defaultTags);
const availableTags = ["推理", "工具", "向量", "图片", "语音识别"];

// 过滤后的提供商选项
const providerOptions = computed<CascaderOption[]>(() => {
  // 如果没有加载提供商数据，返回空数组
  if (providerStore.providers.length === 0) {
    return [];
  }

  // 转换提供商数据为级联选择器选项
  return providerStore.providers.map(provider => {
    // 过滤模型
    const filteredModels = provider.models
      ? provider.models.filter(model => {
          // 如果没有选择标签过滤，则显示所有模型
          if (selectedTags.value.length === 0) return true;
          
          // 否则按标签过滤
          return model.tags?.some(tag => selectedTags.value.includes(tag));
        })
      : [];

    // 构建级联选择器选项
    return {
      label: provider.name,
      value: provider.id,
      isLeaf: false,
      disabled: !provider.models || filteredModels.length === 0,
      children: filteredModels.map(model => ({
        label: model.name,
        value: provider.id + '|' + model.name,
        isLeaf: true,
        // 添加额外信息用于UI展示
        tags: model.tags || [],
      }))
    };
  });
});

// 处理模型加载
const handleLoad = async (option: CascaderOption) => {
  if (option.children) {
    return;
  }

  loading.value = true;
  try {
    // 这里实际应用中可能需要异步加载模型数据
    // 在这个示例中我们直接使用已有的数据
    await providerStore.fetchProviderById(option.value as number);
  } finally {
    loading.value = false;
  }
};

// 加载所有提供商数据
const loadAllProviders = async () => {
  loading.value = true;
  try {
    await providerStore.fetchAllProviders();
  } finally {
    loading.value = false;
  }
};

// 处理模型选择变化 - 添加更多日志和确保类型转换正确
const handleChange = (value: string | null) => {
  console.log('ModelSelector handleChange 触发，收到值:', value, selectedModel.value);

  // 解析value格式为 providerId:modelName
  if (typeof value === 'string') {
    const [providerId, modelName] = value.split('|');
    // selectedModel.value = [parseInt(providerId, 10), modelName];

    // 获取provider对象
    // const provider = providerStore.providers.find(p => p.id === parseInt(providerId, 10));
    // if (!provider) {
    //   console.warn('未找到ID为', providerId, '的提供商');
    // } else {
    //   console.log('找到提供商:', provider.name);
    // }

    // 触发change事件，明确指定providerId是数字类型
    emit("change", {
      providerId,
      modelName,
      // provider
    });
  } else {
    console.log('清空选择器值');
    emit("update:modelValue", []);
    emit("change", null);
  }
};

// 切换标签过滤
const toggleTagFilter = (tag: string) => {
  if (selectedTags.value.includes(tag)) {
    selectedTags.value = selectedTags.value.filter(t => t !== tag);
  } else {
    selectedTags.value.push(tag);
  }
};

// 自定义过滤函数，支持按模型名称搜索
const handleFilter = (pattern: string, option: CascaderOption) => {
  if (!pattern) return true;
  
  // 如果是提供商级别，匹配提供商名称
  if (!option.isLeaf) {
    return option.label?.toString().toLowerCase().includes(pattern.toLowerCase()) || false;
  }
  
  // 如果是模型级别，匹配模型名称
  return option.label?.toString().toLowerCase().includes(pattern.toLowerCase()) || false;
};

// 监听默认标签变化
watch(
  () => props.defaultTags,
  (newTags) => {
    selectedTags.value = [...newTags];
  }
);

// 监听props.modelValue变化
watch(
  () => props.modelValue,
  (newValue) => {
    selectedModel.value = newValue;
  }
);

// 确保在组件挂载后正确初始化选择值
onMounted(async () => {
  console.log('ModelSelector mounted, 初始值:', props.modelValue, '默认值:', props.defaultValue);
  await loadAllProviders();
  console.log('提供商数据加载完成，共有', providerStore.providers.length, '个提供商');
});

</script>

<style scoped>
.model-selector {
  width: 100%;
}

/* 自定义级联选择器样式 */
:deep(.custom-cascader) {
  --cascader-menu-height: 320px;
}

:deep(.n-base-selection) {
  border-radius: 8px;
  transition: all 0.2s ease;
}

:deep(.n-base-selection:hover) {
  border-color: var(--primary-color);
}

:deep(.n-base-selection-focus) {
  box-shadow: 0 0 0 2px rgba(var(--primary-color-rgb), 0.2);
}

/* 自定义选项样式 */
.custom-option {
  padding: 4px 0;
  display: flex;
  flex-direction: column;
  width: 100%;
}

.option-content {
  display: flex;
  align-items: center;
  gap: 8px;
}

.option-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  color: var(--primary-color);
}

.option-label {
  font-size: 14px;
}

.option-tags {
  display: flex;
  gap: 4px;
  margin-top: 4px;
  margin-left: 28px;
}

/* 提供商级选项加粗 */
:deep(.n-cascader-menu:first-child .n-cascader-option-content) {
  font-weight: 500;
}

/* 选中状态样式 */
:deep(.n-cascader-option.n-cascader-option--selected) {
  background-color: rgba(var(--primary-color-rgb), 0.1);
}

:deep(.n-cascader-option.n-cascader-option--pending) {
  background-color: rgba(var(--primary-color-rgb), 0.05);
}

/* 隐藏级联选择器中的单选框 */
:deep(.n-cascader-option__checkbox) {
  display: none !important;
}

/* 确保内容没有缩进 */
:deep(.n-cascader-option__content) {
  margin-left: 0;
  padding-left: 8px !important;
}

/* 筛选标签样式 */
.tag-filter {
  padding: 10px;
}

.filter-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.filter-tag {
  cursor: pointer;
  transition: all 0.2s;
}

.filter-tag:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

/* 菜单样式优化 */
:deep(.n-cascader-menu) {
  border-right: 1px solid var(--border-color);
}

:deep(.n-cascader-menu:last-child) {
  border-right: none;
}

/* 确保标签内容垂直居中 */
:deep(.n-tag__content) {
  display: flex;
  align-items: center;
}
</style>
