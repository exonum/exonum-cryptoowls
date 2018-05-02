<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Auction</h1>
        </div>
      </div>
    </div>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import { mapState } from 'vuex'
  import Spinner from '../components/Spinner.vue'

  module.exports = {
    components: {
      Spinner
    },
    props: ['id'],
    data() {
      return {
        auction: {},
        isSpinnerVisible: false
      }
    },
    computed: mapState({
      keyPair: state => state.keyPair
    }),
    methods: {
      async loadAuction() {
        this.isSpinnerVisible = true

        try {
          await this.$blockchain.getAuction(this.id)
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async makeBid() {
        this.isSpinnerVisible = true

        try {
          await this.$blockchain.makeBid(this.keyPair, this.id, this.price)
          this.isSpinnerVisible = false
          this.$notify('success', 'Transaction accepted')
          this.loadUser()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async closeAuction() {
        this.isSpinnerVisible = true

        try {
          await this.$blockchain.closeAuction(this.keyPair, this.id)
          this.isSpinnerVisible = false
          this.$notify('success', 'Transaction accepted')
          this.loadUser()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      }
    },
    mounted() {
      this.$nextTick(function() {
        this.loadAuction()
      })
    }
  }
</script>
