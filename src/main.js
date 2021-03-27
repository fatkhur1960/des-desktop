import Vue from 'vue'
import App from './App.vue'
import store from './store'

Vue.config.productionTip = false

import SuiVue from 'semantic-ui-vue'
import 'semantic-ui-css/semantic.min.css'
Vue.use(SuiVue)

import VuePageTransition from 'vue-page-transition'
Vue.use(VuePageTransition)

import Vuelidate from 'vuelidate'
Vue.use(Vuelidate)

import Notifications from 'vue-notification'
Vue.use(Notifications)

import vmodal from 'vue-js-modal'
Vue.use(vmodal, {
  dialog: true,
})

import moment from 'moment'
import 'moment/locale/id'
import VueMomentJS from 'vue-momentjs'
moment.locale('id')
Vue.use(VueMomentJS, moment)

import uiPlugins from './plugins/ui_plugin'
Vue.use(uiPlugins)

import apiPlugin from './plugins/api_plugin'
import * as api from './plugins/api'
Vue.use(apiPlugin, {
  item: new api.Item(),
  auth: new api.Auth(),
  user: new api.User(),
  sale: new api.Sale(),
})

import router from './router'

new Vue({
  router,
  store,
  render: (h) => h(App),
}).$mount('#app')
