<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Auction</h1>

          <div class="row mt-5">
            <div class="col-sm-6 col-md-4">
              <h2>Summary</h2>
              <ul class="list-group mt-3">
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>ID:</strong></div>
                    <div class="col-sm-9">{{ id }}</div>
                  </div>
                </li>
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Started at:</strong></div>
                    <div class="col-sm-9">{{ $moment.getDate(startedAt) }}</div>
                  </div>
                </li>
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Is closed:</strong></div>
                    <div class="col-sm-9">{{ closed }}</div>
                  </div>
                </li>
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Bidding Merkle root:</strong></div>
                    <div class="col-sm-9">
                      <code>{{ biddingMerkleRoot }}</code>
                    </div>
                  </div>
                </li>
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Duration:</strong></div>
                    <div class="col-sm-9">{{ auction.duration }}</div>
                  </div>
                </li>
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Start price:</strong></div>
                    <div class="col-sm-9">{{ auction.start_price }}</div>
                  </div>
                </li>
              </ul>
            </div>
            <div class="col-sm-6 col-md-4">
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
            <div class="col-sm-6 col-md-4">
              <owl-icon v-if="owl.dna" v-bind:dna="owl.dna"/>
            </div>
          </div>

          <div class="row mt-5">
            <div v-if="owner !== keyPair.publicKey" class="col-sm-6">
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
        biddingMerkleRoot: '',
        closed: false,
        id: '',
        startedAt: {},
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
          const data = await this.$blockchain.getAuction(this.id)
          this.auction = data.auction
          this.biddingMerkleRoot = data.bidding_merkle_root
          this.closed = data.closed
          this.id = data.id
          this.startedAt = data.started_at
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
      }
    },
    mounted() {
      this.$nextTick(function() {
        this.loadAuction()
      })
    }
  }
</script>
