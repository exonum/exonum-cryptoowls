<template>
  <div class="card-columns">
    <div class="card" v-for="owl in owls">
      <owl-icon v-bind:dna="owl.owl.dna" class="card-img-top"/>
      <div class="card-body">
        <h5 class="card-title">
          Кличка: <router-link :to="{ name: 'owl', params: { hash: $blockchain.owlHash(owl.owl) } }" class="break-word">{{ owl.owl.name }}</router-link>
        </h5>
        <p class="card-text">ДНК: <code>{{ owl.owl.dna }}</code></p>
        <p class="card-text">Хозяин: <code><router-link :to="{ name: 'user', params: { publicKey: owl.owner } }" class="break-word">{{ owl.owner }}</router-link></code></p>
      </div>
      <div class="card-footer">
        <small class="text-muted">Последнее разведение: {{ $moment(parseInt(owl.last_breeding.secs) * 1000).format('DD.MM.YYYY, HH:mm:ss') }}</small>
      </div>
    </div>
  </div>
</template>

<script>
  const OwlIcon = require('./OwlIcon.vue')

  module.exports = {
    name: 'owl-list',
    components: {
      OwlIcon
    },
    props: ['owls']
  }
</script>
