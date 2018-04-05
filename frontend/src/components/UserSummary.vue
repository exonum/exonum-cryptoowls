<template>
  <ul class="list-group">
    <li class="list-group-item">
      <div class="row">
        <div class="col-sm-3"><strong>Name:</strong></div>
        <div class="col-sm-9">{{ user.name }}</div>
      </div>
    </li>
    <li class="list-group-item">
      <div class="row">
        <div class="col-sm-3"><strong>Public key:</strong></div>
        <div class="col-sm-9">
          <code>{{ user.public_key }}</code>
        </div>
      </div>
    </li>
    <li class="list-group-item">
      <div class="row">
        <div class="col-sm-3"><strong>Balance:</strong></div>
        <div class="col-sm-9">{{ user.balance }}</div>
      </div>
    </li>
    <li v-if="user.last_fillup" class="list-group-item">
      <div class="row">
        <div class="col-sm-3"><strong>Last issue:</strong></div>
        <div class="col-sm-9">{{ $moment(user.last_fillup) }}</div>
      </div>
    </li>
  </ul>
</template>

<script>
  module.exports = {
    props: ['user']
  }
</script>
