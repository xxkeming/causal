import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useGlobalStore = defineStore('global', () => {
  // 全局加载状态
  const loading = ref(false);
  
  // 语音录制对象
  const mediaRecorder = ref<MediaRecorder | null>(null);

  // 设置加载状态
  function setLoadingState(testing: boolean) {
    loading.value = testing;
  }
  
  // 设置媒体录制器
  function setMediaRecorder(recorder: MediaRecorder | null) {
    mediaRecorder.value = recorder;
  }
  
  // 获取媒体录制器
  function getMediaRecorder(): MediaRecorder | null {
    return mediaRecorder.value;
  }

  return {
    isLoading: loading,
    setLoadingState,
    setMediaRecorder,
    getMediaRecorder
  };
});
