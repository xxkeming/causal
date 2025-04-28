<template>
  <div class="tool-edit-view">
    <div class="tool-edit-container">
      <!-- 左侧列表 -->
      <div class="tool-edit-code">
        <div class="code-editor-container">
          <div class="code-editor-header">
            <div class="editor-actions">
              <n-space>
                <n-button quaternary size="small" :loading="globalStore.isLoading" @click="goBack">
                  <template #icon>
                    <n-icon><CloseOutline /></n-icon>
                  </template>
                  返回
                </n-button>
                <n-button quaternary size="small" :loading="globalStore.isLoading" @click="testIo">
                  <template #icon><n-icon><PlayOutline /></n-icon></template>
                  测试命令
                </n-button>
                <n-button 
                  quaternary 
                  size="small" 
                  :loading="globalStore.isLoading" 
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
              <n-spin :show="globalStore.isLoading">
                <n-empty v-if="!toolList.length" description="暂无工具" />
                <n-list v-else>
                  <n-list-item v-for="tool in toolList" :key="tool.name">
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

                <n-form-item label="命令" path="data.command" :required="true">
                  <n-input
                    v-model:value="toolForm.data.command"
                    placeholder="输入执行命令（如 npx, uv）"
                    style="max-width: 500px;"
                    :spellcheck="false"
                    :autocomplete="false"
                  />
                </n-form-item>
                
                <n-form-item label="参数" path="data.args">
                  <div class="args-container">
                    <div class="args-header">
                      <div class="args-input-group">
                        <div class="args-input-wrapper">
                          <n-input 
                            v-model:value="newArg" 
                            placeholder="输入命令参数" 
                            @keydown.enter.prevent="addArg"
                            style="width: 420px;"
                            :spellcheck="false"
                            :autocomplete="false"
                          />
                        </div>
                        <div class="args-button-wrapper">
                          <n-button 
                            type="primary" 
                            size="small" 
                            :disabled="!newArg"
                            @click="addArg"
                          >
                            添加
                          </n-button>
                        </div>
                      </div>
                    </div>
                    
                    <n-empty v-if="!hasArgs" description="暂无参数" size="small" style="margin-top: 16px;" />
                    <div v-else class="args-table-container">
                      <n-table size="small" :bordered="false" style="margin-top: 12px;">
                        <thead>
                          <tr>
                            <th>参数值</th>
                            <th style="width: 80px;">操作</th>
                          </tr>
                        </thead>
                        <tbody>
                          <tr v-for="(arg, index) in toolForm.data.args" :key="index">
                            <td><span class="arg-value">{{ arg }}</span></td>
                            <td>
                              <n-button size="tiny" quaternary type="error" @click="removeArg(index)">
                                <n-icon><TrashOutline /></n-icon>
                              </n-button>
                            </td>
                          </tr>
                        </tbody>
                      </n-table>
                    </div>
                  </div>
                </n-form-item>

                <n-form-item label="环境变量">
                  <div class="env-vars-container">
                    <div class="env-header">
                      <div class="env-input-group">
                        <div class="env-input-wrapper">
                          <n-input 
                            v-model:value="envKey" 
                            placeholder="如：API_KEY" 
                            :status="envKeyError ? 'error' : undefined"
                            style="width: 150px;"
                            :spellcheck="false"
                            :autocomplete="false"
                          />
                        </div>
                        <div class="env-input-wrapper">
                          <n-input 
                            v-model:value="envValue" 
                            placeholder="变量值" 
                            :status="envValueError ? 'error' : undefined"
                            style="width: 250px;"
                            :spellcheck="false"
                            :autocomplete="false"
                          />
                        </div>
                        <div class="env-button-wrapper">
                          <n-button 
                            type="primary" 
                            size="small" 
                            :disabled="!envKey || !envValue"
                            @click="addEnvVar"
                          >
                            添加
                          </n-button>
                        </div>
                      </div>
                    </div>
                    
                    <n-empty v-if="!hasEnvVars" description="暂无环境变量" size="small" style="margin-top: 16px;" />
                    <div v-else class="env-table-container">
                      <n-table size="small" :bordered="false" style="margin-top: 12px;">
                        <thead>
                          <tr>
                            <th style="width: 120px;">变量名</th>
                            <th>变量值</th>
                            <th style="width: 80px;">操作</th>
                          </tr>
                        </thead>
                        <tbody>
                          <tr v-for="(value, key) in toolForm.data.env" :key="key">
                            <td><span class="env-key-text">{{ key }}</span></td>
                            <td><span class="env-value-text">{{ value }}</span></td>
                            <td>
                              <n-button size="tiny" quaternary type="error" @click="removeEnvVar(key as string)">
                                <n-icon><TrashOutline /></n-icon>
                              </n-button>
                            </td>
                          </tr>
                        </tbody>
                      </n-table>
                    </div>
                  </div>
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
import { NForm, NFormItem, NInput, NButton, NIcon, NSelect, NSpace, NEmpty, NSpin, NTable, NList, NListItem  } from 'naive-ui';
import { CloseOutline, SaveOutline, PlayOutline, TrashOutline } from '@vicons/ionicons5';
import { useToolStore } from '../../stores/toolStore';
import { useGlobalStore } from '../../stores/globalStore';
import { useMessage } from 'naive-ui';
import AvatarSelector from '../../components/AvatarSelector.vue';
import type { Tool, ToolCategory, ToolMcpIo } from '../../services/typings';
import { getMcpIoTools } from '../../services/api';
import { McpTool } from '../../services';

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
const globalStore = useGlobalStore();

