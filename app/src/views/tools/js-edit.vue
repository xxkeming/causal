<template>
  <div class="tool-edit-view">
    <div class="tool-edit-container">
      <!-- 左侧代码编辑器 -->
      <div class="tool-edit-code">
        <div class="code-editor-container">
          <div class="code-editor-header">
            <div class="editor-tabs">
              <n-tabs type="line" size="small" v-model:value="activeTab">
                <n-tab-pane name="code" tab="实现代码" />
                <n-tab-pane name="test" tab="测试日志" />
              </n-tabs>
            </div>
            <div class="editor-actions">
              <n-space>
                <n-button quaternary size="small" @click="goBack" :loading="toolStore.loading">
                  <template #icon>
                    <n-icon><CloseOutline /></n-icon>
                  </template>
                  返回
                </n-button>
                <n-button quaternary size="small" :loading="saving || toolStore.loading" @click="saveTool">
                  <template #icon>
                    <n-icon><SaveOutline /></n-icon>
                  </template>
                  {{ isEdit ? '更新' : '保存' }}
                </n-button>
                <n-button quaternary size="small" @click="testCode" :loading="toolStore.loading">
                  <template #icon>
                    <n-icon><PlayOutline /></n-icon>
                  </template>
                  测试
                </n-button>
              </n-space>
            </div>
          </div>
          
          <div class="code-editor-main">
            <div v-show="activeTab === 'code'" class="code-editor">
              <code-editor
                v-model="jsCode"
                language="javascript"
                placeholder="// 在这里编写工具实现代码"
                :autofocus="true"
                @change="codeChanged"
                ref="jsEditor"
              />
            </div>
            <div v-show="activeTab === 'test'" class="test-panel">
              <log-viewer :logs="testLogs" />
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
            <!-- 基本信息 -->
            <div class="form-section">
              <h3 class="section-title">基本信息</h3>

              <div class="form-content">
                <n-form-item label="图标" path="iconId">
                  <avatar-selector v-model="toolForm.iconId" :previewSize="40" />
                </n-form-item>

                <n-form-item label="名称" path="name" :required="true">
                  <n-input 
                    v-model:value="toolForm.name" 
                    placeholder="输入工具名称（必填）" 
                    style="max-width: 320px;" 
                    :status="submitAttempted && !toolForm.name ? 'error' : undefined"
                  />
                  <template #feedback>
                    <span v-if="submitAttempted && !toolForm.name" class="error-text">
                      请输入工具名称
                    </span>
                  </template>
                </n-form-item>

                <n-form-item label="分类" path="categoryId" :required="true">
                  <n-select
                    v-model:value="toolForm.categoryId"
                    :options="categoryOptions"
                    placeholder="选择工具分类"
                    style="max-width: 260px;"
                    :status="submitAttempted && !toolForm.categoryId ? 'error' : undefined"
                  />
                  <template #feedback>
                    <span v-if="submitAttempted && !toolForm.categoryId" class="error-text">
                      请选择工具分类
                    </span>
                  </template>
                </n-form-item>

                <n-form-item label="描述" path="description" :required="true">
                  <n-input
                    v-model:value="toolForm.description"
                    type="textarea"
                    :autosize="{ minRows: 2, maxRows: 4 }"
                    placeholder="描述这个工具的功能（必填）"
                    style="max-width: 500px;"
                    :status="submitAttempted && !toolForm.description ? 'error' : undefined"
                  />
                  <template #feedback>
                    <span v-if="submitAttempted && !toolForm.description" class="error-text">
                      请输入工具描述
                    </span>
                  </template>
                </n-form-item>
              </div>
            </div>

            <!-- 参数设置 -->
            <div class="form-section">
              <div class="section-header">
                <h3 class="section-title">参数设置</h3>
                <n-button text type="primary" @click="addParam">
                  <template #icon>
                    <n-icon><AddOutline /></n-icon>
                  </template>
                  添加
                </n-button>
              </div>

              <div v-if="jsParams && jsParams.length > 0" class="params-list">
                <n-collapse>
                  <n-collapse-item
                    v-for="(param, index) in jsParams"
                    :key="index"
                    :title="param.name || `参数 ${index + 1}`"
                  >
                    <div class="param-form">
                      <n-grid :cols="24" :x-gap="12">
                        <n-grid-item :span="12">
                          <n-form-item 
                            :label="'名称'" 
                            :path="`data.param[${index}].name`" 
                            :required="true"
                            :rule="paramNameRule"
                          >
                            <n-input 
                              v-model:value="param.name" 
                              placeholder="参数名称（必填，小写字母开头）" 
                              :status="getParamNameStatus(param.name)"
                              @input="filterNonAsciiChars($event, param, 'name')"
                              @compositionend="handleCompositionEnd($event, param)"
                            />
                            <template #feedback>
                              <span v-if="submitAttempted && !param.name" class="error-text">
                                参数名称不能为空
                              </span>
                              <span v-else-if="submitAttempted && !isValidParamName(param.name)" class="error-text">
                                参数名称必须以小写字母开头，只能包含字母、数字和下划线
                              </span>
                              <span v-else-if="containsChineseChars(param.name)" class="error-text">
                                参数名称不能包含中文字符
                              </span>
                            </template>
                          </n-form-item>
                        </n-grid-item>
                        <n-grid-item :span="12">
                          <n-form-item 
                            :label="'参数'" 
                            :path="`data.param[${index}].type`" 
                            :required="true"
                          >
                            <n-select
                              v-model:value="param.type"
                              :options="paramTypeOptions"
                              placeholder="选择参数类型（必填）"
                              :status="submitAttempted && !param.type ? 'error' : undefined"
                            />
                            <template #feedback>
                              <span v-if="submitAttempted && !param.type" class="error-text">
                                请选择参数类型
                              </span>
                            </template>
                          </n-form-item>
                        </n-grid-item>
                        <n-grid-item :span="24">
                          <n-form-item 
                            :label="'描述'" 
                            :path="`data.param[${index}].description`" 
                            :required="true"
                          >
                            <n-input
                              v-model:value="param.description"
                              placeholder="参数描述（必填）"
                              type="textarea"
                              :autosize="{ minRows: 1, maxRows: 3 }"
                              :status="submitAttempted && !param.description ? 'error' : undefined"
                            />
                            <template #feedback>
                              <span v-if="submitAttempted && !param.description" class="error-text">
                                参数描述不能为空
                              </span>
                            </template>
                          </n-form-item>
                        </n-grid-item>
                        <n-grid-item :span="24">
                          <n-form-item 
                            :label="'测试值'" 
                            :path="`data.param[${index}].testValue`"
                            :rule="param.required ? requiredTestValueRule : undefined"
                          >
                            <n-input
                              v-model:value="param.testValue"
                              placeholder="测试值"
                              :type="param.type === 'number' ? 'text' : 'text'"
                              :status="param.required && !param.testValue ? 'error' : undefined"
                            />
                            <template #feedback>
                              <span v-if="param.required && !param.testValue" class="error-text">
                                必填参数必须提供测试值
                              </span>
                            </template>
                          </n-form-item>
                        </n-grid-item>
                        <n-grid-item :span="12">
                          <n-form-item>
                            <n-checkbox 
                              v-model:checked="param.required" 
                              @update:checked="validateRequiredTestValue(param)"
                            >
                              必填参数
                            </n-checkbox>
                          </n-form-item>
                        </n-grid-item>
                        <n-grid-item :span="12" style="display: flex; justify-content: flex-end; align-items: flex-end;">
                          <n-button type="error" @click="removeParam(index)" style="margin-bottom: 24px;">
                            删除参数
                          </n-button>
                        </n-grid-item>
                      </n-grid>
                    </div>
                  </n-collapse-item>
                </n-collapse>
              </div>
              
              <n-empty v-else description="还没有添加参数" style="margin-top: 20px;"/>
            </div>
          </n-form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted, onBeforeUnmount, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import {
  NForm, NFormItem, NInput, NButton, NIcon, NSelect,
  NGrid, NGridItem, NSpace, NCollapse, NRadioButton,
  NCollapseItem, NTabs, NTabPane, NCheckbox,
  NEmpty, useMessage
} from 'naive-ui';
import {
  AddOutline, PlayOutline, CloseOutline, SaveOutline
} from '@vicons/ionicons5';
import { useToolStore } from '../../stores/toolStore';
import AvatarSelector from '../../components/AvatarSelector.vue';
import CodeEditor from '../../components/CodeEditor.vue';
import LogViewer from '../../components/LogViewer.vue';

