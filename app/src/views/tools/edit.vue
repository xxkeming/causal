<template>
  <div class="tool-edit-router">
    <js-edit 
      v-if="isJsType || (!isEdit && (tool?.data.type === 'javaScript' || toolType === 'js'))" 
      :tool="tool || undefined" 
      :category-id="categoryIdFromQuery" 
      :categories="categories"
    />
    <sse-edit
      v-else-if="isSseType || (!isEdit && (tool?.data.type === 'mcpSse' || toolType === 'mcp-sse'))"
      :tool="tool || undefined"
      :category-id="categoryIdFromQuery" 
      :categories="categories"
    />
    <io-edit
      v-else-if="isIoType || (!isEdit && (tool?.data.type === 'mcpIo' || toolType === 'mcp-io'))"
      :tool="tool || undefined"
      :category-id="categoryIdFromQuery" 
      :categories="categories"
    />
    <div v-else class="tool-unsupported">
      <n-result
        status="warning"
        title="暂不支持该类型工具的编辑"
        :description="`当前不支持编辑类型为 ${ toolType || '未知'} 的工具`"
      >
        <template #footer>
          <n-button @click="goBack">
            返回工具列表
          </n-button>
        </template>
      </n-result>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { NResult, NButton, useMessage } from 'naive-ui';
import { useToolStore } from '../../stores/toolStore';
import { useToolCategoryStore } from '../../stores/toolCategoryStore';
import { Tool, ToolCategory } from '../../services/typings';
import JsEdit from './js-edit.vue';
import SseEdit from './mcpsse-edit.vue';
import IoEdit from './mcpio-edit.vue';

const route = useRoute();
const router = useRouter();

const categoryStore = useToolCategoryStore();
const toolStore = useToolStore();
const message = useMessage();

// 判断是否为编辑模式
const isEdit = computed(() => !!route.params.id);
const toolId = computed(() => Number(route.params.id));

// 当前工具
const tool = ref<Tool | undefined>(undefined);
const isJsType = computed(() => {
  if (!tool.value) {
    return toolType.value === 'js';
  }
  return tool.value.data.type === 'javaScript';
});

const isSseType = computed(() => {
  if (!tool.value) {
    return toolType.value === 'mcp-sse';
  }
  return tool.value.data.type === 'mcpSse';
});

const isIoType = computed(() => {
  if (!tool.value) {
    return toolType.value === 'mcp-io';
  }
  return tool.value.data.type === 'mcpIo';
});

// 工具类型判断
const toolType = computed(() => route.query.type as string);

// 分类数据
const categories = ref<ToolCategory[]>([]);

// 从查询参数获取分类ID，用于创建新工具时设置默认分类
const categoryIdFromQuery = computed(() => {
  const categoryId = route.query.categoryId;
  return categoryId ? Number(categoryId) : undefined;
});

// 返回上一页
function goBack() {
  router.push('/tools');
}

// 加载分类数据
async function loadCategories() {
  try {
    await categoryStore.fetchCategories();
    categories.value = categoryStore.categories;
  } catch (error) {
    console.error('Failed to fetch categories:', error);
    message.error('加载分类数据失败');
  }
}

// 加载工具数据
async function loadToolData() {
  if (isEdit.value) {
    try {
      const fetchedTool = await toolStore.fetchToolById(toolId.value);
      if (fetchedTool) {
        tool.value = fetchedTool;
        
        if (!isJsType.value && !isSseType.value && !isIoType.value) {
          message.warning(`当前不支持编辑类型为 ${tool.value.data.type} 的工具`);
        }
      } else {
        message.error('未找到工具数据');
        router.push('/tools');
      }
    } catch (error) {
      message.error('加载工具数据失败');
      console.error(error);
      router.push('/tools');
    }
  }
}

onMounted(async () => {
  // 先加载分类数据
  await loadCategories();
  // 再加载工具数据
  await loadToolData();

  console.log('tool type:', toolType.value);
  console.log('tool:', tool.value);
});
</script>

<style scoped>
.tool-edit-router {
  height: 100%;
}

.tool-unsupported {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  padding: 20px;
}
</style>