const isEdit = computed(() => !!route.params.id);
const toolId = computed(() => Number(route.params.id));

const formRef = ref(null);
const toolList = ref<McpTool[]>([]);

// 表单数据类型定义
interface FormState {
  id: number;
  name: string;
  description: string;
  categoryId: number | undefined;
  iconId?: number;
  data: ToolMcpIo;
}

// 表单数据初始化
const toolForm = reactive<FormState>({
  id: 0,
  name: '',
  description: '',
  categoryId: undefined,
  iconId: undefined,
  data: {
    type: 'mcpIo',
    command: '',
    args: [],
    env: {}
  }
});

// 参数输入相关
const newArg = ref('');
const hasArgs = computed(() => toolForm.data.args && toolForm.data.args.length > 0);

// 环境变量相关
const envKey = ref('');
const envValue = ref('');
const envKeyError = ref('');
const envValueError = ref('');

// 添加参数
function addArg() {
  if (!newArg.value) return;
  
  if (!toolForm.data.args) {
    toolForm.data.args = [];
  }
  
  toolForm.data.args.push(newArg.value);
  newArg.value = '';
  message.success('参数添加成功');
}

// 移除参数
function removeArg(index: number) {
  toolForm.data.args?.splice(index, 1);
  message.success('参数移除成功');
}

// 添加环境变量
function addEnvVar() {
  envKeyError.value = '';
  envValueError.value = '';
  
  if (!envKey.value) {
    envKeyError.value = '变量名不能为空';
    return;
  }
  
  if (!/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(envKey.value)) {
    envKeyError.value = '变量名必须以字母或下划线开头，只能包含字母、数字和下划线';
    return;
  }
  
  if (!envValue.value) {
    envValueError.value = '变量值不能为空';
    return;
  }
  
  if (!toolForm.data.env) {
    toolForm.data.env = {};
  }
  
  toolForm.data.env[envKey.value] = envValue.value;
  envKey.value = '';
  envValue.value = '';
  message.success('环境变量添加成功');
}

// 移除环境变量
function removeEnvVar(key: string) {
  if (toolForm.data.env) {
    delete toolForm.data.env[key];
    message.success('环境变量移除成功');
  }
}

// 检查是否有环境变量
const hasEnvVars = computed(() => {
  return toolForm.data.env && Object.keys(toolForm.data.env).length > 0;
});

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
  'data.command': {
    required: true,
    message: '请输入命令'
  }
};

// 添加测试状态
const testPassed = ref(false);

