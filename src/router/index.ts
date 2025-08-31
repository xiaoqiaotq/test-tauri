import type { RouteRecordRaw } from 'vue-router'

import { createRouter, createWebHashHistory } from 'vue-router'

import Monitor from '../pages/Monitor.vue'
import Home from '../pages/Home.vue'
const routes: Readonly<RouteRecordRaw[]> = [
  {
    path: '/monitor',
    component: Monitor,
  },
  {
    path: '/home',
    component: Home,
  },
  // {
  //   path: '/',
  //   component: App,
  // },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
