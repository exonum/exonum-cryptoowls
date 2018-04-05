<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>All owls</h1>

          <owl-list v-bind:owls="owls"/>
        </div>
      </div>
    </div>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import Spinner from '../components/Spinner.vue'
  import OwlList from '../components/OwlList.vue'

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
