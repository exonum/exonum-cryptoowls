<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Личный кабинет</h1>

          <h2 class="mt-5">Мой профиль</h2>
          <user-summary v-bind:user="user" class="mt-3"/>

          <button class="btn btn-primary mt-3" @click.prevent="issue">Пополнить счёт</button>

          <h2 class="mt-5">Мои совы</h2>
          <owl-list v-bind:owls="owls" class="mt-3"/>

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
        user: [],
        owls: [],
        isSpinnerVisible: false
      }
    },
    methods: {
      loadUser: function() {
        const self = this
        const keyPair = this.$store.state.keyPair

        if (keyPair === null) {
          this.$store.commit('logout')
          this.$router.push({name: 'auth'})
          return
        }

        this.isSpinnerVisible = true

        this.$blockchain.getUser(keyPair.publicKey).then(data => {
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
      },

      issue: function() {
        const self = this
        const keyPair = this.$store.state.keyPair

        this.isSpinnerVisible = true

        this.$blockchain.issue(keyPair).then(data => {
          self.$notify('success', 'Счёт пополнен')
          self.isSpinnerVisible = false
        }).catch(error => {
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
