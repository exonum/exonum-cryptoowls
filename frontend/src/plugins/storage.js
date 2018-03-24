const SERVICE_NAME = 'cryptoowls'

export default {
  install(Vue) {
    Vue.prototype.$storage = {
      set: function(keyPair) {
        localStorage.setItem(SERVICE_NAME, JSON.stringify(keyPair))
      },
      get: function() {
        return new Promise(function(resolve, reject) {
          let keyPair = JSON.parse(localStorage.getItem(SERVICE_NAME))

          if (keyPair === null) {
            return reject(new Error('User not found in local storage'))
          }

          resolve(keyPair)
        })
      },
      remove: function() {
        localStorage.removeItem(SERVICE_NAME)
      }
    }
  }
}
