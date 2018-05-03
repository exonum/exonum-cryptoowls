<template>
  <ul class="list-group">
    <li class="list-group-item font-weight-bold">
      <div class="row">
        <div class="col-sm-2">Auction ID</div>
        <div class="col-sm-2">Owl</div>
        <div class="col-sm-2">Started at</div>
        <div class="col-sm-2">Start price</div>
        <div class="col-sm-2">Duration</div>
        <div class="col-sm-2">Closed</div>
      </div>
    </li>
    <li v-for="auction in auctions" class="list-group-item" :key="auction.id">
      <div class="row">
        <div class="col-sm-2">
          <code>
            <router-link :to="{ name: 'auction', params: { id: auction.id } }" class="break-word">{{ auction.id }}</router-link>
          </code>
        </div>
        <div class="col-sm-2">
          <code>
            <router-link :to="{ name: 'owl', params: { hash: auction.auction.owl_id } }" class="break-word">{{ auction.auction.owl_id }}</router-link>
          </code>
        </div>
        <div class="col-sm-2">{{ $moment.getDate(auction.started_at) }}</div>
        <div class="col-sm-2">{{ auction.auction.start_price }}</div>
        <div class="col-sm-2">{{ auction.auction.duration }}</div>
        <div class="col-sm-2">{{ auction.closed }}</div>
      </div>
    </li>
  </ul>
</template>

<script>
  module.exports = {
    name: 'auction-list',
    props: ['auctions']
  }
</script>
