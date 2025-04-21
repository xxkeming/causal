<template>
  <n-modal 
    v-model:show="visible" 
    :mask-closable="false" 
    class="agent-form-modal"
    style="width: 800px; margin-top: 80px;"
    preset="dialog"
    :transform-origin="'center'"
    :positive-text="isEdit ? '保存' : '创建'"
    negative-text="取消"
    @negative-click="handleCancel"
    @positive-click="handleSubmit"
    :title="isEdit ? '编辑智能体' : '创建智能体'"
  >
    <template #icon>
      <n-icon>
        <CreateOutline v-if="isEdit" />
        <AddCircleOutline v-else />
      </n-icon>
    </template>
    
    <n-scrollbar style="max-height: 70vh">
      <n-tabs type="line" animated lazy v-model:value="activeTab">
        <!-- 基本信息标签页 -->
        <n-tab-pane name="basic" tab="基本信息">
          <n-form
            ref="basicFormRef"
            :model="formModel"
            :rules="basicRules"
            label-placement="left"
            label-width="80"
            require-mark-placement="right-hanging"
          >
            <n-form-item label="图标" path="iconId">
              <avatar-selector v-model="formModel.iconId" :previewSize="40" />
            </n-form-item>

            <n-form-item label="名称" path="name">
              <n-input 
                v-model:value="formModel.name" 
                placeholder="输入智能体名称" 
                :style="{ width: '320px' }" 
                :status="submitAttempted && !formModel.name ? 'error' : undefined"
              />
              <template #feedback>
                <span v-if="submitAttempted && !formModel.name" class="error-text">
                  请输入智能体名称
                </span>
              </template>
            </n-form-item>
            
            <n-form-item label="描述" path="description">
              <n-input
                v-model:value="formModel.description"
                type="textarea"
                placeholder="描述此智能体的功能"
                :autosize="{ minRows: 2, maxRows: 4 }"
                :style="{ width: '100%', maxWidth: '600px' }"
              />
            </n-form-item>
            
            <n-form-item label="所属分类" path="categoryId">
              <n-select
                v-model:value="formModel.categoryId"
                :options="categoryOptions"
                placeholder="选择分类"
                :style="{ width: '250px' }"
                :status="submitAttempted && !formModel.categoryId ? 'error' : undefined"
              />
              <template #feedback>
                <span v-if="submitAttempted && !formModel.categoryId" class="error-text">
                  请选择分类
                </span>
              </template>
            </n-form-item>
            
          </n-form>
        </n-tab-pane>
        
        <!-- 模型设置标签页 -->
        <n-tab-pane name="model" tab="模型设置">
          <div class="tab-header-with-tooltip">
            <div class="tab-header-title">
              <n-icon size="18" class="tab-header-icon"><SettingsOutline /></n-icon>
              <span>选择大语言模型和配置提示词,不同模型有不同的能力，提示词定义了智能体的行为方式。</span>
            </div>
          </div>
          
          <n-form
            ref="modelFormRef"
            :model="formModel"
            :rules="modelRules"
            label-placement="left"
            label-width="auto"
            require-mark-placement="right-hanging"
          >
            <n-form-item label="选择模型" path="model" required>
              <model-selector
                v-model="selectedModelValue"
                @change="handleModelChange"
                :default-tags="['推理']"
                :show-tag-filter="false"
                :style="{ width: '380px' }"
                :status="submitAttempted && !formModel.model ? 'error' : undefined"
              />
              <template #feedback>
                <span v-if="submitAttempted && !formModel.model" class="error-text">
                  请选择模型
                </span>
              </template>
            </n-form-item>
            
            <n-form-item path="prompt">
              <template #label>
                <div class="label-with-help">
                  <n-popover trigger="hover" placement="top">
                    <template #trigger>
                      <n-icon size="16" class="help-icon">
                        <HelpCircleOutline />
                      </n-icon>
                    </template>
                    <span class="popover-content">提示词定义了智能体的角色、行为方式和知识领域。高质量的提示词可以显著提高智能体的回答质量。</span>
                  </n-popover>
                  <span style="margin-left: 8px;">提示词</span>
                </div>
              </template>
              <div class="prompt-textarea-container">
                <div class="prompt-editor">
                  <n-input
                    v-model:value="formModel.prompt"
                    type="textarea"
                    placeholder="输入智能体提示词"
                    :autosize="false"
                    :style="{ width: '100%' }"
                    class="prompt-textarea"
                    :status="submitAttempted && !formModel.prompt ? 'error' : undefined"
                  />
                  <div class="prompt-editor-tools">
                    <n-text class="prompt-character-count" :depth="3">{{ formModel.prompt.length }} 字符</n-text>
                  </div>
                </div>
              </div>
              <template #feedback>
                <span v-if="submitAttempted && !formModel.prompt" class="error-text">
                  请输入提示词
                </span>
              </template>
            </n-form-item>
          </n-form>
        </n-tab-pane>
        
        <!-- 高级设置标签页 -->
        <n-tab-pane name="advanced" tab="高级设置">
          <div class="tab-header-with-tooltip">
            <div class="tab-header-title">
              <n-icon size="18" class="tab-header-icon"><OptionsOutline /></n-icon>
              <span>调整模型参数以控制输出内容的特性,这些参数影响模型生成内容的风格、随机性和长度。</span>
            </div>
          </div>
        
          <n-form
            :model="formModel"
            label-placement="left"
            label-width="auto"
            require-mark-placement="right-hanging"
          >
            <n-form-item>
              <template #label>
                <div class="label-with-help">
                  <n-popover trigger="hover" placement="top">
                    <template #trigger>
                      <n-icon size="16" class="help-icon">
                        <HelpCircleOutline />
                      </n-icon>
                    </template>
                    <span class="popover-content">控制回复的随机性和创造性。值越高，回复越多样但可能不准确；值越低，回复越可预测和保守。</span>
                  </n-popover>
                  <span style="margin-left: 8px;">温度</span>
                </div>
              </template>
              <div class="slider-block">
                <n-slider
                  v-model:value="formModel.temperature"
                  :min="0"
                  :max="2"
                  :step="0.1"
                  :tooltip="true"
                  :marks="temperatureMarks"
                  :style="{ width: '400px' }"
                />
                <div class="value-display">
                  {{ formModel.temperature.toFixed(1) }}
                </div>
              </div>
            </n-form-item>

            <n-form-item>
              <template #label>
                <div class="label-with-help">
                  <n-popover trigger="hover" placement="top">
                    <template #trigger>
                      <n-icon size="16" class="help-icon">
                        <HelpCircleOutline />
                      </n-icon>
                    </template>
                    <span class="popover-content">最高概率采样，值越大，回复内容越赋有多样性、创造性、随机性；设为0根据事实回答。日常聊天建议设置为0.9</span>
                  </n-popover>
                  <span style="margin-left: 8px;">Top P</span>
                </div>
              </template>
              <div class="slider-block">
                <n-slider
                  v-model:value="formModel.topP"
                  :min="0"
                  :max="1"
                  :step="0.1"
                  :tooltip="true"
                  :marks="topPMarks"
                  :style="{ width: '400px' }"
                />
                <div class="value-display">
                  {{ formModel.topP.toFixed(1) }}
                </div>
              </div>
            </n-form-item>

            <n-form-item>
              <template #label>
                <div class="label-with-help">
                  <n-popover trigger="hover" placement="top">
                    <template #trigger>
                      <n-icon size="16" class="help-icon">
                        <HelpCircleOutline />
                      </n-icon>
                    </template>
                    <div class="popover-content">
                      0 表示不限制。限制模型生成的最大token数量，一个token约等于0.75个英文单词或0.4个汉字。
                    </div>
                  </n-popover>
                  <span style="margin-left: 8px;">Max Token</span>
                </div>
              </template>
              <div class="form-item-row">
                <n-input-number
                  v-model:value="formModel.maxTokens"
                  :min="0"
                  :step="100"
                  :style="{ width: '160px' }"
                />
              </div>
            </n-form-item>
        
            <n-form-item>
              <template #label>
                <div class="label-with-help">
                  <n-popover trigger="hover" placement="top">
                    <template #trigger>
                      <n-icon size="16" class="help-icon">
                        <HelpCircleOutline />
                      </n-icon>
                    </template>
                    <div class="popover-content">
                      要保留在上下文中的消息数量，数值越大，上下文越长，消耗的 token 越多。普通聊天建议 5-10
                    </div>
                  </n-popover>
                  <span style="margin-left: 8px;">记忆</span>
                </div>
              </template>
              <div class="form-item-row">
                <n-input-number
                  v-model:value="formModel.contextSize"
                  :on-update:value="val => {
                    formModel.contextSize = val ?? 0;
                    if (formModel.contextSize === 0) {
                      formModel.contextExtend = false; // 如果设置为0，自动关闭记忆保留
                    }
                  }"
                  :min="0"
                  :max="50"
                  :style="{ width: '160px' }"
                />
              </div>
            </n-form-item>

            <n-form-item>
              <template #label>
                <div class="label-with-help">
                  <n-popover trigger="hover" placement="top">
                    <template #trigger>
                      <n-icon size="16" class="help-icon">
                        <HelpCircleOutline />
                      </n-icon>
                    </template>
                    <div class="popover-content">
                      启用后会在上下文中保留上传的附件数据，消耗更多token但能增强连续性
                    </div>
                  </n-popover>
                  <span style="margin-left: 8px;">记忆保留附件</span>
                </div>
              </template>
              <div class="form-item-row">
                <n-switch 
                  v-model:value="formModel.contextExtend"
                  :on-update:value="val => {
                    formModel.contextExtend = val;
                    if (val && formModel.contextSize === 0) {
                      formModel.contextSize = 5; // 如果开启且记忆为0,自动设置为默认值
                    }
                  }"
                />
              </div>
            </n-form-item>

          </n-form>
        </n-tab-pane>
        
        <!-- 工具设置标签页 -->
        <n-tab-pane name="tools" tab="工具设置">
          <div class="tab-header-with-tooltip">
            <div class="tab-header-title">
              <n-icon size="18" class="tab-header-icon"><BuildOutline /></n-icon>
              <span>为智能体配置工具，赋予额外的能力，例如搜索网络、执行代码、分析数据等。</span>
            </div>
          </div>
          
          <div class="tool-selector-container">
            <tool-selector v-model="formModel.tools" />
          </div>
        </n-tab-pane>
        
        <!-- 自定义参数标签页 -->
        <n-tab-pane name="params" tab="自定义参数">
          <div class="tab-header-with-tooltip">
            <div class="tab-header-title">
              <n-icon size="18" class="tab-header-icon"><CodeOutline /></n-icon>
              <span>自定义参数可以被API调用或智能体内部逻辑使用，用于自定义智能体的特定行为。</span>
            </div>
          </div>

          <n-scrollbar style="max-height: 400px">
            <div class="params-list">
              <!-- 参数项列表 -->
              <div v-for="(param, index) in formModel.params" :key="index" class="param-item">
                <div class="param-fields-row">
                  <div class="param-title">
                    <n-text strong>{{ index + 1 }}</n-text>
                  </div>
                  
                  <n-input 
                    v-model:value="param.name" 
                    placeholder="参数名称" 
                    class="param-name-input" 
                  />
                  
                  <n-select
                    v-model:value="param.type"
                    :options="paramTypeOptions"
                    placeholder="类型"
                    class="param-type-select"
                  />
                  
                  <n-input 
                    v-model:value="param.value" 
                    placeholder="参数值" 
                    class="param-value-input"
                    :type="param.type === 'number' ? 'text' : 'text'"
                  />
                  
                  <n-button
                    circle
                    quaternary
                    type="error"
                    size="small"
                    @click="removeParam(index)"
                    class="delete-button"
                  >
                    <template #icon><n-icon><TrashOutline /></n-icon></template>
                  </n-button>
                </div>
              </div>
              
              <!-- 内联添加表单 - 始终显示 -->
              <div class="param-item param-inline-add">
                <div class="param-fields-row new-param-row">
                  <div class="param-title">
                    <n-text type="info">+</n-text>
                  </div>
                  
                  <n-input 
                    v-model:value="newParam.name" 
                    placeholder="参数名称" 
                    class="param-name-input" 
                  />
                  
                  <n-select
                    v-model:value="newParam.type"
                    :options="paramTypeOptions"
                    placeholder="类型"
                    class="param-type-select"
                  />
                  
                  <n-input 
                    v-model:value="newParam.value" 
                    placeholder="参数值" 
                    class="param-value-input"
                    :type="newParam.type === 'number' ? 'text' : 'text'"
                  />
                  
                  <n-button 
                    circle 
                    type="primary"
                    size="small"
                    @click="addInlineParam"
                    :disabled="!newParam.name"
                    class="inline-add-button"
                    title="添加参数"
                  >
                    <template #icon><n-icon><AddOutline /></n-icon></template>
                  </n-button>
                </div>
              </div>

              <!-- 当没有参数时显示提示 -->
              <div v-if="formModel.params.length === 0" class="no-params-tip">
                <n-text depth="3">还没有参数，请使用上面的表单添加</n-text>
              </div>
            </div>
          </n-scrollbar>
        </n-tab-pane>
        
        <!-- 添加自定义问题标签页 -->
        <n-tab-pane name="questions" tab="自定义问题">
          <div class="tab-header-with-tooltip">
            <div class="tab-header-title">
              <n-icon size="18" class="tab-header-icon"><ChatbubbleOutline /></n-icon>
              <span>设置常用问题，让用户快速了解智能体的功能和特点。</span>
            </div>
          </div>

          <n-scrollbar style="max-height: 400px">
            <div class="questions-list">
              <!-- 问题列表 -->
              <div v-for="(_question, index) in formModel.customQuestions" :key="index" class="question-item">
                <div class="question-fields-row">
                  <div class="question-title">
                    <n-text strong>{{ index + 1 }}</n-text>
                  </div>
                  
                  <n-input 
                    v-model:value="formModel.customQuestions[index]" 
                    type="text"
                    placeholder="输入常用问题" 
                    class="question-input"
                  />
                  
                  <n-button
                    circle
                    quaternary
                    type="error"
                    size="small"
                    @click="removeQuestion(index)"
                    class="delete-button"
                  >
                    <template #icon><n-icon><TrashOutline /></n-icon></template>
                  </n-button>
                </div>
              </div>
              
              <!-- 添加问题输入框 -->
              <div class="question-item question-inline-add">
                <div class="question-fields-row new-question-row">
                  <div class="question-title">
                    <n-text type="info">+</n-text>
                  </div>
                  
                  <n-input 
                    v-model:value="newQuestion"
                    type="text"
                    placeholder="添加新的常用问题" 
                    class="question-input"
                    @keyup.enter="addQuestion"
                  />
                  
                  <n-button 
                    circle 
                    type="primary"
                    size="small"
                    @click="addQuestion"
                    :disabled="!newQuestion"
                    class="inline-add-button"
                    title="添加问题"
                  >
                    <template #icon><n-icon><AddOutline /></n-icon></template>
                  </n-button>
                </div>
              </div>

              <!-- 提示文本 -->
              <div v-if="formModel.customQuestions.length === 0" class="no-questions-tip">
                <n-text depth="3">还没有自定义问题，请添加常见问题</n-text>
              </div>
            </div>
          </n-scrollbar>
        </n-tab-pane>
      </n-tabs>
    </n-scrollbar>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch } from 'vue';
