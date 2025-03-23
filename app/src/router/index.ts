import { createRouter, createWebHistory } from 'vue-router'
import MainLayout from '../layouts/MainLayout.vue'

const routes = [
  {
    path: '/',
    component: MainLayout,
    children: [
      {
        path: '',
        redirect: '/chat'
      },
      {
        path: 'profile',
        name: 'profile',
        component: () => import('../views/profile/index.vue')
      },
      {
        path: 'messages',
        name: 'messages',
        component: () => import('../views/messages/index.vue')
      },
      {
        path: 'agents',
        name: 'agents',
        component: () => import('../views/agents/index.vue')
      },
      {
        path: 'knowledge',
        name: 'knowledge',
        component: () => import('../views/knowledge/index.vue')
      },
      {
        path: 'tools',
        name: 'tools',
        component: () => import('../views/tools/index.vue')
      },
      {
        path: 'tools/edit',
        name: 'toolCreate',
        component: () => import('../views/tools/edit.vue'),
        props: (route: { query: { categoryId?: string } }) => ({ categoryId: route.query.categoryId }) // 添加类型注解
      },
      {
        path: 'tools/edit/:id',
        name: 'toolEdit',
        component: () => import('../views/tools/edit.vue'),
        props: true // 保持编辑时支持动态参数
      },
      {
        path: 'chat',
        name: 'chat',
        component: () => import('../views/chat/index.vue')
      },
      {
        path: 'settings',
        redirect: '/settings/about'  // 添加重定向
      },
      {
        path: 'settings/model',
        name: 'settingModel',
        component: () => import('../views/settings/model.vue')
      },
      {
        path: 'settings/about',
        name: 'settingAbout',
        component: () => import('../views/settings/about.vue')
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// // 添加全局导航守卫
// router.beforeEach((to, from, next) => {
//   const globalStore = useGlobalStore();
  
//   // 如果正在测试中，阻止导航并显示提示信息
//   if (globalStore.isTesting) {
//     // 使用 naive-ui 的 discrete API 创建消息提示
//     const { message } = createDiscreteApi(['message']);
//     message.warning('测试正在进行中，请等待测试完成后再进行操作');
    
//     // 阻止导航
//     next(false);
//     return;
//   }
  
//   // 正常情况下允许导航
//   next();
// });

export default router
