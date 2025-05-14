<template>
  <div class="about-page">
    <!-- 添加可滚动容器包裹所有内容 -->
    <div class="scrollable-container">
      <div class="content-section">
        <div class="app-info-section">
          <div class="app-logo">
            <!-- 替换占位符为实际logo.png -->
            <img src="/logo.png" alt="AI Test Logo" class="logo-image" />
          </div>
          <div class="app-details">
            <h2>{{ appN }}</h2>
            <p class="version">版本 {{ appV }}</p>
            <p class="build-info">构建日期: {{ appD }}</p>
            <div class="action-buttons">
              <n-button @click="checkForUpdates" :loading="globalStore.isLoading" type="primary" size="small">
                检查更新
              </n-button>
            </div>
          </div>
        </div>

        <!-- 添加更新进度条，只在下载更新时显示 -->
        <div v-if="downloading" class="update-progress-container">
          <div class="update-progress-header">
            <div class="update-icon">
              <n-icon size="20"><DownloadOutline /></n-icon>
            </div>
            <div class="update-text">
              <p class="update-status">正在下载更新</p>
              <p class="update-percentage">{{ downloadProgress }}%</p>
            </div>
          </div>
          <n-progress 
            :percentage="downloadProgress" 
            type="line" 
            :indicator-placement="'inside'" 
            :processing="downloadProgress < 100"
            :height="8"
            color="#36ad6a"
          />
          <div class="update-footer">
            <p class="update-size">{{ formatFileSize(downloadCurrent) }} / {{ formatFileSize(downloadTotal) }}</p>
            <p class="update-note">下载完成后应用将自动重启</p>
          </div>
        </div>

        <n-divider />

        <h3>系统信息</h3>
        <n-descriptions bordered :column="2" class="system-info">
          <n-descriptions-item label="操作系统">
            {{ systemInfo.os }}
          </n-descriptions-item>
          <n-descriptions-item label="设备">
            {{ systemInfo.device }}
          </n-descriptions-item>
        </n-descriptions>

        <n-divider />

        <h3>技术栈</h3>
        <div class="tech-stack">
          <n-tag v-for="tech in techStack" :key="tech.name" :type="tech.type" class="tech-tag">
            {{ tech.name }}
          </n-tag>
        </div>

        <n-divider />

        <h3>开源许可</h3>
        <p>本应用基于以下开源许可证：</p>
        <n-list bordered>
          <n-list-item v-for="license in licenses" :key="license.name">
            <n-thing :title="license.name" :description="license.type">
              {{ license.description }}
            </n-thing>
          </n-list-item>
        </n-list>

        <n-divider />

        <div class="copyright">
          <p>© {{ currentYear }} {{ appN }} 团队. 保留所有权利。</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { NButton, NDivider, NDescriptions, NDescriptionsItem, NTag, NList, NListItem, NThing, useMessage, useDialog, NProgress, NIcon } from 'naive-ui';
import { appName, appVersion, appDate } from '../../services/api';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { DownloadOutline } from '@vicons/ionicons5';
import { useGlobalStore } from '../../stores/globalStore';

const globalStore = useGlobalStore();
const message = useMessage();
const dialog = useDialog();

// 更新相关状态
const downloading = ref(false);
const downloadProgress = ref(0);
const downloadTotal = ref(0);
const downloadCurrent = ref(0);

// 应用信息
const appN = ref<string>('');
const appV = ref<string>('');
const appD = ref<string>('');

onMounted(async () => {
  appN.value = await appName();
  appV.value = await appVersion();
  appD.value = await appDate();
})

// 当前年份
const currentYear = computed(() => new Date().getFullYear());

// 系统信息
const systemInfo = ref({
  os: detectOS(),
  browser: detectBrowser(),
  device: detectDevice()
});

// 技术栈信息 - 修复类型问题
const techStack = [
  { name: 'Tauri', type: 'primary' as const },
  { name: 'Rust', type: 'info' as const },
  { name: 'Vue 3', type: 'success' as const },
  { name: 'Naive UI', type: 'success' as const },
  { name: 'Vite', type: 'warning' as const },
  
];

