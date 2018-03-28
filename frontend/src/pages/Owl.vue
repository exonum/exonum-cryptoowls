<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Сова</h1>

          <h2 class="mt-5">Профиль</h2>
          <ul class="list-group mt-3">
            <li class="list-group-item">
              <div class="row">
                <div class="col-sm-3"><strong>Кличка:</strong></div>
                <div class="col-sm-9">{{ name }}</div>
              </div>
            </li>
            <li class="list-group-item">
              <div class="row">
                <div class="col-sm-3"><strong>ДНК:</strong></div>
                <div class="col-sm-9">
                  <code>{{ dna }}</code>
                </div>
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
      </div>
    </div>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  const Spinner = require('../components/Spinner.vue')

  module.exports = {
    components: {
      Spinner
    },
    props: ['dna'],
    data: function() {
      return {
        isSpinnerVisible: false
      }
    },
    methods: {
      loadOwl: function() {
        const self = this
        this.isSpinnerVisible = true
        this.$blockchain.getOwl(this.dna).then(data => {
          self.name = data.name
          self.$blockchain.getOrders(self.dna).then(data => {
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
