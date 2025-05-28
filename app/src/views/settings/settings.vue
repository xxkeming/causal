<template>
  <div class="search-settings-page">
    <div class="scrollable-container">
      <div class="content-section">
        <div class="header-section">
          <h2>系统配置</h2>
        </div>

        <n-divider />

        <div class="form-section">
          <n-form
            ref="formRef"
            :model="settings"
            :rules="rules"
            label-placement="left"
            label-width="120"
            require-mark-placement="right-hanging"
            class="config-form"
          >
            <n-form-item label="搜索服务" path="search.type.name">
              <n-select
                v-model:value="settings.search.type.name"
                :options="searchServiceOptions"
                placeholder="请选择搜索服务"
                @update:value="handleServiceChange"
                class="medium-input"
              />
            </n-form-item>

            <n-form-item label="搜索服务密钥" path="search.type.apiKey">
              <n-input
                v-model:value="settings.search.type.apiKey"
                :placeholder="`请输入${getServiceLabel(settings.search.type.name)} API Key`"
                type="password"
                show-password-on="click"
                class="large-input"
              />
            </n-form-item>

            <n-form-item label="搜索模式" path="search.mode">
              <n-radio-group v-model:value="settings.search.mode" class="radio-group">
                <n-space>
                  <n-radio :value="1">先搜索</n-radio>
                  <n-radio :value="2">工具-智能搜索</n-radio>
                </n-space>
              </n-radio-group>
            </n-form-item>

            <n-form-item label="结果数量" path="search.resultCount">
              <n-input-number
                v-model:value="settings.search.resultCount"
                :min="1"
                :max="10"
                placeholder="保留搜索结果数量"
                class="small-input"
              />
            </n-form-item>

            <n-divider />
            
            <n-form-item label="语音识别" path="transcriptions">
              <ModelSelector
                v-model="transcriptionModelValue"
                :default-tags="['语音识别']"
                :show-tag-filter="false"
                size="small"
                class="model-selector-short"
                @change="onModelChange"
              />
            </n-form-item>
          </n-form>

          <div class="action-buttons">
            <n-button
              type="primary"
              @click="saveConfig"
              :loading="saving"
            >
              保存配置
            </n-button>
            <n-button
              @click="resetConfig"
              :disabled="saving"
            >
              重置
            </n-button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { NButton, NDivider, NForm, NFormItem, NInput, NInputNumber, NRadioGroup, NRadio, NSpace, NSelect, useMessage } from 'naive-ui'
import { getSettings, setSettings } from '../../services/api'
import type { Settings, Search } from '../../services/typings'
import type { FormRules } from 'naive-ui'
import ModelSelector from '../../components/ModelSelector.vue'

const message = useMessage()
const saving = ref(false)

const searchServiceOptions = [
  {
    label: 'Tavily搜索',
    value: 'Tavily' as const
  },
]

const defaultSearch: Search = {
  type: { name: 'Tavily', apiKey: '' },
  mode: 1,
  resultCount: 5
}

const settings = ref<Settings>({
  search: { ...defaultSearch },
  transcriptions: undefined
})

// 新增：用于 ModelSelector 绑定的字符串值
const transcriptionModelValue = ref<string | undefined>(undefined)

const rules: FormRules = {
  'search.type.name': {
    required: false,
    trigger: ['blur', 'change'],
    message: '请选择搜索服务'
  },
  'search.type.apiKey': {
    required: false,
    trigger: ['blur', 'change'],
    message: '请输入API Key'
  },
  'search.mode': {
    required: false,
    trigger: ['blur', 'change'],
    type: 'number',
    message: '请选择搜索模式'
  },
  'search.resultCount': {
    required: false,
    trigger: ['blur', 'change'],
    type: 'number',
    message: '请设置结果数量'
  }
}

const formRef = ref<any>(null)

onMounted(async () => {
  try {
    const config = await getSettings()
    settings.value = config
    transcriptionModelValue.value = config.transcriptions ? `${config.transcriptions.id}|${config.transcriptions.name}` : undefined;
  } catch (error) {
    console.error('加载配置失败')
  }
})

const saveConfig = async () => {
  try {
    await formRef.value?.validate()
    saving.value = true
    await setSettings(settings.value)
    message.success('保存成功')
  } catch (error) {
    message.error('保存失败')
  } finally {
    saving.value = false
  }
}

const getServiceLabel = (name: 'Tavily' | 'Baidu') => {
  return searchServiceOptions.find(option => option.value === name)?.label || name
}

const handleServiceChange = (name: 'Tavily' | 'Baidu') => {
  if (name === 'Tavily') {
    settings.value.search.type = { name: 'Tavily', apiKey: '' }
  } else if (name === 'Baidu') {
    settings.value.search.type = { name: 'Baidu', apiKey: '' }
  }
}

// 移除对 transcriptionModelValue 的 watch，只保留 onModelChange 方法
const onModelChange = (val: any) => {
  settings.value.transcriptions = {id: Number(val.providerId), name: val.modelName};
  transcriptionModelValue.value = val ? `${val.providerId}|${val.modelName}` : undefined;
}

// 保证下列方法在 <script setup> 下可用
const resetConfig = () => {
  settings.value = {
    search: { ...defaultSearch },
    transcriptions: undefined
  }
}
</script>

<style scoped>
.search-settings-page {
  width: 100%;
  height: 100%;
  position: relative;
}

.scrollable-container {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.scrollable-container::-webkit-scrollbar {
  display: none;
}

.content-section {
  margin: 0 auto;
  padding: 20px;
}

.header-section {
  margin-bottom: 20px;
}

.header-section h2 {
  margin: 0;
  font-size: 24px;
  font-weight: 500;
}

.description {
  margin-top: 8px;
  color: var(--text-color-secondary);
}

.form-section {
  margin-top: 20px;
}

.config-form {
  width: 100%;
}

/* 输入框宽度类 */
.small-input {
  width: 120px;
}

.medium-input {
  width: 200px;
}

.large-input {
  width: 460px;
}

.radio-group {
  margin-top: 4px;
}

/* 调整表单项间距 */
:deep(.n-form-item) {
  margin-bottom: 24px;
}

/* 调整表单项标签和内容的对齐 */
:deep(.n-form-item-label) {
  line-height: 32px;
}

:deep(.n-form-item-blank) {
  display: flex;
  align-items: center;
}

.action-buttons {
  margin-top: 30px;
  display: flex;
  gap: 12px;
  justify-content: flex-start;
}

.model-selector-short {
  width: 450px;
}
</style>