// 许可证信息
const licenses = [
  { 
    name: 'Tauri', 
    type: 'MIT/Apache-2.0 许可证', 
    description: '构建更小、更快、更安全的桌面应用程序' 
  },
  { 
    name: 'Rust', 
    type: 'MIT/Apache-2.0 许可证', 
    description: '安全、并发和快速的系统编程语言' 
  },
  { 
    name: 'Vue.js', 
    type: 'MIT 许可证', 
    description: '渐进式 JavaScript 框架' 
  },
  { 
    name: 'Naive UI', 
    type: 'MIT 许可证', 
    description: '一个 Vue 3 组件库' 
  },
  { 
    name: 'Vite', 
    type: 'MIT 许可证', 
    description: '下一代前端构建工具' 
  }
];

// 检测操作系统
function detectOS() {
  const platform = window.navigator.platform.toLowerCase();
  if (platform.includes('win')) return 'Windows';
  if (platform.includes('mac')) return 'macOS';
  if (platform.includes('linux')) return 'Linux';
  if (platform.includes('android')) return 'Android';
  if (platform.includes('ios') || platform.includes('iphone') || platform.includes('ipad')) return 'iOS';
  return '未知操作系统';
}

// 检测浏览器
function detectBrowser() {
  const agent = window.navigator.userAgent.toLowerCase();
  if (agent.includes('edg')) return 'Microsoft Edge';
  if (agent.includes('chrome')) return 'Google Chrome';
  if (agent.includes('firefox')) return 'Mozilla Firefox';
  if (agent.includes('safari') && !agent.includes('chrome')) return 'Safari';
  if (agent.includes('opera') || agent.includes('opr')) return 'Opera';
  return '未知浏览器';
}

// 检测设备类型
function detectDevice() {
  const agent = window.navigator.userAgent.toLowerCase();
  if (agent.includes('mobile')) return '移动设备';
  if (agent.includes('tablet')) return '平板设备';
  return '桌面设备';
}

// 格式化文件大小
function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

// 检查更新 - 优化流程
async function checkForUpdates() {
  globalStore.setLoadingState(true);

  try {
    const update = await check();
    if (update) {
      console.log(`发现新版本 ${update.version} 发布于 ${update.date}`);
      
      // 保存更新信息，但不立即开始更新
      const shouldUpdate = await new Promise<boolean>((resolve) => {
        dialog.warning({
          title: '发现新版本',
          content: `发现新版本 ${update.version}
            
更新内容：
${update.body || '- 功能改进和bug修复'}`,
          positiveText: '立即更新',
          negativeText: '暂不更新',
          style: {
            position: 'relative',
            marginTop: '20vh'
          },
          onPositiveClick: () => {
            resolve(true);
          },
          onNegativeClick: () => {
            resolve(false);
          },
          onMaskClick: () => {
            resolve(false);
          },
          onEsc: () => {
            resolve(false);
          },
          onClose: () => {
            resolve(false);
          }
        });
      });
      
      // 对话框关闭后，如果用户选择了更新，则开始下载
      if (shouldUpdate) {
        // 确保对话框完全关闭后再开始更新
        setTimeout(() => {
          downloadAndInstallUpdate(update);
        }, 100);
      } else {
        globalStore.setLoadingState(false);
      }
    } else {
      message.success('您已经在使用最新版本！');
      globalStore.setLoadingState(false);
    }
  } catch (error) {
    console.error('Error checking for updates:', error);
    message.error('检查更新失败，请稍后再试。');
    globalStore.setLoadingState(false);
  }
}

// 下载并安装更新
async function downloadAndInstallUpdate(update: any) {
  downloading.value = true;
  downloadProgress.value = 0;
  downloadCurrent.value = 0;
  
  try {
    let contentLength = 0;
    
    await update.downloadAndInstall((event: any) => {
      switch (event.event) {
        case 'Started':
          downloadTotal.value = event.data.contentLength || 0;
          contentLength = event.data.contentLength || 0;
          console.log(`开始下载 ${downloadTotal.value} 字节`);
          break;
          
        case 'Progress':
          downloadCurrent.value += event.data.chunkLength;
          // 计算进度百分比
          if (contentLength > 0) {
            downloadProgress.value = Math.min(Math.round((downloadCurrent.value / downloadTotal.value) * 100), 100);
          }
          // console.log(`已下载 ${downloadCurrent.value} / ${downloadTotal.value} 字节`);
          break;
          
        case 'Finished':
          downloadProgress.value = 100;
          console.log('下载完成');
          break;
      }
    });

    console.log('更新已安装');
    message.success('更新已成功安装，应用将重启');
    
    // 设置短暂延迟后重启应用
    setTimeout(async () => {
      await relaunch();
    }, 1500);
  } catch (error) {
    console.error('Error downloading update:', error);
    message.error('下载更新失败，请稍后再试:' + error);
    downloading.value = false;
  } finally {
    downloading.value = false;
    globalStore.setLoadingState(false);
  }
}
</script>

