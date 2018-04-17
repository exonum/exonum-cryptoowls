import Vue from 'vue'
import App from './App.vue'
import router from './router'
import Notify from './plugins/notify'
import Blockchain from './plugins/blockchain'
import SvgJs from './plugins/svg'
import moment from './plugins/moment'
import store from './store'

Vue.use(Notify)
Vue.use(Blockchain)
Vue.use(SvgJs)
Vue.use(moment)

new Vue({
  el: '#app',
  router,
  store,
  render: createElement => createElement(App)
})