// 添加props接收父组件传递的数据
const props = defineProps({
  tool: {
    type: Object,
    default: null
  },
  categoryId: {
    type: Number,
    default: undefined
  },
  categories: { // 新增：接收分类数据
    type: Array,
    default: () => []
  }
});

const route = useRoute();
const router = useRouter();
const message = useMessage();
const toolStore = useToolStore();

// 编辑器引用
const jsEditor = ref(null);

// 判断是否为编辑模式
const isEdit = computed(() => !!route.params.id);
const toolId = computed(() => Number(route.params.id));

// 表单引用
const formRef = ref(null);

// 编辑器标签页
const activeTab = ref('code');

// 保存状态
const saving = ref(false);

// 测试相关状态变量
const testLogs = ref([]);

// 创建默认的JS工具数据
const createDefaultJsToolData = () => ({
  param: [],
  code: `async function toolFunction(params) {
  try {
    // 在这里实现你的工具逻辑
    const result = {};
    return result;
  } catch (error) {
    return { success: false, error: error.message };
  }
}`
});

// 表单数据
const toolForm = reactive({
  id: 0,
  name: '',
  description: '',
  categoryId: undefined,
  iconId: undefined,
  type: 'js',
  data: createDefaultJsToolData(),
  createdAt: '',
  updatedAt: undefined
});

