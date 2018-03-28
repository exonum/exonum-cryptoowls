<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Личный кабинет</h1>

          <div class="row mt-5">
            <div class="col-sm-6">
              <h2>Мой профиль</h2>
              <user-summary v-bind:user="user" class="mt-3"/>
              <button class="btn btn-lg btn-block btn-primary mt-3" @click.prevent="issue">Пополнить счёт</button>
            </div>
            <div class="col-sm-6">
              <h2>Инкубатор</h2>
              <form class="mt-3" @submit.prevent="makeOwl">
                <div class="form-group">
                  <label class="control-label">Кличка:</label>
                  <input v-model="name" class="form-control" type="text">
                </div>
                <div class="form-group">
                  <label class="control-label">Отец:</label>
                  <select v-model="father" class="form-control">
                    <option v-for="owl in owls" class="form-control" :value="$blockchain.getOwlHash(owl.owl)">{{ owl.owl.name }}</option>
                  </select>
                </div>
                <div class="form-group">
                  <label class="control-label">Мать:</label>
                  <select v-model="mother" class="form-control">
                    <option v-for="owl in owls" class="form-control" :value="$blockchain.getOwlHash(owl.owl)">{{ owl.owl.name }}</option>
                  </select>
                </div>
                <button type="submit" class="btn btn-lg btn-block btn-primary">Скретить</button>
              </form>
            </div>
          </div>

          <h2 class="mt-5">Мои совы</h2>
          <owl-list v-bind:owls="owls"/>

          <h2 class="mt-5">Предложения мне</h2>
          <ul class="list-group mt-3">
            <li class="list-group-item font-weight-bold">
              <div class="row">
                <div class="col-sm-3">Сова</div>
                <div class="col-sm-3">Пользователь</div>
                <div class="col-sm-2">Статус</div>
                <div class="col-sm-2">Цена</div>
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
                <div class="col-sm-2">{{ order.status }}</div>
                <div class="col-sm-2">{{ order.price }}</div>
                <div v-if="order.status === 'pending'" class="col-sm-2">
                  <button type="submit" class="btn btn-primary" @click.prevent="acceptOrder(order)">Продать</button>
                </div>
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

        if (this.$store.state.keyPair === null) {
          this.$store.commit('logout')
          this.$router.push({name: 'auth'})
          return
        }

        this.isSpinnerVisible = true

        this.$blockchain.getUser(this.$store.state.keyPair.publicKey).then(data => {
          self.user = data
          this.isSpinnerVisible = false
          self.loadOwls()
        }).catch(error => {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      },

      loadOwls: function() {
        const self = this

        this.isSpinnerVisible = true

        this.$blockchain.getUserOwls(self.$store.state.keyPair.publicKey).then(data => {
          self.owls = data
          self.isSpinnerVisible = false
          self.loadOrders()
        }).catch(error => {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      },

      loadOrders: function() {
        const self = this

        this.isSpinnerVisible = true

        this.$blockchain.getUserOrders(self.$store.state.keyPair.publicKey).then(data => {
          self.orders = data
          self.isSpinnerVisible = false
        }).catch(error => {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      },

      issue: function() {
        const self = this

        this.isSpinnerVisible = true

        this.$blockchain.issue(this.$store.state.keyPair).then(data => {
          self.$notify('success', 'Счёт пополнен')
          self.isSpinnerVisible = false
          self.loadUser()
        }).catch(error => {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      },

      makeOwl: function() {
        const self = this

        this.isSpinnerVisible = true

        this.$blockchain.makeOwl(this.$store.state.keyPair, this.name, this.mother, this.father).then(data => {
          self.$notify('success', 'Инкубация прошла успешно')
          self.isSpinnerVisible = false
          self.loadUser()
        }).catch(error => {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      },

      acceptOrder: function(order) {
        const self = this

        this.isSpinnerVisible = true

        self.$blockchain.acceptOrder(this.$store.state.keyPair, this.$blockchain.getOrderHash(order)).then(data => {
          self.$notify('success', 'Сова продана')
          self.isSpinnerVisible = false
          self.loadUser()
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
