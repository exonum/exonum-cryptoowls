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
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Owner:</strong></div>
                    <div class="col-sm-9">
                      <code>
                        <router-link :to="{ name: 'user', params: { publicKey: owner } }" class="break-word">{{ owner }}</router-link>
                      </code>
                    </div>
                  </div>
                </li>
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Last breeding:</strong></div>
                    <div class="col-sm-9">{{ $moment(lastBreeding) }}</div>
                  </div>
                </li>
              </ul>

              <h2 class="mt-5">Orders</h2>
              <ul v-if="isOwner" class="list-group mt-3">
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

              <div v-if="$store.state.keyPair && !isOwner">
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
  import Spinner from '../components/Spinner.vue'
  import OwlIcon from '../components/OwlIcon.vue'

  module.exports = {
    components: {
      Spinner,
      OwlIcon
    },
    props: ['hash'],
    data: function() {
      return {
        owl: {},
        owner: '',
        isOwner: false,
        lastBreeding: {},
        isSpinnerVisible: false
      }
    },
    methods: {
      loadOwl: function() {
        const self = this

        this.isSpinnerVisible = true

        this.$blockchain.getOwl(this.hash).then(data => {
          self.owl = data.owl
          self.owner = data.owner
          if (self.$store.state.keyPair) {
            self.isOwner = self.$store.state.keyPair.publicKey === data.owner
          }
          self.isSpinnerVisible = false
          self.lastBreeding = data.last_breeding
          self.loadOrders()
        }).catch(error => {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      },

      loadOrders: function() {
        const self = this

        this.isSpinnerVisible = true

        self.$blockchain.getOrders(this.hash).then(data => {
          self.orders = data
          self.isSpinnerVisible = false
        }).catch(error => {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      },

      createOrder: function() {
        const self = this

        this.isSpinnerVisible = true

        self.$blockchain.createOrder(this.$store.state.keyPair, this.$blockchain.getOwlHash(this.owl), this.price).then(data => {
          self.$notify('success', 'Transaction accepted')
          self.isSpinnerVisible = false
          self.loadOwl()
        }).catch(error => {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      },

      acceptOrder: function(order) {
        const self = this

        this.isSpinnerVisible = true

        self.$blockchain.acceptOrder(this.$store.state.keyPair, this.$blockchain.getOrderHash(order)).then(data => {
          self.$notify('success', 'Transaction accepted')
          self.isSpinnerVisible = false
          self.loadOwl()
        }).catch(error => {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      }
    },
    mounted: function() {
      this.$nextTick(function() {
        this.loadOwl()
      })
    }
  }
</script>
