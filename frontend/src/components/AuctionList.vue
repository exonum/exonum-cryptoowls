<template>
  <ul class="list-group">
    <li class="list-group-item font-weight-bold">
      <div class="row">
        <div class="col-sm-3">Auction ID</div>
        <div class="col-sm-3">Owl</div>
        <div class="col-sm-3">Start price</div>
        <div class="col-sm-3">Close in</div>
      </div>
    </li>
    <li v-for="auction in auctions" class="list-group-item" :key="auction.id">
      <div class="row">
        <div class="col-sm-3">
          <code>
            <router-link :to="{ name: 'auction', params: { id: auction.id } }" class="break-word">{{ auction.id }}</router-link>
          </code>
        </div>
        <div class="col-sm-3">
          <code>
            <router-link :to="{ name: 'owl', params: { hash: auction.auction.owl_id } }" class="break-word">{{ auction.auction.owl_id }}</router-link>
          </code>
        </div>
        <div class="col-sm-3">{{ auction.auction.start_price }}</div>
        <div v-if="auction.closed" class="col-sm-3">Closed</div>
        <div v-else class="col-sm-3">
          <countdown :from="$moment.toTimestamp(auction.started_at) + parseInt(auction.auction.duration)" :timeout="auction.auction.duration" :text="'Closed'"/>
        </div>
      </div>
    </li>
  </ul>
</template>

<script>
  import Countdown from '../components/Countdown.vue'

  module.exports = {
    components: {
      Countdown
    },
    name: 'auction-list',
    props: ['auctions']
  }
</script>
