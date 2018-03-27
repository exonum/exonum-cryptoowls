<template>
  <ul class="list-group">
    <li class="list-group-item font-weight-bold">
      <div class="row">
        <div class="col-sm-3">Кличка</div>
        <div class="col-sm-3">ДНК</div>
        <div class="col-sm-3">Хозяин</div>
        <div class="col-sm-3">Последнее разведение</div>
      </div>
    </li>
    <li v-for="owl in owls" class="list-group-item">
      <div class="row">
        <div class="col-sm-3">
          <router-link :to="{ name: 'owl', params: { dna: owl.owl.dna } }" class="break-word">{{ owl.owl.name }}</router-link>
        </div>
        <div class="col-sm-3">
          <code>{{ owl.owl.dna }}</code>
        </div>
        <div class="col-sm-3">
          <code>
            <router-link :to="{ name: 'user', params: { publicKey: owl.owner } }" class="break-word">{{ owl.owner }}</router-link>
          </code>
        </div>
        <div class="col-sm-3">{{ owl.last_breeding }}</div>
      </div>
    </li>
  </ul>
</template>

<script>
  module.exports = {
    name: 'owl-list',
    props: ['owls']
  }
</script>
