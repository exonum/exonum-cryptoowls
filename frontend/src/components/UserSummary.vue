<template>
  <div>
    <p>{{ user.public_key }}</p>
    <p>{{ user.name }}</p>
    <p>{{ user.balance }}</p>
    <p>{{ user.last_fillup }}</p>
  </div>
</template>

<script>
  module.exports = {
    props: ['user']
  }
</script>
