import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: () => import('../components/Search.vue')
  },
  {
    path: '/searchTNotes',
    name: 'searchTNotes',
    component: () => import('../components/SearchTNotes.vue')
  },
  {
    path: '/searchMNotes',
    name: 'searchMNotes',
    component: () => import('../components/SearchMNotes.vue')
  },
  {
    path: '/searchCName',
    name: 'searchCName',
    component: () => import('../components/SearchItemCName.vue')
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router 