import {
  NModal, NButton, NForm, NFormItem, NInput, NInputNumber,
  NSelect, NIcon, NSlider, NText,
  NScrollbar, NTabs, NTabPane, NSwitch,
  NPopover, useMessage
} from 'naive-ui';
import { 
  AddOutline, TrashOutline, HelpCircleOutline,
  SettingsOutline, OptionsOutline, BuildOutline, CodeOutline,
  AddCircleOutline, CreateOutline, ChatbubbleOutline
} from '@vicons/ionicons5';
import { Agent, ModelParam, ProviderModel } from '../../../services/typings';
import { useAgentCategoryStore } from '../../../stores/agentCategoryStore';
import AvatarSelector from '../../../components/AvatarSelector.vue';
import ModelSelector from '../../../components/ModelSelector.vue';
// 导入工具选择器组件
import ToolSelector from '../../../components/ToolSelector.vue';

// Props定义
const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  },
  isEdit: {
    type: Boolean,
    default: false
  },
  agentData: {
    type: Object as () => Agent | null,
    default: null
  },
  defaultCategoryId: {
    type: Number,
    default: null
  }
});

const emit = defineEmits(['update:visible', 'submit', 'cancel']);

// 创建计算属性处理双向绑定
const visible = computed({
  get: () => props.visible,
  set: value => emit('update:visible', value)
});

