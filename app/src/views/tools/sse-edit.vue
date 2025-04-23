<template>
  <div class="tool-edit-view">
    <div class="tool-edit-container">
      <!-- 左侧列表 -->
      <div class="tool-edit-code">
        <div class="code-editor-container">
          <div class="code-editor-header">
            <div class="editor-actions">
              <n-space>
                <n-button quaternary size="small" @click="goBack">
                  <template #icon>
                    <n-icon><CloseOutline /></n-icon>
                  </template>
                  返回
                </n-button>
                <n-button quaternary size="small" :loading="testing" @click="testUrl">
                  <template #icon><n-icon><PlayOutline /></n-icon></template>
                  测试连接
                </n-button>
                <n-button 
                  quaternary 
                  size="small" 
                  :loading="saving" 
                  :disabled="!testPassed"
                  @click="saveTool"
                >
                  <template #icon><n-icon><SaveOutline /></n-icon></template>
                  {{ isEdit ? '更新' : '保存' }}
                </n-button>
              </n-space>
            </div>
          </div>
          
          <div class="code-editor-main">
            <div class="tool-list">
              <n-spin :show="testing">
                <n-empty v-if="!toolForm.data.tools?.length" description="暂无工具" />
                <n-list v-else>
                  <n-list-item v-for="tool in toolForm.data.tools" :key="tool.name">
                    <n-space align="center" justify="space-between">
                      <div class="tool-info">
                        <div class="tool-name">{{ tool.name }}</div>
                        <div class="tool-desc">{{ tool.description }}</div>
                      </div>
                    </n-space>
                  </n-list-item>
                </n-list>
              </n-spin>
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧表单 -->
      <div class="tool-edit-form">
        <div class="form-scroll-container">
          <n-form
            ref="formRef"
            :model="toolForm"
            :rules="formRules"
            label-placement="left"
            label-width="80px"
            require-mark-placement="right-hanging"
          >
            <div class="form-section">
              <h3 class="section-title">基本信息</h3>

              <div class="form-content">
                <n-form-item label="图标" path="iconId">
                  <avatar-selector v-model="toolForm.iconId" :previewSize="40" />
                </n-form-item>

                <n-form-item label="名称" path="name" :required="true">
                  <n-input 
                    v-model:value="toolForm.name" 
                    placeholder="输入工具名称" 
                    style="max-width: 320px;" 
                  />
                </n-form-item>

                <n-form-item label="分类" path="categoryId" :required="true">
                  <n-select
                    v-model:value="toolForm.categoryId"
                    :options="categoryOptions"
                    placeholder="选择工具分类"
                    style="max-width: 260px;"
                  />
                </n-form-item>

                <n-form-item label="描述" path="description" :required="true">
                  <n-input
                    v-model:value="toolForm.description"
                    type="textarea"
                    :autosize="{ minRows: 2, maxRows: 4 }"
                    placeholder="描述这个工具的功能"
                    style="max-width: 500px;"
                  />
                </n-form-item>

                <n-form-item label="URL" path="data.url" :required="true">
                  <n-input
                    v-model:value="toolForm.data.url"
                    placeholder="输入 SSE 服务地址"
                    style="max-width: 500px;"
                  />
                </n-form-item>
              </div>
            </div>
          </n-form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { NForm, NFormItem, NInput, NButton, NIcon, NSelect, NSpace, NEmpty, NSpin, NList, NListItem } from 'naive-ui';
import { CloseOutline, SaveOutline, PlayOutline } from '@vicons/ionicons5';
import { useToolStore } from '../../stores/toolStore';
import { useMessage } from 'naive-ui';
import AvatarSelector from '../../components/AvatarSelector.vue';
import type { Tool, ToolCategory, ToolMcpSse } from '../../services/typings';
import { getMcpSseTools } from '../../services/api';

// Props 类型定义
interface Props {
  tool?: Tool;
  categoryId?: number;
  categories: ToolCategory[];
}

const props = defineProps<Props>();

const route = useRoute();
const router = useRouter();
const message = useMessage();
const toolStore = useToolStore();

const isEdit = computed(() => !!route.params.id);
const toolId = computed(() => Number(route.params.id));

const formRef = ref(null);
const saving = ref(false);

