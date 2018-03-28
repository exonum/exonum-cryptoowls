import svg from 'svg.js'

export default {
  install(Vue) {
    Vue.prototype.$svg = svg
  }
}
