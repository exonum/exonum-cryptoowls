<template>
  <div>
    <h1>Owls</h1>

    <owl-list v-bind:owls="owls"></owl-list>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  const Spinner = require('../components/Spinner.vue')

  module.exports = {
    components: {
      Spinner
    },
    data: function() {
      return {
        owls: Array
      }
    },
    methods: {
      loadOwls: function() {
        const self = this

        this.isSpinnerVisible = true

        this.$blockchain.getOwls().then(owls => {
          self.owls = owls
          self.isSpinnerVisible = false
        }).catch(function(error) {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      }
    },
    mounted: function() {
      this.$nextTick(function() {
        this.loadOwls()
      })
    }
  }
</script>