// 使用分类Store
const categoryStore = useAgentCategoryStore();

// 初始化表单数据
const formModel = reactive<{
  name: string;
  description: string;
  categoryId: number | null;
  iconId: number; // 将 avatarId 改为 iconId
  model?: ProviderModel;
  prompt: string;
  temperature: number;
  topP: number; // 添加 topP 属性
  maxTokens: number;
  contextSize: number;
  contextExtend: boolean;
  tools: number[];
  params: ModelParam[];
  customQuestions: string[];
}>({
  name: '',
  description: '',
  categoryId: null,
  iconId: 0, // 将 avatarId 改为 iconId
  prompt: '',
  temperature: 0.7,
  topP: 0.9, // 初始化 topP 属性
  maxTokens: 0,
  contextSize: 5,
  contextExtend: false,
  tools: [],
  params: [],
  customQuestions: []
});

// 添加模型选择器相关状态
const selectedModelValue = ref<string | undefined>(undefined);

// 处理模型选择变更
const handleModelChange = (modelInfo: { providerId: number, modelName: string } | null) => {
  if (modelInfo) {
    console.log('接收到模型选择变更:', modelInfo);
    
    selectedModelValue.value = modelInfo.providerId + '|' + modelInfo.modelName;
    
    // 确保 providerId 是数字类型
    const providerId = typeof modelInfo.providerId === 'string' 
      ? parseInt(modelInfo.providerId, 10) 
      : modelInfo.providerId;
    
    formModel.model = {
      id: providerId,
      name: modelInfo.modelName
    };
    
    console.log('设置表单模型值:', formModel.model);
  } else {
    formModel.model = undefined;
  }
};

