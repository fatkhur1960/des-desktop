import Vue from 'vue'
import Router from 'vue-router'

import Login from '@/screen/Login'
import Item from '@/screen/Item'
import Sales from '@/screen/Sales'
import Predict from '@/screen/Predict'
import Admin from '@/screen/Admin'
import Home from '@/screen/Home'

Vue.use(Router)

let router = new Router({
  scrollBehavior(_to, _from, _savedPosition) {
    return { x: 0, y: 0 }
  },
  routes: [
    {
      path: '/',
      name: 'login',
      component: Login,
      meta: {
        sidebar: false,
      },
    },
    {
      path: '/home',
      name: 'home',
      component: Home,
      meta: {
        sidebar: true,
      },
    },
    {
      path: '/item',
      name: 'item',
      component: Item,
      meta: {
        sidebar: true,
      },
    },
    {
      path: '/sales',
      name: 'sales',
      component: Sales,
      meta: {
        sidebar: true,
      },
    },
    {
      path: '/predict',
      name: 'predict',
      component: Predict,
      meta: {
        sidebar: true,
      },
    },
    {
      path: '/admin',
      name: 'admin',
      component: Admin,
      meta: {
        sidebar: true,
      },
    },
  ],
})

export default router
