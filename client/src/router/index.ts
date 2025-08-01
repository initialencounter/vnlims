import { createRouter, createWebHashHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/search',
    name: 'home',
    component: () => import('../components/Search.vue')
  },
  {
    path: '/',
    alias: ['/searchTNotes'],
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
  },
  {
    path: '/searchPrincipal',
    name: 'searchPrincipal',
    component: () => import('../components/SearchPrincipal.vue')
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router