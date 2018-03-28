<template>
  <div>
    <div class="container mt-5">
      <div class="row justify-content-sm-center">
        <div class="col-md-6 col-md-offset-3">
          <h1 class="mt-5 mb-4">Авторизация</h1>
          <tabs>
            <tab :is-active="true" title="Регистрация">
              <form @submit.prevent="register">
                <div class="form-group">
                  <label class="control-label">Имя:</label>
                  <input v-model="name" type="text" class="form-control" placeholder="Введите имя" maxlength="260" required>
                </div>
                <button type="submit" class="btn btn-lg btn-block btn-primary">Зарегистрироваться</button>
              </form>
            </tab>
            <tab title="Вход">
              <form @submit.prevent="login">
                <div class="form-group">
                  <label class="control-label">Публичный ключ:</label>
                  <input v-model="publicKey" type="text" class="form-control" placeholder="Введите публичный ключ" required>
                </div>
                <div class="form-group">
                  <label class="control-label">Секретный ключ:</label>
                  <input v-model="secretKey" type="text" class="form-control" placeholder="Введите секретный ключ" required>
                </div>
                <button type="submit" class="btn btn-lg btn-block btn-primary">Войти</button>
              </form>
            </tab>
          </tabs>
        </div>
      </div>
    </div>

    <modal :visible="isModalVisible" title="Успешная регистрация" action-btn="Войти" @close="closeModal" @submit="proceed">
      <div class="alert alert-warning" role="alert">Сохраните пару ключей в безопасном месте. Ключи понадобятся для следующего входа.</div>
      <div class="form-group">
        <label>Публичный ключ:</label>
        <div><code>{{ keyPair.publicKey }}</code></div>
      </div>
      <div class="form-group">
        <label>Секретный ключ:</label>
        <div><code>{{ keyPair.secretKey }}</code></div>
      </div>
    </modal>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  const Tab = require('../components/Tab.vue')
  const Tabs = require('../components/Tabs.vue')
  const Modal = require('../components/Modal.vue')
  const Spinner = require('../components/Spinner.vue')

  module.exports = {
    components: {
      Tab,
      Tabs,
      Modal,
      Spinner
    },
    data: function() {
      return {
        keyPair: {},
        isModalVisible: false,
        isSpinnerVisible: false
      }
    },
    methods: {
      login: function() {
        if (!this.$validateHex(this.publicKey)) {
          return this.$notify('error', 'Некорректный публичный кюч')
        }

        if (!this.$validateHex(this.secretKey, 64)) {
          return this.$notify('error', 'Некорректный секретный ключ')
        }

        this.isSpinnerVisible = true

        this.$store.commit('login', {
          publicKey: this.publicKey,
          secretKey: this.secretKey
        })

        this.$nextTick(function() {
          this.$router.push({name: 'dashboard'})
        })
      },

      register: function() {
        const self = this

        if (!this.name) {
          return this.$notify('error', 'Имя не может быть пустым')
        }

        this.isSpinnerVisible = true

        this.$blockchain.createUser(this.name).then(keyPair => {
          self.name = ''
          self.keyPair = keyPair
          self.isSpinnerVisible = false
          self.isModalVisible = true
        }).catch(error => {
          self.isSpinnerVisible = false
          self.$notify('error', error.toString())
        })
      },

      closeModal: function() {
        this.isModalVisible = false
      },

      proceed: function() {
        this.isModalVisible = false

        this.$store.commit('login', this.keyPair)

        this.$nextTick(function() {
          this.$router.push({name: 'dashboard'})
        })
      }
    }
  }
</script>
