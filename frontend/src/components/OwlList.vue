<template>
  <ul>
    <li v-for="owl in owls">
      <router-link :to="{ name: 'owl', params: { dna: owl.dna } }">{{ owl.name }}</router-link>
    </li>
  </ul>
</template>

<script>
  module.exports = {
    name: 'owl-list',
    props: ['owls']
  }
</script>