// 温度标记
const temperatureMarks = {
  0: '事实性',
  1: '平衡',
  2: '创造性'
};

// topP 标记
const topPMarks = {
  0: '0',
  0.5: '0.5',
  1: '1'
};

// 分类选项
const categoryOptions = computed(() => {
  return categoryStore.categories.map(category => ({
    label: category.name,
    value: category.id
  }));
});

// 参数类型选项
const paramTypeOptions = [
  { label: '字符串', value: 'string' },
  { label: '数字', value: 'number' },
  { label: '布尔值', value: 'boolean' },
  { label: '对象', value: 'object' }
];

// 删除参数
function removeParam(index: number) {
  formModel.params.splice(index, 1);
}

// 新参数临时数据
const newParam = reactive({
  name: '',
  type: 'string',
  value: ''
});

// 内联添加参数方法
function addInlineParam() {
  if (!newParam.name) return;
  
  formModel.params.push({
    name: newParam.name,
    type: newParam.type,
    value: newParam.value
  });
  
  // 重置表单
  newParam.name = '';
  newParam.type = 'string';
  newParam.value = '';
}

// 新问题输入框的值
const newQuestion = ref('');

// 添加问题方法
function addQuestion() {
  if (!newQuestion.value) return;
  
  if (!formModel.customQuestions) {
    formModel.customQuestions = [];
  }
  
  formModel.customQuestions.push(newQuestion.value);
  newQuestion.value = '';
}

