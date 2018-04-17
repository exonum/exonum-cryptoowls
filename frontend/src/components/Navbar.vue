<template>
  <nav class="navbar navbar-expand-lg navbar-light bg-light">
    <div class="container">
      <router-link :to="{ name: isAuthorized ? 'dashboard' : 'auth' }" class="navbar-brand">
        <img src="images/cryptoowl.png" width="36" height="36" class="align-middle mr-2" alt="">
        Cryptoowls
      </router-link>
      <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbar" aria-controls="navbar" aria-expanded="false" aria-label="Toggle navigation">
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="collapse navbar-collapse" id="navbar">
        <ul class="navbar-nav">
          <li class="nav-item">
            <router-link :to="{ name: 'users' }" class="nav-link">Users</router-link>
          </li>
          <li class="nav-item">
            <router-link :to="{ name: 'owls' }" class="nav-link">Owls</router-link>
          </li>
          <li class="nav-item">
            <router-link :to="{ name: 'blockchain' }" class="nav-link">Blockchain</router-link>
          </li>
        </ul>
        <ul v-if="isAuthorized" class="navbar-nav ml-auto">
          <li class="nav-item">
            <a href="#" class="nav-link" @click="logout">Log Out</a>
          </li>
        </ul>
      </div>
    </div>
  </nav>
</template>

<script>
  module.exports = {
    name: 'navbar',
    computed: {
      isAuthorized() {
        return this.$store.state.keyPair !== null
      }
    },
    methods: {
      logout() {
        this.$store.commit('logout')
        this.$router.push({name: 'auth'})
      }
    }
  }
</script>
