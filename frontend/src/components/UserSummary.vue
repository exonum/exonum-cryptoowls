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
    <li class="list-group-item">
      <div class="row">
        <div class="col-sm-3"><strong>Reserved balance:</strong></div>
        <div class="col-sm-9">{{ user.reserved }}</div>
      </div>
    </li>
    <li v-if="user.last_fillup" class="list-group-item">
      <div class="row">
        <div class="col-sm-3"><strong>Last issue:</strong></div>
        <div class="col-sm-9">{{ $moment.getDate(user.last_fillup) }}</div>
      </div>
    </li>
    <li v-if="user.public_key === keyPair.publicKey && user.last_fillup" class="list-group-item">
      <div class="row">
        <div class="col-sm-3"><strong>Available to issue:</strong></div>
        <div class="col-sm-9">
          <countdown :from="$moment.toTimestamp(user.last_fillup)" :timeout="60" :text="'right now!'"/>
        </div>
      </div>
    </li>
  </ul>
</template>

<script>
  import { mapState } from 'vuex'
  import Countdown from '../components/Countdown.vue'

  module.exports = {
    components: {
      Countdown
    },
    props: ['user'],
    computed: mapState({
      keyPair: state => state.keyPair
    })
  }
</script>