// 表单数据类型定义
interface FormState {
  id: number;
  name: string;
  description: string;
  categoryId: number;
  iconId?: number;
  data: ToolMcpSse;
}

// 表单数据初始化
const toolForm = reactive<FormState>({
  id: 0,
  name: '',
  description: '',
  categoryId: 0, // 设置默认值为 0
  iconId: undefined,
  data: {
    type: 'mcpSse',
    url: '',
    tools: []
  }
});

// 工具列表状态
const toolsLoading = ref(false);

// 分类选项
const categoryOptions = computed(() => {
  return props.categories.map((category: ToolCategory) => ({
    label: category.name,
    value: category.id
  }));
});

// 表单验证规则
const formRules = {
  name: {
    required: true,
    message: '请输入工具名称'
  },
  categoryId: {
    required: true,
    message: '请选择分类'
  },
  description: {
    required: true,
    message: '请输入描述'
  },
  'data.url': {
    required: true,
    message: '请输入 URL 地址'
  }
};

// 获取MCP工具列表
async function fetchMcpTools() {
  toolsLoading.value = true;
  try {
    const data = await getMcpSseTools(toolForm.data.url);
    toolForm.data.tools = data;
  } catch (error) {
    console.error('Failed to fetch MCP tools:', error);
    message.error('获取工具列表失败');
  } finally {
    toolsLoading.value = false;
  }
}

// 添加测试状态
const testing = ref(false);
const testPassed = ref(false);

// 测试URL连接
async function testUrl() {
  if (!toolForm.data.url) {
    message.error('请输入 URL 地址');
    return;
  }

  testing.value = true;
  try {
    const data = await getMcpSseTools(toolForm.data.url);
    toolForm.data.tools = data;
    testPassed.value = true;
  } catch (error) {
    console.error('URL测试失败:', error);
    message.error('URL测试失败');
    testPassed.value = false;
    toolForm.data.tools = [];
  } finally {
    testing.value = false;
  }
}

// 重置测试状态当URL变化时
watch(() => toolForm.data.url, () => {
  testPassed.value = false;
  toolForm.data.tools = [];
});

// 保存工具前，手动验证参数
async function saveTool() {
  if (!testPassed.value) {
    message.error('请先测试URL连接');
    return;
  }

  if (!formRef.value) return;

  try {
    
    // 检查名称和描述是否已填写
    if (!toolForm.name) {
      message.error('请填写所有必填字段');
      return;
    }
    
    // 检查分类是否已选择 - 添加明确的分类检查
    if (!toolForm.categoryId) {
      message.error('请选择工具分类');
      return;
    }
    
    // 确保categoryId是数字类型
    toolForm.categoryId = Number(toolForm.categoryId);
    
    // 所有验证都通过，现在开始保存
    saving.value = true;
    
    if (isEdit.value) {
      console.log('更新', toolForm);

      await toolStore.modifyTool({
        ...toolForm,
        createdAt: props.tool?.createdAt || Date.now(), // 添加必需的 createdAt
        updatedAt: Date.now()
      });
      message.success('工具更新成功');
    } else {
      const submitData: Omit<Tool, 'id' | 'createdAt' | 'updatedAt'> = {
        name: toolForm.name,
        description: toolForm.description,
        categoryId: toolForm.categoryId,
        iconId: toolForm.iconId,
        data: {
          type: 'mcpSse',
          url: toolForm.data.url,
          tools: toolForm.data.tools
        }
      };
      await toolStore.createTool(submitData);
      message.success('工具创建成功');
    }

    setTimeout(() => {
      router.push('/tools');
    }, 1000);
    
  } catch (error) {
    console.error('Failed to save tool:', error);
    message.error('保存失败：' + (error || '未知错误'));
  } finally {
    saving.value = false;
  }
}

// 返回上一页
function goBack() {
  router.push('/tools');
}

// 监听 URL 变化自动获取工具列表
watch(() => toolForm.data.url, (newUrl) => {
  if (newUrl && toolForm.data.url !== newUrl) {
    fetchMcpTools();
  } else {
    toolForm.data.tools = [];
  }
});

