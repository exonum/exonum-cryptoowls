<template>
  <div>
    <p>Public key: {{ user.public_key }}</p>
    <p>Name: {{ user.name }}</p>
    <p>Balance: {{ user.balance }}</p>
    <p>Last fillup: {{ user.last_fillup }}</p>
  </div>
</template>

<script>
  module.exports = {
    props: ['user']
  }
</script>