<style scoped>
.about-page {
  width: 100%;
  height: 100%;
  position: relative;
}

/* 新增可滚动容器样式 */
.scrollable-container {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  
  /* 隐藏滚动条 */
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE and Edge */
}

/* Webkit浏览器中隐藏滚动条 */
.scrollable-container::-webkit-scrollbar {
  display: none;
}

.content-section {
  max-width: 100%;
  margin: 0 auto;
  padding: 20px;
}

.header-section {
  margin-bottom: 20px;
}

.title {
  font-size: 24px;
  font-weight: 600;
  margin-bottom: 0;
}

.app-info-section {
  display: flex;
  align-items: center;
  margin-bottom: 30px;
}

.app-logo {
  flex: 0 0 100px;
  margin-right: 20px;
}

/* 添加 Logo 图片样式 */
.logo-image {
  width: 80px;
  height: 80px;
  object-fit: contain;
}

/* 移除原有的占位符样式 */
/* 
.logo-placeholder {
  width: 80px;
  height: 80px;
  background-color: var(--primary-color);
  color: white;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 28px;
  font-weight: bold;
}
*/

.app-details {
  flex: 1;
}

.app-details h2 {
  margin: 0 0 10px 0;
  font-size: 22px;
}

.version {
  margin: 0;
  font-size: 16px;
  color: var(--text-color-secondary);
}

.build-info {
  margin: 5px 0 15px;
  color: var(--text-color-secondary);
  font-size: 14px;
}

.action-buttons {
  display: flex;
  gap: 10px;
  margin-top: 15px;
}

.system-info {
  margin-top: 15px;
  margin-bottom: 30px;
}

.tech-stack {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin: 15px 0 30px;
}

.tech-tag {
  font-size: 14px;
}

.copyright {
  margin-top: 30px;
  text-align: center;
  color: var(--text-color-secondary);
  font-size: 14px;
}

h3 {
  margin-top: 20px;
  margin-bottom: 15px;
  font-size: 18px;
  font-weight: 500;
}

/* 确保滚动容器的滚动条被完全隐藏 */
.scrollable-container {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  
  /* 增强滚动条隐藏 */
  scrollbar-width: none !important; /* Firefox */
  -ms-overflow-style: none !important; /* IE and Edge */
}

/* 增强 Webkit浏览器中隐藏滚动条 */
.scrollable-container::-webkit-scrollbar {
  display: none !important;
  width: 0 !important;
  height: 0 !important;
  background: transparent !important;
}

/* 确保内部容器也不显示滚动条 */
.scrollable-container * {
  scrollbar-width: none !important; /* Firefox */
  -ms-overflow-style: none !important; /* IE and Edge */
}

.scrollable-container *::-webkit-scrollbar {
  display: none !important;
  width: 0 !important;
  height: 0 !important;
}

/* 添加更新进度条样式 */
.update-progress-container {
  margin: 20px 0;
  padding: 16px;
  border-radius: 8px;
  background-color: var(--card-color, #f9f9f9);
  border: 1px solid var(--border-color, #eaeaea);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.update-progress-header {
  display: flex;
  align-items: center;
  margin-bottom: 10px;
}

.update-icon {
  margin-right: 10px;
}

.update-text {
  flex: 1;
}

.update-status {
  font-size: 14px;
  font-weight: 500;
  color: var(--primary-color);
}

.update-percentage {
  font-size: 14px;
  color: var(--text-color-secondary);
}

.update-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 10px;
}

.update-size {
  font-size: 13px;
  color: var(--text-color-secondary);
}

.update-note {
  font-size: 13px;
  color: var(--text-color-secondary);
}
</style>
