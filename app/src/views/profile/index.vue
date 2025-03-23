<template>
  <div class="profile-view">
    <n-card title="个人中心" size="large">
      <n-space vertical size="large">
        <div class="profile-header">
          <n-avatar
            round
            :size="80"
            :src="user.avatar || ''"
            class="profile-avatar"
          >
            <n-icon size="40"><PersonOutline /></n-icon>
          </n-avatar>
          <div class="profile-info">
            <h1 class="profile-name">{{ user.name }}</h1>
            <p class="profile-description">{{ user.description }}</p>
          </div>
        </div>

        <n-divider />
        
        <n-descriptions label-placement="left" bordered>
          <n-descriptions-item label="用户名">
            {{ user.name }}
          </n-descriptions-item>
          <n-descriptions-item label="邮箱">
            {{ user.email }}
          </n-descriptions-item>
          <n-descriptions-item label="角色">
            {{ user.role }}
          </n-descriptions-item>
          <n-descriptions-item label="注册时间">
            {{ user.joinDate }}
          </n-descriptions-item>
        </n-descriptions>
        
        <div class="profile-statistics">
          <n-card title="应用概况" size="small">
            <n-grid cols="3" x-gap="16">
              <n-grid-item>
                <n-statistic label="对话数">
                  <n-number-animation :from="0" :to="user.stats.sessions" />
                </n-statistic>
              </n-grid-item>
              <n-grid-item>
                <n-statistic label="智能体">
                  <n-number-animation :from="0" :to="user.stats.agents" />
                </n-statistic>
              </n-grid-item>
              <n-grid-item>
                <n-statistic label="知识库">
                  <n-number-animation :from="0" :to="user.stats.knowledgebases" />
                </n-statistic>
              </n-grid-item>
            </n-grid>
          </n-card>
        </div>
        
        <n-card title="关于我" size="small">
          <p>{{ user.about }}</p>
        </n-card>
      </n-space>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { 
  NCard, NAvatar, NSpace, NDivider, NDescriptions, 
  NDescriptionsItem, NIcon, NGrid, NGridItem, 
  NStatistic, NNumberAnimation 
} from 'naive-ui'
import { PersonOutline } from '@vicons/ionicons5'

// 模拟用户数据，实际应用中可以从API获取
const user = ref({
  name: '测试用户',
  email: 'user@example.com',
  description: 'AI应用开发者',
  role: '管理员',
  joinDate: '2023-10-01',
  avatar: '',
  about: '欢迎使用Causal Studio - 这是一个强大的工具，可以帮助您创建和管理智能体、知识库和工具。我们致力于提供最好的用户体验。',
  stats: {
    sessions: 12,
    agents: 3,
    knowledgebases: 5
  }
})
</script>

<style scoped>
.profile-view {
  padding: 16px;
  max-width: 900px;
  margin: 0 auto;
}

.profile-header {
  display: flex;
  align-items: center;
  gap: 24px;
  padding: 16px 0;
}

.profile-avatar {
  flex-shrink: 0;
}

.profile-info {
  flex-grow: 1;
}

.profile-name {
  margin: 0 0 8px 0;
  font-size: 24px;
  font-weight: 600;
}

.profile-description {
  margin: 0;
  font-size: 16px;
  color: var(--text-color-secondary);
}

.profile-statistics {
  margin-top: 16px;
}
</style>
