import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useGlobalStore = defineStore('global', () => {
  // 全局加载状态
  const loading = ref(false);

  // 设置加载状态
  function setLoadingState(testing: boolean) {
    loading.value = testing;
  }

  return {
    isLoading: loading,
    setLoadingState
  };
});
