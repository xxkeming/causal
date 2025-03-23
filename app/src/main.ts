import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import './styles/main.css'
import 'highlight.js/styles/github.css'; // 导入highlight.js样式

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount('#app')
