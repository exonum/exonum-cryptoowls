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
    data() {
      return {
        owls: [],
        isSpinnerVisible: false
      }
    },
    methods: {
      async loadOwls() {
        this.isSpinnerVisible = true

        try {
          this.owls = await this.$blockchain.getOwls()
          this.isSpinnerVisible = false
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      }
    },
    mounted() {
      this.$nextTick(function() {
        this.loadOwls()
      })
    }
  }
</script>
