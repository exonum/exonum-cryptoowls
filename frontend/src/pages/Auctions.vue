<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>All auctions</h1>

          <ul class="list-group mt-3">
            <li class="list-group-item font-weight-bold">
              <div class="row">
                <div class="col-sm-3">Auction</div>
                <div class="col-sm-3">Owl</div>
                <div class="col-sm-3">Start price</div>
                <div class="col-sm-3">Duration</div>
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
                    <router-link :to="{ name: 'owl', params: { hash: auction.owl_id } }" class="break-word">{{ auction.owl_id }}</router-link>
                  </code>
                </div>
                <div class="col-sm-3">{{ auction.start_price }}</div>
                <div class="col-sm-3">{{ auction.duration }}</div>
              </div>
            </li>
          </ul>
        </div>
      </div>
    </div>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import Spinner from '../components/Spinner.vue'

  module.exports = {
    components: {
      Spinner
    },
    data() {
      return {
        auctions: [],
        isSpinnerVisible: false
      }
    },
    methods: {
      async loadAuctions() {
        this.isSpinnerVisible = true

        try {
          this.auctions = await this.$blockchain.getAuctions()
          this.isSpinnerVisible = false
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      }
    },
    mounted() {
      this.$nextTick(function() {
        this.loadAuctions()
      })
    }
  }
</script>
