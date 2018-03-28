import Vue from 'vue'
import App from './App.vue'
import router from './router'
import Validate from './plugins/validate'
import Notify from './plugins/notify'
import Blockchain from './plugins/blockchain'
import SvgJs from './plugins/svg'
import store from './store'

Vue.use(Validate)
Vue.use(Notify)
Vue.use(Blockchain)
Vue.use(SvgJs)

new Vue({
  el: '#app',
  router,
  store,
  render: (createElement) => createElement(App)
})