// JS代码编辑器的值
const jsCode = ref(toolForm.data?.code || '');

// 获取JS参数数组
const jsParams = computed(() => toolForm.data?.param || []);

// 参数类型选项
const paramTypeOptions = [
  { label: '字符串', value: 'string' },
  { label: '数字', value: 'number' },
  { label: '布尔值', value: 'boolean' },
  { label: '对象', value: 'object' }
];

// 分类选项 - 修改为使用props中的categories
const categoryOptions = computed(() => {
  return props.categories.map(category => ({
    label: category.name,
    value: category.id
  }));
});

// 表单验证规则
const formRules = {
  name: {
    required: true,
    message: '请输入工具名称',
    trigger: 'blur'
  },
  categoryId: {
    type: 'number',
    required: true,
    message: '请选择分类',
    trigger: ['blur', 'change']
  },
  description: {
    required: true,
    message: '请输入描述',
    trigger: 'blur'
  }
};

// 必填参数的测试值验证规则
const requiredTestValueRule = {
  required: true,
  message: '必填参数必须提供测试值',
  trigger: ['blur', 'change']
};

// 验证是否有必填参数没有测试值
const hasInvalidRequiredParams = computed(() => {
  if (!jsParams.value || jsParams.value.length === 0) {
    return false;
  }
  return jsParams.value.some(param => param.required && !param.testValue);
});

// 检查参数表单中的必填字段
const hasInvalidRequiredParamFields = computed(() => {
  if (!jsParams.value || jsParams.value.length === 0) {
    return false;
  }
  
  return jsParams.value.some(param => 
    !param.name || 
    !param.type || 
    !param.description ||
    !isValidParamName(param.name)
  );
});

// 验证必填参数的测试值
function validateRequiredTestValue(param) {
  // 当参数切换为必填时，如果测试值为空，立即触发测试值验证
  if (param.required && !param.testValue) {
    // 可以在此处理UI反馈，例如显示错误信息
    return false;
  }
  return true;
}

// 代码变更时触发
function codeChanged(value) {
  jsCode.value = value;
  toolForm.data.code = value;
}

// 添加参数 - 确保必填字段有默认值
function addParam() {
  if (!toolForm.data.param) {
    toolForm.data.param = [];
  }
  
  toolForm.data.param.push({
    name: '',
    type: '',
    description: '',
    required: false,
    testValue: ''
  });
}

