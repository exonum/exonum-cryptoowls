<template>
  <nav class="navbar navbar-expand-lg navbar-light bg-light">
    <div class="container">
      <router-link :to="{ name: 'auth' }" class="navbar-brand">
        <img src="images/cryptoowl.png" width="36" height="36" class="d-inline-block align-middle" alt="">
        Crypto Owls
      </router-link>
      <div class="collapse navbar-collapse">
        <ul class="navbar-nav">
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
    methods: {
      logout: function() {
        this.$storage.remove()
        this.$router.push({name: 'home'})
      }
    }
  }
</script>
