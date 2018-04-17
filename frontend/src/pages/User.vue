<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>User</h1>

          <h2 class="mt-5">Profile</h2>
          <user-summary v-bind:user="user" class="mt-3"/>

          <h2 class="mt-5">Owls</h2>
          <owl-list v-bind:owls="owls"/>

          <h2 class="mt-5">User orders</h2>
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
    props: ['publicKey'],
    data() {
      return {
        user: [],
        owls: [],
        orders: [],
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
          this.loadOrders()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async loadOrders() {
        this.isSpinnerVisible = true

        try {
          this.orders = await this.$blockchain.getUserOrders(this.publicKey)
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
