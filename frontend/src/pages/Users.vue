<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>All users</h1>

          <ul class="list-group mt-3">
            <li class="list-group-item font-weight-bold">
              <div class="row">
                <div class="col-sm-3">Name</div>
                <div class="col-sm-3">Public key</div>
                <div class="col-sm-3">Balance</div>
                <div class="col-sm-3">Last issue</div>
              </div>
            </li>
            <li v-for="user in users" class="list-group-item" :key="user.public_key">
              <div class="row">
                <div class="col-sm-3">
                  <router-link :to="{ name: 'user', params: { publicKey: user.public_key } }" class="break-word">{{ user.name }}</router-link>
                </div>
                <div class="col-sm-3"><code>{{ user.public_key }}</code></div>
                <div class="col-sm-3">{{ user.balance }}</div>
                <div class="col-sm-3">{{ $moment(user.last_fillup) }}</div>
              </div>
            </li>
          </ul>
        </div>
      </div>
    </div>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import Spinner from '../components/Spinner.vue'

  module.exports = {
    components: {
      Spinner
    },
    data: function() {
      return {
        users: [],
        isSpinnerVisible: false
      }
    },
    methods: {
      loadUsers: function() {
        const self = this

        this.isSpinnerVisible = true

        this.$blockchain.getUsers().then(users => {
          self.users = users
          self.isSpinnerVisible = false
        }).catch(error => {
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
