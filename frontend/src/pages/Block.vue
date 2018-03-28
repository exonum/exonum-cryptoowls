<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Блок {{ height }}</h1>

          <h2 class="mt-5">Транзакции</h2>
          <ul class="list-group mt-3">
            <li class="list-group-item font-weight-bold">
              <div class="row">
                <div class="col-sm-12">Хеш</div>
              </div>
            </li>
            <li v-for="transaction in transactions" class="list-group-item">
              <div class="row">
                <div class="col-sm-12">
                  <router-link :to="{ name: 'transaction', params: { hash: transaction } }">{{ transaction }}</router-link>
                </div>
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
    props: {
      height: String
    },
    data: function() {
      return {
        transactions: [],
        isSpinnerVisible: false
      }
    },
    methods: {
      loadBlock: function() {
        const self = this

        this.isSpinnerVisible = true

        this.$blockchain.getBlock(this.height).then(data => {
          self.transactions = data.txs
          self.isSpinnerVisible = false
        }).catch(error => {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      }
    },
    mounted: function() {
      this.$nextTick(function() {
        this.loadBlock()
      })
    }
  }
</script>
