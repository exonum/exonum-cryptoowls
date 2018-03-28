import moment from 'moment'

export default {
  install(Vue) {
    Vue.prototype.$moment = moment
  }
}
