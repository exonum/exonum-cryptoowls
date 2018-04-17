import moment from 'moment'
import bigInt from 'big-integer'

export default {
  install(Vue) {
    Vue.prototype.$moment = {
      toTimestamp(systemTime) {
        return bigInt(systemTime.secs).multiply(1000).plus(bigInt(systemTime.nanos).over(1000000)).valueOf()
      },

      getDate(systemTime) {
        return moment(this.toTimestamp(systemTime)).format('DD.MM.YYYY, HH:mm:ss')
      },

      diff(a, b) {
        return moment(a).diff(b, 'seconds')
      }
    }
  }
}
