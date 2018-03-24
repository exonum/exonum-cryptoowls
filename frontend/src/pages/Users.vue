<template>
  <div>
    <h1>Users</h1>

    <ul>
      <li v-for="user in users">
        <router-link :to="{ name: 'user', params: { publicKey: user.public_key } }">{{ user.name }}</router-link>
      </li>
    </ul>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  const Spinner = require('../components/Spinner.vue')

  module.exports = {
    components: {
      Spinner
    },
    data: function() {
      return {
        users: Array
      }
    },
    methods: {
      loadUsers: function() {
        const self = this

        this.isSpinnerVisible = true

        this.$blockchain.getUsers().then(users => {
          self.users = users
          self.isSpinnerVisible = false
        }).catch(function(error) {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      }
    },
    mounted: function() {
      this.$nextTick(function() {
        this.loadUsers()
      })
    }
  }
</script>
