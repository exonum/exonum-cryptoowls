<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Транзакция</h1>

          <h2 class="mt-5">Транзакции</h2>
          <ul class="list-group">
            <li class="list-group-item">
              <div class="row">
                <div class="col-sm-3"><strong>Хеш:</strong></div>
                <div class="col-sm-9">{{ hash }}</div>
              </div>
            </li>
            <li class="list-group-item">
              <div class="row">
                <div class="col-sm-3"><strong>Блок:</strong></div>
                <div class="col-sm-9">
                  <router-link :to="{ name: 'block', params: { height: location.block_height } }">{{ location.block_height }}</router-link>
                </div>
              </div>
            </li>
            <li class="list-group-item">
              <div class="row">
                <div class="col-sm-3"><strong>Тип:</strong></div>
                <div class="col-sm-9">
                  <code>{{ type }}</code>
                </div>
              </div>
            </li>
            <li class="list-group-item">
              <div class="row">
                <div class="col-sm-3"><strong>Версия протокола:</strong></div>
                <div class="col-sm-9">{{ transaction.protocol_version }}</div>
              </div>
            </li>
            <li class="list-group-item">
              <div class="row">
                <div class="col-sm-3"><strong>ID сети:</strong></div>
                <div class="col-sm-9">{{ transaction.network_id }}</div>
              </div>
            </li>
            <li class="list-group-item">
              <div class="row">
                <div class="col-sm-3"><strong>ID сервиса:</strong></div>
                <div class="col-sm-9">{{ transaction.service_id }}</div>
              </div>
            </li>
            <li class="list-group-item">
              <div class="row">
                <div class="col-sm-3"><strong>ID транзакции:</strong></div>
                <div class="col-sm-9">{{ transaction.message_id }}</div>
              </div>
            </li>
            <li class="list-group-item">
              <div class="row">
                <div class="col-sm-3"><strong>Подпись:</strong></div>
                <div class="col-sm-9">{{ transaction.signature }}</div>
              </div>
            </li>
            <li class="list-group-item">
              <div class="row">
                <div class="col-sm-3"><strong>Содержимое транзакции:</strong></div>
                <div class="col-sm-9">
                  <pre><code>{{ JSON.stringify(transaction.body, null, 2) }}</code></pre>
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
      hash: String
    },
    data: function() {
      return {
        transaction: {},
        location: {},
        type: ''
      }
    },
    methods: {
      loadTransaction: function() {
        const self = this

        this.$blockchain.getTransaction(this.hash).then(response => {
          self.transaction = response.data.content
          self.location = response.data.location
          self.type = response.data.type
          self.isSpinnerVisible = false
        }).catch(error => {
          self.$notify('error', error.toString())
          self.isSpinnerVisible = false
        })
      }
    },
    mounted: function() {
      this.$nextTick(function() {
        this.loadTransaction()
      })
    }
  }
</script>
