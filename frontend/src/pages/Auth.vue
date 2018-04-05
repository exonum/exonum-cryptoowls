<template>
  <div>
    <div class="container mt-5">
      <div class="row justify-content-sm-center">
        <div class="col-md-6 col-md-offset-3">
          <h1 class="mt-5 mb-4">Authorization</h1>
          <tabs>
            <tab :is-active="true" title="Registration">
              <form @submit.prevent="register">
                <div class="form-group">
                  <label class="control-label">Name:</label>
                  <input v-model="name" type="text" class="form-control" placeholder="Enter name" maxlength="260" required>
                </div>
                <button type="submit" class="btn btn-lg btn-block btn-primary">Register</button>
              </form>
            </tab>
            <tab title="Log In">
              <form @submit.prevent="login">
                <div class="form-group">
                  <label class="control-label">Public key:</label>
                  <input v-model="publicKey" type="text" class="form-control" placeholder="Enter public key" required>
                </div>
                <div class="form-group">
                  <label class="control-label">Secret key:</label>
                  <input v-model="secretKey" type="text" class="form-control" placeholder="Enter secret key" required>
                </div>
                <button type="submit" class="btn btn-lg btn-block btn-primary">Log In</button>
              </form>
            </tab>
          </tabs>
        </div>
      </div>
    </div>

    <modal :visible="isModalVisible" title="Successful registration" action-btn="Log In" @close="closeModal" @submit="proceed">
      <div class="alert alert-warning" role="alert">Keep key pair in safe place. You will need next time to log in.</div>
      <div class="form-group">
        <label>Public key:</label>
        <div><code>{{ keyPair.publicKey }}</code></div>
      </div>
      <div class="form-group">
        <label>Secret key:</label>
        <div><code>{{ keyPair.secretKey }}</code></div>
      </div>
    </modal>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import Spinner from '../components/Spinner.vue'
  import Tab from '../components/Tab.vue'
  import Tabs from '../components/Tabs.vue'
  import Modal from '../components/Modal.vue'

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
