<template>
  <div>
    <div class="container">
      <div class="row">
        <div class="col-sm-12">
          <h1>Личный кабинет</h1>

          <h2>Резюме</h2>
          <user-summary v-bind:user="user"></user-summary>

          <button class="btn btn-primary" @click.prevent="issue">Пополнить счёт</button>

          <h2>Совы</h2>
          <owl-list v-bind:owls="owls"></owl-list>

        </div>
      </div>
    </div>

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
        this.$storage.get().then(keyPair => {
          self.isSpinnerVisible = true
          self.$blockchain.getUser(keyPair.publicKey).then(data => {
            self.user = data
            self.$blockchain.getUserOwls(keyPair.publicKey).then(data => {
              self.owls = data
              self.isSpinnerVisible = false
            }).catch(error => {
              self.$notify('error', error.toString())
              self.isSpinnerVisible = false
            })
          }).catch(error => {
            self.$notify('error', error.toString())
            self.isSpinnerVisible = false
          })
        }).catch(error => {
          self.$notify('error', error.toString())
          self.logout()
        })
      },

      issue: function() {
        const self = this
        this.$storage.get().then(keyPair => {
          self.isSpinnerVisible = true
          self.$blockchain.issue(keyPair).then(data => {
            self.$notify('success', 'Счёт пополнен')
            self.isSpinnerVisible = false
          }).catch(error => {
            self.$notify('error', error.toString())
            self.isSpinnerVisible = false
          })
        }).catch(error => {
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