// 删除问题方法
function removeQuestion(index: number) {
  formModel.customQuestions.splice(index, 1);
}

// 表单引用
const basicFormRef = ref(null);
const modelFormRef = ref(null);

// 当前激活的标签页
const activeTab = ref('basic');

// 拆分表单验证规则，对应不同表单
const basicRules = {
  name: {
    required: true,
    message: '请输入智能体名称',
    trigger: 'blur'
  },
  categoryId: {
    required: true,
    message: '请选择分类',
    trigger: 'change'
  }
};

// 处理模型验证规则
const modelRules = {
  model: {
    required: true,
    message: '请选择模型',
    trigger: 'change',
    validator: (_rule: any, _value: any) => {
      if (!formModel.model) {
        return new Error('请选择模型');
      }
      return true;
    }
  },
  prompt: {
    required: true,
    message: '请输入提示词',
    trigger: 'blur'
  }
};

// 使用消息组件
const message = useMessage();

// 提交尝试状态
const submitAttempted = ref(false);

// 处理提交
async function handleSubmit() {

  // 标记已尝试提交
  submitAttempted.value = true;
  
  // 检查基本必填字段
  if (!formModel.name) {
    activeTab.value = 'basic';
    message.error('表单验证失败，请输入智能体名称');
    return false;
  }
  
  if (!formModel.categoryId) {
    activeTab.value = 'basic';
    message.error('表单验证失败，请选择分类');
    return false;
  }
  
  // 检查模型和提示词
  if (!formModel.model) {
    activeTab.value = 'model';
    message.error('表单验证失败，请选择模型');
    return false;
  }
  
  if (!formModel.prompt) {
    activeTab.value = 'model';
    message.error('表单验证失败，请输入提示词');
    return false;
  }

  // 通过所有验证后，构造提交数据
  const submitData = {
    ...formModel,
    // 如果是编辑模式且有id，则保留id
    ...(props.isEdit && props.agentData ? { id: props.agentData.id } : {})
  };
  
  // 提交数据
  emit('submit', submitData);
  
  // 验证通过后关闭模态框
  // emit('update:visible', false);
  return false;
}

