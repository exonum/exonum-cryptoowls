<template>
  <div class="row">
    <div class="col-sm-6 col-md-4 col-lg-3" v-for="owl in owls" :key="$blockchain.getOwlHash(owl.owl)">
      <div class="card mt-3">
        <router-link :to="{ name: 'owl', params: { hash: $blockchain.getOwlHash(owl.owl) } }" class="break-word">
          <owl-icon v-bind:dna="owl.owl.dna" class="card-img-top"/>
        </router-link>
        <div class="card-body">
          <h5 class="card-title">
            <router-link :to="{ name: 'owl', params: { hash: $blockchain.getOwlHash(owl.owl) } }" class="break-word">{{ owl.owl.name }}</router-link>
          </h5>
          <p class="card-text">DNA: <code>{{ owl.owl.dna }}</code></p>
          <p class="card-text">Owner: <code><router-link :to="{ name: 'user', params: { publicKey: owl.owner } }" class="break-word">{{ owl.owner }}</router-link></code></p>
        </div>
        <div v-if="owl.last_breeding" class="card-footer">
          <div class="text-muted">Last breeding was on {{ $moment.getDate(owl.last_breeding) }}</div>
          <div v-if="owl.owner === keyPair.publicKey" class="text-muted mt-2">Ready for breeding: <countdown v-bind:date="owl.last_breeding"/></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
  import { mapState } from 'vuex'
  import OwlIcon from './OwlIcon.vue'
  import Countdown from './Countdown.vue'

  module.exports = {
    name: 'owl-list',
    components: {
      OwlIcon,
      Countdown
    },
    props: ['owls'],
    computed: mapState({
      keyPair: state => state.keyPair
    })
  }
</script>
