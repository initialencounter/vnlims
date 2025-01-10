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
    path: '/searchItemCName',
    name: 'searchItemCName',
    component: () => import('../components/SearchItemCName.vue')
  },
  {
    path: '/updateDatabase',
    name: 'updateDatabase',
    // @ts-ignore
    component: () => import('../components/UpdateDatabase.vue')
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router 