// 测试命令
async function testIo() {
  if (!toolForm.data.command) {
    message.error('请输入命令');
    return;
  }

  globalStore.setLoadingState(true);
  try {
    // 简单校验命令格式
    if (toolForm.data.command.trim().length === 0) {
      throw new Error('命令不能为空');
    }
    
    // 实际应用中这里应该调用后端接口测试命令是否可用
    // 此处模拟测试成功
    const data = await getMcpIoTools(toolForm.data);
    toolList.value = data;
    testPassed.value = true;
    message.success('命令测试通过');
  } catch (error) {
    console.error('命令测试失败:', error);
    message.error('命令测试失败');
    toolList.value = [];
    testPassed.value = false;
  } finally {
    globalStore.setLoadingState(false);
  }
}

// 重置测试状态当命令变化时
watch(() => toolForm.data.command, () => {
  testPassed.value = false;
});

// 保存工具前，手动验证参数
async function saveTool() {
  if (!testPassed.value) {
    message.error('请先测试命令');
    return;
  }

  if (!formRef.value) return;

  try {
    // 检查名称和描述是否已填写
    if (!toolForm.name) {
      message.error('请填写所有必填字段');
      return;
    }
    
    // 检查分类是否已选择
    if (!toolForm.categoryId) {
      message.error('请选择工具分类');
      return;
    }
    
    // 确保categoryId是数字类型
    toolForm.categoryId = Number(toolForm.categoryId);
    
    // 所有验证都通过，现在开始保存
    globalStore.setLoadingState(true);

    if (isEdit.value) {
      await toolStore.modifyTool({
        ...toolForm,
        categoryId: toolForm.categoryId as number, // Ensure categoryId is a number
        createdAt: props.tool?.createdAt || Date.now(),
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
          type: 'mcpIo',
          command: toolForm.data.command,
          args: toolForm.data.args,
          env: toolForm.data.env
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
    globalStore.setLoadingState(false);
  }
}

// 返回上一页
function goBack() {
  router.push('/tools');
}

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
    
    // 初始化空数组和对象
    toolForm.data.args = [];
    toolForm.data.env = {};
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
  height: 100%;
  overflow: hidden;
}

.tool-edit-code {
  /* flex: 0 0 55%; */
  width: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border-right: 1px solid var(--border-color);
}

.tool-edit-form {
  /* flex: 0 0 45%; */
  min-width: 580px;
  position: relative;
  padding: 0;
}

.form-scroll-container {
  height: 100%;
  overflow-y: auto;
  padding: 8px 8px 16px 8px;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.form-scroll-container::-webkit-scrollbar {
  display: none;
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

.env-vars-container {
  max-width: 500px;
}

.env-header {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
  align-items: center;
}

.editor-actions {
  display: flex;
  gap: 8px;
}

.error-text {
  color: var(--error-color);
  font-size: 12px;
}

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

.args-container {
  max-width: 500px;
}

.args-header {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}

.args-input-group {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px;
  background-color: var(--card-color);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  width: 100%;
}

.args-input-wrapper {
  flex: 1;
}

.args-button-wrapper {
  display: flex;
  align-items: center;
}

.args-table-container {
  border: 1px solid var(--border-color);
  border-radius: 4px;
  margin-top: 16px;
  overflow: hidden;
}

.arg-value {
  font-family: monospace;
  padding: 2px 6px;
  background: rgba(0, 0, 0, 0.04);
  border-radius: 3px;
  color: var(--text-color);
}

html.dark .arg-value {
  background: rgba(255, 255, 255, 0.08);
}

:deep(.n-input.n-input--error:focus) {
  border-color: var(--error-color);
}

.env-input-group {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px;
  background-color: var(--card-color);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  width: 100%;
}

.env-input-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.env-button-wrapper {
  display: flex;
  align-items: flex-end;
}

.env-key-text {
  font-family: monospace;
  color: var(--primary-color);
  font-weight: 500;
}

.env-value-text {
  font-family: monospace;
  color: var(--text-color-3);
}

.env-table-container {
  border: 1px solid var(--border-color);
  border-radius: 4px;
  margin-top: 16px;
  overflow: hidden;
}

:deep(.n-table) {
  --n-merged-th-color: transparent;
  --n-merged-td-color: transparent;
  --n-th-color: transparent;
  --n-td-color: transparent;
  --n-border-color: var(--border-color);
}
</style>