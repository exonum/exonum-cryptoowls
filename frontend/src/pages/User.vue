<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>User</h1>
          <hr>

          <h2 class="mt-3">Profile</h2>
          <user-summary :user="user" class="mt-3"/>

          <h2 class="mt-5">Owls</h2>
          <owl-list :owls="owls"/>

          <h2 class="mt-5">Auctions</h2>
          <auction-list :auctions="auctions" class="mt-3"/>
        </div>
      </div>
    </div>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import Spinner from '../components/Spinner.vue'
  import UserSummary from '../components/UserSummary.vue'
  import OwlList from '../components/OwlList.vue'
  import AuctionList from '../components/AuctionList.vue'

  module.exports = {
    components: {
      Spinner,
      UserSummary,
      OwlList,
      AuctionList
    },
    props: ['publicKey'],
    data() {
      return {
        user: [],
        owls: [],
        auctions: [],
        isSpinnerVisible: false
      }
    },
    methods: {
      async loadUser() {
        this.isSpinnerVisible = true

        try {
          this.user = await this.$blockchain.getUser(this.publicKey)
          this.owls = await this.$blockchain.getUserOwls(this.publicKey)
          this.isSpinnerVisible = false
          this.loadAuctions()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async loadAuctions() {
        this.isSpinnerVisible = true

        try {
          this.auctions = await this.$blockchain.getUserAuctions(this.publicKey)
          this.isSpinnerVisible = false
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      }
    },
    mounted() {
      this.$nextTick(function() {
        this.loadUser()
      })
    }
  }
</script>
