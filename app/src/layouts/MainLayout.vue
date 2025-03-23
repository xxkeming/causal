<template>
  <n-layout position="absolute" class="main-layout">
    <n-layout has-sider position="absolute" style="top: 0; bottom: 0;">
      <n-layout-sider
        :width="48"
        :content-style="siderStyle"
        class="main-sider"
      >
        <sidebar />
      </n-layout-sider>
      
      <n-layout class="main-content">
        <router-view />
      </n-layout>
    </n-layout>
  </n-layout>
</template>

<script setup lang="ts">
import { NLayout, NLayoutSider } from 'naive-ui'
import Sidebar from '../components/Sidebar.vue'

const siderStyle = {
  padding: 0
}
</script>

<style scoped>
.main-layout {
  height: 100%;
}

.main-sider {
  z-index: 9;
}

.main-content {
  padding: 0;
  height: 100%;
}

:deep(.n-layout-scroll-container) {
  overflow: hidden;
}
</style>
