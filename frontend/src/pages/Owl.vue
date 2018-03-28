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
              <ul class="list-group mt-3">
                <li class="list-group-item font-weight-bold">
                  <div class="row">
                    <div class="col-sm-4">Публичный ключ</div>
                    <div class="col-sm-4">Статус</div>
                    <div class="col-sm-4">Цена</div>
                  </div>
                </li>
                <li v-for="order in orders" class="list-group-item">
                  <div class="row">
                    <div class="col-sm-4">
                      <code>{{ order.public_key }}</code>
                    </div>
                    <div class="col-sm-4">{{ order.status }}</div>
                    <div class="col-sm-4">{{ order.price }}</div>
                  </div>
                </li>
              </ul>
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
          self.lastBreeding = data.last_breeding
          self.$blockchain.getOrders(self.hash).then(data => {
            self.orders = data
            self.isSpinnerVisible = false
          }).catch(error => {
            self.$notify('error', error.toString())
            self.isSpinnerVisible = false
          })
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
