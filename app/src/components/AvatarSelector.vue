<template>
  <div class="avatar-selector-container">
    <!-- 预览区域 - 点击打开选择器 -->
    <div class="avatar-preview" @click="showPopover = true">
      <n-avatar 
        :color="currentAvatar ? currentAvatar.color : '#d9d9d9'" 
        round
        :size="previewSize"
      >
        <n-icon :size="previewIconSize" v-if="currentAvatar">
          <component :is="currentAvatar.icon" />
        </n-icon>
        <n-icon :size="previewIconSize" v-else>
          <PersonOutline />
        </n-icon>
      </n-avatar>
      <div class="avatar-preview-overlay">
        <n-icon><PencilOutline /></n-icon>
      </div>
    </div>
    
    <!-- 头像选择弹出框 -->
    <n-modal
      v-model:show="showPopover"
      preset="card"
      :style="{ width: '480px' }"
      :title="title"
      :mask-closable="true"
    >
      <div class="avatar-selector">
        <!-- 移除了头部的随机选择按钮 -->
        
        <div class="avatar-grid">
          <div 
            v-for="icon in iconStore.icons" 
            :key="icon.id" 
            class="avatar-item"
            :class="{ 'avatar-selected': tempSelectedAvatar === icon.id }"
            @click="selectAndConfirmAvatar(icon)"
          >
            <n-avatar 
              :color="icon.color" 
              round
              size="small"
            >
              <n-icon>
                <component :is="icon.icon" />
              </n-icon>
            </n-avatar>
          </div>
        </div>
      </div>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { NAvatar, NIcon, NModal } from 'naive-ui';
import { PersonOutline, PencilOutline } from '@vicons/ionicons5';
import { useIconStore, IconOption } from '../stores/iconStore';

// 初始化图标store
const iconStore = useIconStore();

// 组件属性定义
const props = defineProps({
  modelValue: {
    type: Number,
    default: 0
  },
  title: {
    type: String,
    default: '选择图标'
  },
  showRandom: {
    type: Boolean,
    default: false  // 设置默认为false，不显示随机功能
  },
  previewSize: {
    type: Number,
    default: 40
  }
});

// 计算头像图标大小
const previewIconSize = computed(() => {
  return Math.floor(props.previewSize * 0.6);
});

// 组件事件
const emit = defineEmits(['update:modelValue', 'change']);

// 控制弹出框显示
const showPopover = ref(false);
const tempSelectedAvatar = ref(props.modelValue);

// 监听modelValue变化，同步临时选择
watch(() => props.modelValue, (val) => {
  tempSelectedAvatar.value = val;
});

// 选择并直接确认头像 - 合并了选择和确认功能
const selectAndConfirmAvatar = (icon: IconOption) => {
  tempSelectedAvatar.value = icon.id;
  emit('update:modelValue', icon.id);
  emit('change', icon);
  showPopover.value = false;
};

// 获取当前选中的头像
const currentAvatar = computed(() => {
  return iconStore.getIconById(props.modelValue);
});

// 暴露方法给父组件
defineExpose({
  icons: iconStore.icons,
  currentAvatar,
  openSelector: () => { showPopover.value = true; }
});
</script>

<style scoped>
.avatar-selector-container {
  display: inline-block;
  position: relative;
}

.avatar-preview {
  position: relative;
  display: inline-flex;
  cursor: pointer;
  border-radius: 50%;
  transition: all 0.2s ease;
}

.avatar-preview:hover .avatar-preview-overlay {
  opacity: 1;
}

.avatar-preview-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.4);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.avatar-selector {
  width: 100%;
  padding: 0;
}

.avatar-grid {
  display: grid;
  grid-template-columns: repeat(10, 1fr);
  gap: 4px;
  margin-bottom: 8px;
  max-height: 300px;
  overflow-y: auto;
  padding: 4px;
}

@media (max-width: 768px) {
  .avatar-grid {
    grid-template-columns: repeat(8, 1fr);
  }
}

.avatar-item {
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  padding: 4px;
  border-radius: 50%;
  transition: all 0.2s ease;
}

.avatar-item:hover {
  background-color: rgba(0, 0, 0, 0.05);
  transform: scale(1.1);
}

.avatar-selected {
  background-color: rgba(24, 160, 88, 0.1);
  box-shadow: 0 0 0 2px rgba(24, 160, 88, 0.5);
}

:deep(.n-avatar) {
  font-size: 14px;
}

:deep(.n-card-header) {
  padding: 8px 12px;
}

:deep(.n-card-footer) {
  padding: 8px 12px;
}

:deep(.n-card__content) {
  padding: 0 8px 8px;
}
</style>