// 加载工具数据
async function loadToolData() {
  if (isEdit.value) {
    if (props.tool) {
      // 如果父组件已经传入工具数据，直接使用
      Object.assign(toolForm, props.tool);
      
      // 确保categoryId是数字类型
      toolForm.categoryId = Number(props.tool.categoryId);
    } else {
      // 否则从服务器获取
      try {
        const tool = await toolStore.fetchToolById(toolId.value);
        if (tool) {
          Object.assign(toolForm, tool);
          
          // 确保categoryId是数字类型
          toolForm.categoryId = Number(tool.categoryId);
        } else {
          message.error('未找到工具数据');
          router.push('/tools');
        }
      } catch (error) {
        message.error('加载工具数据失败');
        console.error(error);
      }
    }
  } else {
    // 创建模式，首先检查props的categoryId
    if (props.categoryId) {
      toolForm.categoryId = props.categoryId;
    }
    // 再检查路由参数
    else if (route.query.categoryId) {
      toolForm.categoryId = Number(route.query.categoryId);
    }
    // 如果都没有，并且有传入的类别数据，则设置第一个类别为默认值
    else if (props.categories.length > 0) {
      toolForm.categoryId = Number(props.categories[0].id);
    }
  }
}

onMounted(async () => {
  await loadToolData();
});
</script>

<style scoped>
.tool-edit-view {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.tool-edit-container {
  display: flex;
  height: 100%; /* 修改高度占满整个区域，不再减去头部高度 */
  overflow: hidden;
}

.tool-edit-code {
  flex: 0 0 60%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border-right: 1px solid var(--border-color); /* 添加右边框 */
}

.tool-edit-form {
  flex: 0 0 40%;
  position: relative;
  padding: 0; /* 移除内边距 */
}

/* 新增样式：隐藏滚动条但保持可滚动 */
.form-scroll-container {
  height: 100%;
  overflow-y: auto;
  padding: 8px 8px 16px 8px;
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE and Edge */
}

/* Webkit浏览器中隐藏滚动条 */
.form-scroll-container::-webkit-scrollbar {
  display: none;
}

.tool-edit-code {
  flex: 0 0 60%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.form-section {
  margin-bottom: 12px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-color);
}

.section-title {
  margin-top: 8px;
  margin-bottom: 16px;
  font-size: 16px;
  font-weight: 500;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.params-list {
  margin-top: 16px;
}

.param-form {
  padding: 8px;
}

.code-editor-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.code-editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  border-bottom: 1px solid var(--border-color);
}

.code-editor-main {
  flex: 1;
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.code-editor {
  position: relative;
  height: 100%;
  overflow: hidden;
}

.form-content {
  max-width: 520px; /* 限制内容最大宽度，让内容区更紧凑 */
}

:deep(.n-collapse-item) {
  overflow: visible;
}

:deep(.n-code) {
  display: none;
}

.test-result-pre {
  background-color: #f5f5f5;
  padding: 12px;
  border-radius: 4px;
  max-height: 400px;
  overflow: auto;
  white-space: pre-wrap;
  font-family: 'Fira Code', 'Menlo', 'Monaco', 'Courier New', monospace;
  font-size: 12px;
  line-height: 1.5;
}

html.dark .test-result-pre {
  background-color: #1a1a1a;
  color: #d4d4d4;
}

.test-panel {
  height: 100%;
}

.test-controls {
  display: flex;
  gap: 8px;
  padding: 12px;
  background-color: #f0f0f0;
  border-bottom: 1px solid var(--border-color);
}

/* 暗色模式适配 */
html.dark .test-panel {
  background-color: #1a1a1a;
}

html.dark .test-controls {
  background-color: #242424;
  border-color: #333;
}

.editor-actions {
  display: flex;
  gap: 8px;
}

/* 添加错误文本样式 */
.error-text {
  color: var(--error-color);
  font-size: 12px;
}

/* 确保表单项反馈有足够高度 */
:deep(.n-form-item-feedback-wrapper) {
  min-height: 18px;
}

.tool-list {
  padding: 16px;
  height: 100%;
  overflow-y: auto;
}

.tool-info {
  flex: 1;
}

.tool-name {
  font-weight: 500;
  margin-bottom: 4px;
}

.tool-desc {
  font-size: 13px;
  color: var(--text-color-2);
}
</style>