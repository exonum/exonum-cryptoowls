<template>
  <div class="card-columns">
    <div class="card" v-for="owl in owls">
      <owl v-bind:dna="owl.owl.dna" class="card-img-top"/>
      <div class="card-body">
        <h5 class="card-title">
          <router-link :to="{ name: 'owl', params: { dna: owl.owl.dna } }" class="break-word">{{ owl.owl.name }}</router-link>
        </h5>
        <p class="card-text">ДНК: <code>{{ owl.owl.dna }}</code></p>
        <p class="card-text">Хозяин: <code>
          <router-link :to="{ name: 'user', params: { publicKey: owl.owner } }" class="break-word">{{ owl.owner }}</router-link>
        </code></p>
      </div>
      <div class="card-footer">
        <small class="text-muted">Последнее разведение: {{ $moment(parseInt(owl.last_breeding.secs) * 1000).format('DD.MM.YYYY, HH:mm:ss') }}</small>
      </div>
    </div>
  </div>
</template>

<script>
  const Owl = require('./Owl.vue')

  module.exports = {
    name: 'owl-list',
    components: {
      Owl
    },
    props: ['owls']
  }
</script>
