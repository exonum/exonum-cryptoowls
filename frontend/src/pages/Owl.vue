<template>
  <div>
    <div class="container">
      <div class="row">
        <div class="col-sm-12">
          <h1>Owl</h1>

          <h2>Summary</h2>
          <p>{{ name }}</p>
          <p>{{ dna }}</p>

          <h2>Orders</h2>
          <ul>
            <li v-for="order in orders">
              <p>{{ order.public_key }}</p>
              <p>{{ order.status }}</p>
              <p>{{ order.price }}</p>
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
