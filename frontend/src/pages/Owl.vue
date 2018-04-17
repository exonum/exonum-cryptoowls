<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Owl</h1>

          <div class="row">
            <div class="col-sm-6">
              <h2 class="mt-5">Profile</h2>
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

              <h2 class="mt-5">Orders</h2>
              <ul v-if="owner === keyPair.publicKey" class="list-group mt-3">
                <li class="list-group-item font-weight-bold">
                  <div class="row">
                    <div class="col-sm-3">User</div>
                    <div class="col-sm-3">Status</div>
                    <div class="col-sm-3">Price</div>
                  </div>
                </li>
                <li v-for="order in orders" class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3">
                      <code>
                        <router-link :to="{ name: 'user', params: { publicKey: order.public_key } }" class="break-word">{{ order.public_key }}</router-link>
                      </code>
                    </div>
                    <div class="col-sm-3">{{ order.status }}</div>
                    <div class="col-sm-3">{{ order.price }}</div>
                    <div v-if="order.status === 'pending'" class="col-sm-3">
                      <button type="submit" class="btn btn-primary" @click.prevent="acceptOrder(order)">Sell</button>
                    </div>
                  </div>
                </li>
              </ul>
              <ul v-else class="list-group mt-3">
                <li class="list-group-item font-weight-bold">
                  <div class="row">
                    <div class="col-sm-4">User</div>
                    <div class="col-sm-4">Status</div>
                    <div class="col-sm-4">Price</div>
                  </div>
                </li>
                <li v-for="order in orders" class="list-group-item">
                  <div class="row">
                    <div class="col-sm-4">
                      <code>
                        <router-link :to="{ name: 'user', params: { publicKey: order.public_key } }" class="break-word">{{ order.public_key }}</router-link>
                      </code>
                    </div>
                    <div class="col-sm-4">{{ order.status }}</div>
                    <div class="col-sm-4">{{ order.price }}</div>
                  </div>
                </li>
              </ul>

              <div v-if="keyPair && owner !== keyPair.publicKey">
                <h2 class="mt-5">My order</h2>
                <form class="mt-3" @submit.prevent="createOrder">
                  <div class="form-group">
                    <label class="control-label">Price:</label>
                    <input v-model="price" type="number" class="form-control" placeholder="Enter price" min="0" required>
                  </div>
                  <button type="submit" class="btn btn-lg btn-block btn-primary">Make order</button>
                </form>
              </div>
            </div>
            <div class="col-sm-6">
              <owl-icon v-if="owl.dna" v-bind:dna="owl.dna" class="mt-5"/>
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
    props: ['hash'],
    data() {
      return {
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
      async loadOwl() {
        this.isSpinnerVisible = true

        try {
          const data = await this.$blockchain.getOwl(this.hash)
          this.owl = data.owl
          this.owner = data.owner
          this.isSpinnerVisible = false
          this.lastBreeding = data.last_breeding
          this.loadOrders()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async loadOrders() {
        this.isSpinnerVisible = true

        try {
          this.orders = await this.$blockchain.getOrders(this.hash)
          this.isSpinnerVisible = false
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async createOrder() {
        this.isSpinnerVisible = true

        try {
          const owlHash = this.$blockchain.getOwlHash(this.owl)
          await this.$blockchain.createOrder(this.keyPair, owlHash, this.price)
          this.isSpinnerVisible = false
          this.$notify('success', 'Transaction accepted')
          this.loadOrders()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async acceptOrder(order) {
        this.isSpinnerVisible = true

        try {
          const orderHash = this.$blockchain.getOrderHash(order)
          await this.$blockchain.acceptOrder(this.keyPair, orderHash)
          this.isSpinnerVisible = false
          this.$notify('success', 'Transaction accepted')
          this.loadOwl()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      }
    },
    mounted() {
      this.$nextTick(function() {
        this.loadOwl()
      })
    }
  }
</script>
