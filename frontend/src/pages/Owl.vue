<template>
  <div>
    <h1>Owl {{ dna }}</h1>

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
        this.$blockchain.getOwl(dna).then(data => {
          self.name = data.name
          self.$blockchain.getOrders(dna).then(data => {
            self.orders = data
            self.isSpinnerVisible = false
          }).catch(function(error) {
            self.$notify('error', error.toString())
            self.isSpinnerVisible = false
          })
        }).catch(function(error) {
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
