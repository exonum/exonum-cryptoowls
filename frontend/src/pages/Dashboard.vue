<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>My cabinet</h1>

          <div class="row mt-5">
            <div class="col-sm-6">
              <h2>My profile</h2>
              <user-summary v-bind:user="user" class="mt-3"/>
              <button class="btn btn-lg btn-block btn-primary mt-3" @click.prevent="issue">Issue funds</button>
            </div>
            <div class="col-sm-6">
              <h2>Incubator</h2>
              <form class="mt-3" @submit.prevent="makeOwl">
                <div class="form-group">
                  <label class="control-label">Name:</label>
                  <input v-model="name" type="text" class="form-control" placeholder="Enter name" maxlength="260" required>
                </div>
                <div class="form-group">
                  <label class="control-label">Father:</label>
                  <select v-model="father" class="form-control" required>
                    <option v-for="owl in owls" class="form-control" :value="$blockchain.getOwlHash(owl.owl)">{{ owl.owl.name }}</option>
                  </select>
                </div>
                <div class="form-group">
                  <label class="control-label">Mother:</label>
                  <select v-model="mother" class="form-control" required>
                    <option v-for="owl in owls" class="form-control" :value="$blockchain.getOwlHash(owl.owl)">{{ owl.owl.name }}</option>
                  </select>
                </div>
                <button type="submit" class="btn btn-lg btn-block btn-primary">Incubate</button>
              </form>
            </div>
          </div>

          <h2 class="mt-5">My owls</h2>
          <owl-list v-bind:owls="owls"/>

          <h2 class="mt-5">My orders</h2>
          <ul class="list-group mt-3">
            <li class="list-group-item font-weight-bold">
              <div class="row">
                <div class="col-sm-3">Owl</div>
                <div class="col-sm-3">User</div>
                <div class="col-sm-3">Status</div>
                <div class="col-sm-3">Price</div>
              </div>
            </li>
            <li v-for="order in orders" class="list-group-item">
              <div class="row">
                <div class="col-sm-3">
                  <code>
                    <router-link :to="{ name: 'owl', params: { hash: order.owl_id } }" class="break-word">{{ order.owl_id }}</router-link>
                  </code>
                </div>
                <div class="col-sm-3">
                  <code>
                    <router-link :to="{ name: 'user', params: { publicKey: order.public_key } }" class="break-word">{{ order.public_key }}</router-link>
                  </code>
                </div>
                <div class="col-sm-3">{{ order.status }}</div>
                <div class="col-sm-3">{{ order.price }}</div>
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
  import UserSummary from '../components/UserSummary.vue'
  import OwlList from '../components/OwlList.vue'

  module.exports = {
    components: {
      Spinner,
      UserSummary,
      OwlList
    },
    data() {
      return {
        user: {},
        owls: [],
        orders: [],
        isSpinnerVisible: false
      }
    },
    methods: {
      async loadUser() {
        if (this.$store.state.keyPair === null) {
          this.$store.commit('logout')
          this.$router.push({name: 'auth'})
          return
        }

        this.isSpinnerVisible = true

        try {
          this.user = await this.$blockchain.getUser(this.$store.state.keyPair.publicKey)
          this.isSpinnerVisible = false
          this.loadOwls()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
          this.$store.commit('logout')
          this.$router.push({name: 'auth'})
        }
      },

      async loadOwls() {
        this.isSpinnerVisible = true

        try {
          this.owls = await this.$blockchain.getUserOwls(this.$store.state.keyPair.publicKey)
          this.isSpinnerVisible = false
          this.loadOrders()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async loadOrders() {
        this.isSpinnerVisible = true

        try {
          this.orders = await this.$blockchain.getUserOrders(this.$store.state.keyPair.publicKey)
          this.isSpinnerVisible = false
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async issue() {
        this.isSpinnerVisible = true

        try {
          await this.$blockchain.issue(this.$store.state.keyPair)
          this.isSpinnerVisible = false
          this.$notify('success', 'Transaction accepted')
          this.loadUser()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async makeOwl() {
        this.isSpinnerVisible = true

        try {
          await this.$blockchain.makeOwl(this.$store.state.keyPair, this.name, this.mother, this.father)
          this.isSpinnerVisible = false
          this.$notify('success', 'Transaction accepted')
          this.loadUser()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async acceptOrder(order) {
        this.isSpinnerVisible = true

        try {
          const orderHash = this.$blockchain.getOrderHash(order)
          await this.$blockchain.acceptOrder(this.$store.state.keyPair, orderHash)
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
        this.loadUser()
      })
    }
  }
</script>
