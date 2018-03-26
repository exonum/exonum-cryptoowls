<template>
  <div>
    <div class="container">
      <div class="row">
        <div class="col-sm-12">
          <h1>Owls</h1>

          <owl-list v-bind:owls="owls"></owl-list>
        </div>
      </div>
    </div>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  const Spinner = require('../components/Spinner.vue')
  const OwlList = require('../components/OwlList.vue')

  module.exports = {
    components: {
      Spinner,
      OwlList
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
        }).catch(error => {
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
