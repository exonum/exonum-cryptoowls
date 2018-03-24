<template>
  <div>
    <h1>User {{ publicKey }}</h1>

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
    props: ['publicKey'],
    data: function() {
      return {
        user: Object,
        owls: Array
      }
    },
    methods: {
      loadUser: function() {
        const self = this
        this.isSpinnerVisible = true
        this.$blockchain.getUser(publicKey).then(data => {
          self.user = data
          self.$blockchain.getUserOwls(publicKey).then(data => {
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
      }
    },
    mounted: function() {
      this.$nextTick(function() {
        this.loadUser()
      })
    }
  }
</script>
