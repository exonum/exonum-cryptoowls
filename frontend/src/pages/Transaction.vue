<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Транзакция</h1>

          <ul class="list-group mt-5">
            <li class="list-group-item">
              <div class="row">
                <div class="col-sm-3"><strong>Хеш:</strong></div>
                <div class="col-sm-9">
                  <code>{{ hash }}</code>
                </div>
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
                <div class="col-sm-3"><strong>Статус:</strong></div>
                <div class="col-sm-9">
                  <code>{{ status.type }}</code>
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
                <div class="col-sm-9">
                  <code>{{ transaction.signature }}</code>
                </div>
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
        status: {},
        type: ''
      }
    },
    methods: {
      loadTransaction: function() {
        const self = this

        this.$blockchain.getTransaction(this.hash).then(data => {
          self.transaction = data.content
          self.location = data.location
          self.status = data.status
          self.type = data.type
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
