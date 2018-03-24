<template>
  <div>
    <h1>Dashboard</h1>

    <h2>Summary</h2>
    <user-summary v-bind:user="user"></user-summary>

    <h2>Owls</h2>
    <owl-list v-bind:owls="owls"></owl-list>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  const Spinner = require('../components/Spinner.vue')
  const UserSummary = require('../components/UserSummary.vue')
  const OwlList = require('../components/OwlList.vue')

  module.exports = {
    components: {
      Spinner,
      UserSummary,
      OwlList
    },
    data: function() {
      return {
        user: Object,
        owls: Array
      }
    },
    methods: {
      loadUser: function() {
        const self = this
        this.$storage.get().then(function(keyPair) {
          self.isSpinnerVisible = true
          self.$blockchain.getUser(keyPair.publicKey).then(data => {
            self.user = data
            self.$blockchain.getUserOwls(keyPair.publicKey).then(data => {
              self.owls = data
              self.isSpinnerVisible = false
            }).catch(function(error) {
              self.$notify('error', error.toString())
              self.isSpinnerVisible = false
            })
          }).catch(function(error) {
            self.$notify('error', error.toString())
            self.isSpinnerVisible = false
          })
        }).catch(function(error) {
          self.$notify('error', error.toString())
          self.logout()
        })
      },

      logout: function() {
        this.$storage.remove()
        this.$router.push({name: 'auth'})
      }
    },
    mounted: function() {
      this.$nextTick(function() {
        this.loadUser()
      })
    }
  }
</script>
