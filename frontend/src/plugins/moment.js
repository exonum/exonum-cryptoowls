import moment from 'moment'

export default {
  install(Vue) {
    Vue.prototype.$moment = systemTime => moment(parseInt(systemTime.secs) * 1000).format('DD.MM.YYYY, HH:mm:ss')
  }
}
