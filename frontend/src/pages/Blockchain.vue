<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Последние блоки</h1>

          <ul class="list-group mt-5">
            <li class="list-group-item font-weight-bold">
              <div class="row">
                <div class="col-sm-6">Высота</div>
                <div class="col-sm-6">Количество транзакций</div>
              </div>
            </li>
            <li v-for="block in blocks" class="list-group-item">
              <div class="row">
                <div class="col-sm-6">
                  <router-link :to="{ name: 'block', params: { height: block.height } }">{{ block.height }}</router-link>
                </div>
                <div class="col-sm-6">{{ block.tx_count }}</div>
              </div>
            </li>
          </ul>

          <button class="btn btn-primary mt-3" @click.prevent="loadMore">Предыдущие блоки</button>
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
    data() {
      return {
        isSpinnerVisible: false,
        blocks: []
      }
    },
    methods: {
      loadBlocks: function(latest) {
        const self = this

        this.isSpinnerVisible = true

        this.$blockchain.getBlocks(latest).then(blocks => {
          self.blocks = self.blocks.concat(blocks)
          self.isSpinnerVisible = false
        }).catch(error => {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      },

      loadMore: function() {
        this.loadBlocks(this.blocks[this.blocks.length - 1].height - 1)
      }
    },
    mounted: function() {
      this.$nextTick(function() {
        this.loadBlocks()
      })
    }
  }
</script>
