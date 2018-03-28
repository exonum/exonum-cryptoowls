<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Все совы</h1>

          <owl-list v-bind:owls="owls" class="mt-3"/>
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
        owls: [],
        isSpinnerVisible: false
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
