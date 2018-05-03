<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>All auctions</h1>

          <auction-list v-bind:auctions="auctions" class="mt-3"/>
        </div>
      </div>
    </div>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import Spinner from '../components/Spinner.vue'
  import AuctionList from '../components/AuctionList.vue'

  module.exports = {
    components: {
      Spinner,
      AuctionList
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