// 处理取消
function handleCancel() {
  emit('cancel');
  emit('update:visible', false);
}

// 监听visible变化，用于重置表单
watch(() => props.visible, (visible) => {
  if (!visible) {
    // 移除条件 !props.isEdit，使编辑模式下关闭模态框也会重置表单
    // 重置表单
    formModel.name = '';
    formModel.description = '';
    formModel.categoryId = null;
    formModel.iconId = 0;
    formModel.model = undefined;
    formModel.prompt = '';
    formModel.temperature = 0.7;
    formModel.topP = 0.9;
    formModel.maxTokens = 0;
    formModel.contextSize = 5;
    formModel.contextExtend = false;
    formModel.tools = [];
    formModel.params = [];
    formModel.customQuestions = [];
    newQuestion.value = '';
    
    // 重置模型选择器值
    selectedModelValue.value = undefined;

    // 重置验证状态
    submitAttempted.value = false;

    activeTab.value = 'basic';
  } else if (props.isEdit && props.agentData) {
    // 如果是可见状态且是编辑模式，则填充表单数据
    // 这部分代码从watch agentData部分移动过来，确保每次打开编辑窗口都会刷新数据
    formModel.name = props.agentData.name || '';
    formModel.description = props.agentData.description || '';
    formModel.categoryId = props.agentData.categoryId;
    formModel.iconId = props.agentData.iconId || 0;
    formModel.model = props.agentData.model;
    formModel.prompt = props.agentData.prompt || '';
    formModel.temperature = props.agentData.temperature;
    formModel.topP = props.agentData.topP;
    formModel.maxTokens = props.agentData.maxTokens;
    formModel.contextSize = props.agentData.contextSize;
    formModel.contextExtend = props.agentData.contextExtend || false;
    formModel.tools = props.agentData.tools || [];
    formModel.params = props.agentData.params ? props.agentData.params.map(p => ({
      name: p.name,
      type: p.type,
      value: p.value,
      description: '',
      required: false
    })) : [];
    formModel.customQuestions = props.agentData.customQuestions || [];
    
    if (props.agentData.model && props.agentData.model.id !== undefined && props.agentData.model.name) {
      selectedModelValue.value = props.agentData.model.id + '|' + props.agentData.model.name;
    }
  } else if (!props.isEdit) {
    // 非编辑模式打开时，如果有默认分类ID且不是'all'，则设置为默认分类
    if (props.defaultCategoryId && props.defaultCategoryId !== 0) {
      formModel.categoryId = props.defaultCategoryId;
    }
  }
});

// 组件挂载时，确保分类数据已加载
if (categoryStore.categories.length === 0) {
  categoryStore.fetchCategories();
}
</script>

<style scoped>
/* 自定义模态框样式 */
.agent-form-modal :deep(.n-card) {
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
}

/* 添加靠上显示的样式 */
:deep(.n-modal-container) {
  margin-top: 8vh !important;
  align-items: flex-start !important;
}

