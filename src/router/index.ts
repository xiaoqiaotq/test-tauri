import type { RouteRecordRaw } from 'vue-router'

import { createRouter, createWebHashHistory } from 'vue-router'

import Monitor from '../pages/Monitor.vue'
import Home from '../pages/Home.vue'
import App from '../App.vue'
// import Preference from '../pages/preference/index.vue'

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
  //   path: '/preference',
  //   component: Preference,
  // },
  {
    path: '/',
    component: App,
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
