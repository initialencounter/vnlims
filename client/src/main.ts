import 'element-plus/dist/index.css'
import ElementPlus from 'element-plus'
import { createApp } from 'vue'
// @ts-ignore
import App from './App.vue'
import router from './router'
import 'element-plus/theme-chalk/dark/css-vars.css'
import { House, Document, Setting, Fold, Expand } from '@element-plus/icons-vue'

const app = createApp(App)

app.use(ElementPlus)
app.use(router)

app.component('House', House)
app.component('Document', Document)
app.component('Setting', Setting)
app.component('Fold', Fold)
app.component('Expand', Expand)

app.mount('#app')