:deep(.n-modal-body-wrapper) {
  align-items: flex-start !important;
}

/* 确保tabs内容有适当的内边距 */
:deep(.n-tab-pane) {
  padding: 16px 20px;
}

/* 已有样式 */
.empty-params {
  display: flex;
  justify-content: center;
  padding: 20px;
  border: 1px dashed var(--border-color);
  border-radius: 6px;
  margin-bottom: 20px;
}

.param-item {
  margin-bottom: 0; /* 从16px改为0，移除额外的下边距 */
}

.params-header {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 16px;
}

.param-description {
  margin-top: 5px;
}

/* 问号图标提示按钮样式 */
.help-button {
  color: var(--text-color-3);
  margin-left: 8px;
  opacity: 0.7;
}

.help-button:hover {
  opacity: 1;
  color: var(--primary-color);
}

.help-button-vertical {
  color: var(--text-color-3);
  opacity: 0.7;
  margin-left: 8px;
  align-self: flex-start;
  margin-top: 8px;
  cursor: pointer;
}

.help-button-vertical:hover,
.help-button-vertical:focus {
  opacity: 1;
  color: var(--primary-color);
}

/* 表单项行样式 */
.form-item-row {
  display: flex;
  align-items: center;
}

/* 滑块和值显示布局 */
.slider-block {
  display: flex;
  align-items: center;
  gap: 16px;
}

.value-display {
  width: 32px;
  font-weight: 500;
  text-align: center;
  color: var(--text-color);
}

/* 弹出提示框内容样式 */
.popover-content {
  font-size: 13px;
  line-height: 1.6;
  max-width: 300px;
}

/* 移除旧的帮助文本样式 */
.form-item-help {
  display: none;
}

/* 模态框样式调整 */
:deep(.n-card-header) {
  padding: 12px 16px; /* 减少上下左右的内边距 */
}

:deep(.n-card-header__main) {
  font-size: 16px;
  font-weight: 500;
}

:deep(.n-card-header__extra) {
  margin-right: -4px; /* 让关闭按钮靠近右边框 */
}

:deep(.n-card-header__main) {
  margin-left: -4px; /* 让标题靠近左边框 */
}

:deep(.n-form-item .n-form-item-feedback-wrapper) {
  min-height: 0;
}

:deep(.n-form-item) {
  margin-bottom: 24px;
}

:deep(.n-tab-pane) {
  padding: 12px 0;
}

/* 添加一致性样式 */
.form-item-help {
  margin-top: 8px;
  padding-left: 2px;
}

.form-item-help .n-text {
  display: block;
  font-size: 12px;
  line-height: 1.5;
  color: var(--text-color-3);
}

.form-item-help .n-text:first-child {
  margin-bottom: 4px;
  font-weight: 500;
}

/* 修改带帮助按钮的标签样式 */
.label-with-help {
  display: flex;
  align-items: center;
}

/* 标签页标题区域样式 */
.tab-header-with-tooltip {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 24px;
  padding-bottom: 10px;
}

.tab-header-title {
  display: flex;
  align-items: center;
  font-weight: 500;
  font-size: 15px;
  color: var(--text-color);
}

.tab-header-icon {
  margin-right: 8px;
  color: var(--primary-color);
}

/* 提示词文本框样式 - 彻底移除拖动图标 */
.prompt-textarea-container {
  position: relative;
  width: 100%;
  /* max-width: 800px; */
  overflow: hidden;
}

.prompt-editor {
  position: relative;
  width: 100%;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  transition: border-color 0.3s, box-shadow 0.3s;
  overflow: hidden; /* 防止任何溢出元素显示 */
}

.prompt-textarea {
  border: none !important;
  height: 250px !important;
  box-shadow: none !important;
  border-radius: 6px !important;
  padding-bottom: 32px !important;
}

/* 更强大的选择器，确保覆盖所有可能的框架样式 */
.prompt-textarea :deep(.n-input__textarea),
.prompt-textarea :deep(.n-input__textarea-el),
:deep(.prompt-textarea .n-input__textarea-el) {
  height: 100% !important;
  min-height: 250px;
  max-height: 250px;
  resize: none;
}

/* 全局样式覆盖 */
:deep(.n-input__textarea-el) {
  resize: none;
}

