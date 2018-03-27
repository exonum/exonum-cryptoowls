<template>
  <div>
    <p>Публичный ключ: {{ user.public_key }}</p>
    <p>Имя: {{ user.name }}</p>
    <p>На счету: {{ user.balance }}</p>
    <p>Последнее пополнение: {{ user.last_fillup }}</p>
  </div>
</template>

<script>
  module.exports = {
    props: ['user']
  }
</script>
