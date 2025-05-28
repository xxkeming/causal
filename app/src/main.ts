import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import './styles/main.css'
import 'highlight.js/styles/github.css'; // 导入highlight.js样式

// document.addEventListener('contextmenu', (event) => {
//   // 获取当前选中的文本
//   const selectedText = window.getSelection()?.toString().trim();

//   // 如果未选中任何文本且点击目标是按钮类元素
//   if (!selectedText) {
//     event.preventDefault(); // 阻止默认右键菜单
//     console.log('未选中文本，右键菜单已屏蔽');
//   }
// });

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount('#app')