/* 使用伪元素覆盖右下角可能存在的拖动图标区域 */
.prompt-editor::after {
  content: "";
  position: absolute;
  bottom: 0;
  right: 0;
  width: 20px;
  height: 20px;
  background-color: var(--card-color);
  z-index: 2; /* 确保在拖动图标之上 */
  pointer-events: none; /* 允许点击穿透 */
}

.prompt-editor-tools {
  position: absolute;
  bottom: 4px;
  right: 8px;
  display: flex;
  align-items: center;
  padding: 0;
  gap: 8px;
  background-color: transparent; /* 移除背景色，设为完全透明 */
  border-top-left-radius: 0; /* 移除圆角 */
  bottom: 4px;
  right: 8px;
  display: flex;
  align-items: center;
  padding: 0;
  gap: 8px;
  background-color: transparent; /* 移除背景色，设为完全透明 */
  border-top-left-radius: 0; /* 移除圆角 */
  backdrop-filter: none; /* 移除模糊效果 */
  z-index: 1;
}

.prompt-character-count {
  font-size: 12px;
  padding: 0;
  color: var(--text-color-3); /* 确保在透明背景下文字颜色足够清晰 */
}

/* 为暗色模式调整文字颜色 - 保留但简化这部分 */
html.dark .prompt-character-count {
  color: rgba(255, 255, 255, 0.6); /* 暗色模式下更亮的文字颜色 */
}

/* 参数列表样式 - 不再使用卡片形式 */
.params-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  /* padding: 4px; */
  margin-bottom: 16px;
  width: 100%;
}

.param-item {
  transition: transform 0.2s;
}

/* 参数字段行样式 */
.param-fields-row {
  display: flex;
  gap: 8px;
  width: 100%;
  align-items: center;
  background-color: var(--card-color);
}

.new-param-row {
  border: 1px dashed var(--border-color);
  background-color: rgba(0, 0, 0, 0.01);
}

.new-param-row:hover {
  background-color: rgba(24, 160, 88, 0.03);
}

.param-title {
  width: 30px;
  flex-shrink: 0;
  text-align: center;
}

.param-name-input {
  flex: 1.2;
}

.param-type-select {
  flex: 0.8;
}

.param-value-input {
  flex: 2;
}

.delete-button, .inline-add-button {
  flex-shrink: 0;
}

.no-params-tip {
  text-align: center;
  padding: 16px;
  color: var(--text-color-3);
}

/* 移除已不需要的卡片相关样式 */
.param-card, .inline-add-card, .add-param-card {
  display: none;
}

/* 工具选择器容器样式 */
.tool-selector-container {
  margin-top: 16px;
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

/* 删除旧的帮助按钮样式 */
.help-button, .help-button-left {
  display: none;
}

/* 添加新的帮助图标样式 */
.help-icon {
  color: var(--text-color-3);
  margin-right: 4px;
  cursor: pointer;
  opacity: 0.7;
  transition: all 0.2s;
}

.help-icon:hover {
  opacity: 1;
  color: var(--primary-color);
  transform: scale(1.1);
}

.label-with-help {
  display: flex;
  align-items: center;
}

/* 调整表单项行样式 */
.form-item-row {
  display: flex;
  align-items: center;
  gap: 8px; /* 添加间隔使布局更清晰 */
}

/* 弹出提示框内容样式 */
.popover-content {
  font-size: 13px;
  line-height: 1.6;
  max-width: 300px;
}

/* 添加自定义问题相关样式 */
.questions-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 16px;
  width: 100%;
}

.question-item {
  transition: transform 0.2s;
}

.question-fields-row {
  display: flex;
  gap: 8px;
  width: 100%;
  align-items: center;
  background-color: var(--card-color);
  padding: 4px;
}

.new-question-row {
  border: 1px dashed var(--border-color);
  background-color: rgba(0, 0, 0, 0.01);
}

.new-question-row:hover {
  background-color: rgba(24, 160, 88, 0.03);
}

.question-title {
  width: 30px;
  flex-shrink: 0;
  text-align: center;
}

.question-input {
  flex: 1;
}

.no-questions-tip {
  text-align: center;
  padding: 16px;
  color: var(--text-color-3);
}
</style>