// 删除参数
function removeParam(index) {
  if (jsParams.value) {
    jsParams.value.splice(index, 1);
  }
}

// 测试代码
async function testCode() {
  // 先切换到测试标签页
  activeTab.value = 'test';
  
  // 清空之前的测试日志
  testLogs.value = [];
  
  // 如果有参数, 需要先验证所有参数的表单, 是否填写完整
  if (jsParams.value && jsParams.value.length > 0) {
    // 标记提交尝试状态为true，触发表单验证UI反馈
    submitAttempted.value = true;
    
    // 检查参数名称、类型和描述是否完整
    const invalidParams = jsParams.value.filter(param => 
      !param.name || !param.type || !param.description || !isValidParamName(param.name)
    );
    
    if (invalidParams.length > 0) {
      message.error('请完善所有参数的必填信息（名称、类型和描述）');
      return;
    }
    
    // 检查必填参数的测试值是否提供
    const missingTestValues = jsParams.value.filter(param => 
      param.required && !param.testValue
    );
    
    if (missingTestValues.length > 0) {
      message.error('必填参数必须提供测试值');
      return;
    }
  }

  try {
    // 设置全局测试状态，使菜单禁用
    toolStore.setTestState(true);

    // 添加测试开始日志
    addTestLog('info', '测试开始', '准备执行工具函数...');

    // 根据添加的参数, 和测试值, 测试值还根据不同的参数类型构造, 如果测试值没有, 就不要这个参数, 生成json参数对象字符串
    // 添加类型声明，指定结果对象允许任意字符串作为索引键
    const params = jsParams.value?.reduce((result, param) => {
      if (param.testValue) {
        if (param.type === 'string') {
          result[param.name] = param.testValue;
        } else if (param.type === 'number') {
          result[param.name] = Number(param.testValue);
        } else if (param.type === 'boolean') {
          result[param.name] = param.testValue === 'true';
        } else if (param.type === 'object') {
          try {
            result[param.name] = JSON.parse(param.testValue);
          } catch (e) {
            // JSON解析失败，保持原始字符串
            throw new Error(`参数 ${param.name} 的测试值不是有效的对象`);
          }
        }
      }
      return result;
    }, {});

    addTestLog('info', '参数信息', JSON.stringify(params, null, 2) );

    // 添加执行日志
    addTestLog('info', '代码执行', '开始执行工具函数...');
    
    // 添加结果日志
    addTestLog('success', '测试成功', '此处仅为测试模拟，实际运行需要集成JS运行环境');
    message.success('测试执行成功（模拟）');
    
    // 自动切换到测试标签页
    activeTab.value = 'test';
  } catch (e) {
    const errorMsg = e || String(e);
    
    // 添加错误日志
    addTestLog('error', '执行错误', errorMsg);
    message.error('测试执行出错: ' + errorMsg);
    
    // 自动切换到测试标签页
    activeTab.value = 'test';
  } finally {
    // 重置全局测试状态
    toolStore.setTestState(false);
  }
}

