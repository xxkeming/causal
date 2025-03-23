import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useGlobalStore = defineStore('global', () => {
  // 全局加载状态
  const isTesting = ref(false);

  // 设置测试状态
  function setTestingState(testing: boolean) {
    isTesting.value = testing;
  }

  return {
    isTesting,
    setTestingState
  };
});
