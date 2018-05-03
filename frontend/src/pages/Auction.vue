<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Auction</h1>

          <div class="row mt-5">
            <div class="col-sm-6">
              <h2>Owl</h2>
              <ul class="list-group mt-3">
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Name:</strong></div>
                    <div class="col-sm-9">{{ owl.name }}</div>
                  </div>
                </li>
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>DNA:</strong></div>
                    <div class="col-sm-9">
                      <code>{{ owl.dna }}</code>
                    </div>
                  </div>
                </li>
                <li v-if="owner" class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Owner:</strong></div>
                    <div class="col-sm-9">
                      <code>
                        <router-link :to="{ name: 'user', params: { publicKey: owner } }" class="break-word">{{ owner }}</router-link>
                      </code>
                    </div>
                  </div>
                </li>
                <li v-if="lastBreeding" class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Last breeding:</strong></div>
                    <div class="col-sm-9">{{ $moment.getDate(lastBreeding) }}</div>
                  </div>
                </li>
                <li v-if="owner === keyPair.publicKey && lastBreeding" class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Ready for breeding:</strong></div>
                    <div class="col-sm-9"><countdown v-bind:date="lastBreeding"/></div>
                  </div>
                </li>
              </ul>
            </div>
            <div class="col-sm-6">
              <owl-icon v-if="owl.dna" v-bind:dna="owl.dna"/>
            </div>
          </div>

          <div class="row mt-5">
            <div v-if="owner === keyPair.publicKey" class="col-sm-6">
              <h2>Close auction</h2>
              <form class="mt-3" @submit.prevent="closeAuction">
                <button type="submit" class="btn btn-lg btn-block btn-primary">Close</button>
              </form>
            </div>
            <div v-else class="col-sm-6">
              <h2>Make bid</h2>
              <form class="mt-3" @submit.prevent="makeBid">
                <div class="form-group">
                  <label class="control-label">Price:</label>
                  <input v-model="price" type="number" class="form-control" placeholder="Enter price" min="0" required>
                </div>
                <button type="submit" class="btn btn-lg btn-block btn-primary">Make bid</button>
              </form>
            </div>
          </div>
        </div>
      </div>
    </div>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import { mapState } from 'vuex'
  import Spinner from '../components/Spinner.vue'
  import OwlIcon from '../components/OwlIcon.vue'
  import Countdown from '../components/Countdown.vue'

  module.exports = {
    components: {
      Spinner,
      OwlIcon,
      Countdown
    },
    props: ['id'],
    data() {
      return {
        auction: {},
        owl: {},
        owner: '',
        lastBreeding: {},
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
          this.auction = await this.$blockchain.getAuction(this.id)
          this.isSpinnerVisible = false
          this.loadOwl()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async loadOwl() {
        this.isSpinnerVisible = true

        try {
          const data = await this.$blockchain.getOwl(this.auction.owl_id)
          this.owl = data.owl
          this.owner = data.owner
          this.lastBreeding = data.last_breeding
          this.isSpinnerVisible = false
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async makeBid() {
        this.isSpinnerVisible = true

        try {
          await this.$blockchain.makeBid(this.keyPair, this.auction.id, this.price)
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
          await this.$blockchain.closeAuction(this.keyPair, this.auction.id)
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