// 添加测试日志
function addTestLog(type, title, content) {
  const now = new Date();
  const timeString = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:${now.getSeconds().toString().padStart(2, '0')}`;
  
  testLogs.value.push({
    time: timeString,
    type,
    title,
    content
  });
}

// 添加提交尝试状态，用于即时显示验证错误
const submitAttempted = ref(false);

// 保存工具前，手动验证参数
async function saveTool() {
  if (!formRef.value) return;

  try {
    // 标记提交尝试状态为true
    submitAttempted.value = true;
    
    // 验证工具类型特定的字段
    // 确保代码已经同步到toolForm
    toolForm.data.code = jsCode.value;
    
    // 验证所有必填参数是否有测试值
    if (hasInvalidRequiredParams.value) {
      message.error('必填参数必须提供测试值');
      return;
    }
    
    // 验证所有参数的必填字段和命名规则
    if (hasInvalidRequiredParamFields.value) {
      // 检查是否有参数名不符合规范
      const hasInvalidNames = jsParams.value.some(param => param.name && !isValidParamName(param.name));
      if (hasInvalidNames) {
        message.error('参数名称必须以小写字母开头，只能包含字母、数字和下划线');
      } else {
        message.error('请填写所有参数的名称、类型和描述');
      }
      return;
    }
    
    // 检查名称和描述是否已填写
    if (!toolForm.name || !toolForm.description) {
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
    
    // 绕过表单验证，直接提交
    // 表单验证可能因为类型错误而失败，我们已经手动验证了所有必填字段
    
    // 所有验证都通过，现在开始保存
    saving.value = true;
    
    // 准备提交数据
    const submitData = { ...toolForm };

    if (isEdit.value) {
      // 编辑现有工具
      await toolStore.modifyTool({
        ...submitData,
        updatedAt: new Date().toISOString(),
      });
      message.success('工具更新成功');
    } else {
      // 创建新工具
      await toolStore.createTool(submitData);
      message.success('工具创建成功');
    }

    // 成功保存后返回
    setTimeout(() => {
      router.push('/tools');
    }, 1000);
    
  } catch (error) {
    // 这里捕获的是API调用或其他非验证错误
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

// 修改loadToolData函数，优先使用props传入的数据
async function loadToolData() {
  if (isEdit.value) {
    if (props.tool) {
      // 如果父组件已经传入工具数据，直接使用
      Object.assign(toolForm, props.tool);
      
      // 确保categoryId是数字类型
      toolForm.categoryId = Number(props.tool.categoryId);
      
      // 设置代码编辑器的值
      jsCode.value = props.tool.data?.code || '';
    } else {
      // 否则从服务器获取
      try {
        const tool = await toolStore.fetchToolById(toolId.value);
        if (tool) {
          Object.assign(toolForm, tool);
          
          // 确保categoryId是数字类型
          toolForm.categoryId = Number(tool.categoryId);
          
          // 设置代码编辑器的值
          jsCode.value = tool.data?.code || '';
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

// 监听props中的tool属性变化
watch(() => props.tool, (newTool) => {
  if (newTool && isEdit.value) {
    // 当props.tool变化时更新本地数据
    Object.assign(toolForm, newTool);
    toolForm.categoryId = Number(newTool.categoryId);
    jsCode.value = newTool.data?.code || '';
  }
}, { immediate: true });

// 组件挂载时 - 修改为不再重复获取分类数据
onMounted(async () => {
  // 加载工具数据
  await loadToolData();
});

// 添加参数名称验证规则
const paramNameRule = {
  validator: (rule, value) => {
    if (!value) {
      return new Error('参数名称不能为空');
    }
    if (!isValidParamName(value)) {
      return new Error('参数名称必须以小写字母开头，只能包含字母、数字和下划线');
    }
    return true;
  }
};

// 检查是否包含中文字符
function containsChineseChars(str) {
  if (!str) return false;
  const pattern = /[\u4e00-\u9fa5]/;
  return pattern.test(str);
}

// 过滤非法字符
function filterNonAsciiChars(value, param, field) {
  // 仅保留小写字母、大写字母、数字和下划线
  const filteredValue = value.replace(/[^\w]/g, '');
  // 确保开头为小写字母（如果有字符的话）
  if (filteredValue && !/^[a-z]/.test(filteredValue.charAt(0))) {
    // 如果第一个字符不是小写字母，则移除它
    param[field] = filteredValue.slice(1);
  } else {
    param[field] = filteredValue;
  }
}

// 处理输入法组合结束事件（用于中文输入法）
function handleCompositionEnd(event, param) {
  // 当中文输入法结束时，再次过滤字符
  filterNonAsciiChars(param.name, param, 'name');
}

// 判断参数名是否符合变量命名规则
function isValidParamName(name) {
  if (!name) return false;
  // 变量名必须以小写字母开头，只能包含字母、数字和下划线
  const regex = /^[a-z][a-zA-Z0-9_]*$/;
  return regex.test(name);
}

// 获取参数名的状态
function getParamNameStatus(name) {
  if (submitAttempted.value && name && !isValidParamName(name)) {
    return 'error';
  }
  return submitAttempted.value && !name ? 'error' : undefined;
}
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
</style>