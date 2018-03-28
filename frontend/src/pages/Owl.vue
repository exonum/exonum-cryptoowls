<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Сова</h1>

          <div class="row">
            <div class="col-sm-6">
              <h2 class="mt-5">Профиль</h2>
              <ul class="list-group mt-3">
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Кличка:</strong></div>
                    <div class="col-sm-9">{{ owl.name }}</div>
                  </div>
                </li>
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>ДНК:</strong></div>
                    <div class="col-sm-9">
                      <code>{{ owl.dna }}</code>
                    </div>
                  </div>
                </li>
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Хозяин:</strong></div>
                    <div class="col-sm-9">
                      <code>
                        <router-link :to="{ name: 'user', params: { publicKey: owner } }" class="break-word">{{ owner }}</router-link>
                      </code>
                    </div>
                  </div>
                </li>
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Последнее разведение:</strong></div>
                    <div class="col-sm-9">{{ $moment(lastBreeding) }}</div>
                  </div>
                </li>
              </ul>

              <h2 class="mt-5">Ставки</h2>
              <ul v-if="isOwner" class="list-group mt-3">
                <li class="list-group-item font-weight-bold">
                  <div class="row">
                    <div class="col-sm-3">Пользователь</div>
                    <div class="col-sm-3">Статус</div>
                    <div class="col-sm-3">Цена</div>
                    <div class="col-sm-3">Действие</div>
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
                    <div class="col-sm-3">
                      <button type="submit" class="btn btn-primary" @click.prevent="acceptOrder(order)">Продать</button>
                    </div>
                  </div>
                </li>
              </ul>
              <ul v-else class="list-group mt-3">
                <li class="list-group-item font-weight-bold">
                  <div class="row">
                    <div class="col-sm-4">Пользователь</div>
                    <div class="col-sm-4">Статус</div>
                    <div class="col-sm-4">Цена</div>
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
                <h2 class="mt-5">Моя ставка</h2>
                <form class="mt-3" @submit.prevent="createOrder">
                  <div class="form-group">
                    <label class="control-label">Сумма:</label>
                    <input v-model="price" class="form-control" type="text">
                  </div>
                  <button type="submit" class="btn btn-lg btn-block btn-primary">Предложить цену</button>
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
  const Spinner = require('../components/Spinner.vue')
  const OwlIcon = require('../components/OwlIcon.vue')

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
          self.$notify('success', 'Ставка сделана')
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
          self.$notify('success', 'Сова продана')
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
