<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Пользователь</h1>

          <h2 class="mt-5">Профиль</h2>
          <user-summary v-bind:user="user" class="mt-3"/>

          <h2 class="mt-5">Совы</h2>
          <owl-list v-bind:owls="owls"/>

          <h2 class="mt-5">Предложения, сделанные пользователем</h2>
          <ul class="list-group mt-3">
            <li class="list-group-item font-weight-bold">
              <div class="row">
                <div class="col-sm-3">Сова</div>
                <div class="col-sm-3">Пользователь</div>
                <div class="col-sm-3">Статус</div>
                <div class="col-sm-3">Цена</div>
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
  const Spinner = require('../components/Spinner.vue')
  const UserSummary = require('../components/UserSummary.vue')
  const OwlList = require('../components/OwlList.vue')

  module.exports = {
    components: {
      Spinner,
      UserSummary,
      OwlList
    },
    props: ['publicKey'],
    data: function() {
      return {
        user: [],
        owls: [],
        orders: [],
        isSpinnerVisible: false
      }
    },
    methods: {
      loadUser: function() {
        const self = this
        this.isSpinnerVisible = true
        this.$blockchain.getUser(this.publicKey).then(data => {
          self.user = data
          self.$blockchain.getUserOwls(self.publicKey).then(data => {
            self.owls = data
            self.isSpinnerVisible = false
            self.loadOrders()
          }).catch(error => {
            self.$notify('error', error.toString())
            self.isSpinnerVisible = false
          })
        }).catch(error => {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      },

      loadOrders: function() {
        const self = this

        this.isSpinnerVisible = true

        this.$blockchain.getUserOrders(this.publicKey).then(data => {
          self.orders = data
          self.isSpinnerVisible = false
        }).catch(error => {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      }
    },
    mounted: function() {
      this.$nextTick(function() {
        this.loadUser()
      })
    }
  }
</script>
