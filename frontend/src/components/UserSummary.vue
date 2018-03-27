<template>
  <ul class="list-group">
    <li class="list-group-item">
      <div class="row">
        <div class="col-sm-3"><strong>Имя:</strong></div>
        <div class="col-sm-9">{{ user.name }}</div>
      </div>
    </li>
    <li class="list-group-item">
      <div class="row">
        <div class="col-sm-3"><strong>Публичный ключ:</strong></div>
        <div class="col-sm-9">
          <code>{{ user.public_key }}</code>
        </div>
      </div>
    </li>
    <li class="list-group-item">
      <div class="row">
        <div class="col-sm-3"><strong>На счету:</strong></div>
        <div class="col-sm-9">{{ user.balance }}</div>
      </div>
    </li>
    <li class="list-group-item">
      <div class="row">
        <div class="col-sm-3"><strong>Последнее пополнение:</strong></div>
        <div class="col-sm-9">{{ user.last_fillup }}</div>
      </div>
    </li>
  </ul>
</template>

<script>
  module.exports = {
    props: ['user']
  }
</script>
