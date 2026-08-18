import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { convertFileSrc } from '@tauri-apps/api/core'
import App from './App.vue'
import './styles/app.css'

// 让 muya 引擎把本地图片路径经 Tauri asset 协议转成可加载的 URL，
// 避免从 http/dev 源加载 `file://` 子资源被 WebView2 拦截而显示「图片加载失败」。
// 仅在 Tauri 运行时生效；浏览器环境下该钩子不存在，回退到 file://。
if (typeof window !== 'undefined') {
  ;(window as unknown as { __convertFileSrc?: (p: string) => string }).__convertFileSrc = convertFileSrc
}

createApp(App).use(createPinia()).mount('#app')