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
      <div class="alert alert-warning" role="alert">Save the secret key in a safe place. You will need it to log in to the demo next time.</div>
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
    data() {
      return {
        keyPair: {},
        isModalVisible: false,
        isSpinnerVisible: false
      }
    },
    methods: {
      login() {
        this.isSpinnerVisible = true

        this.$store.commit('login', {
          publicKey: this.secretKey.substr(64),
          secretKey: this.secretKey
        })

        this.$nextTick(function() {
          this.$router.push({name: 'dashboard'})
        })
      },

      async register() {
        this.isSpinnerVisible = true

        try {
          const keyPair = await this.$blockchain.createUser(this.name)
          this.name = ''
          this.keyPair = keyPair
          this.isSpinnerVisible = false
          this.isModalVisible = true
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      closeModal() {
        this.isModalVisible = false
      },

      proceed() {
        this.isModalVisible = false

        this.$store.commit('login', this.keyPair)

        this.$nextTick(function() {
          this.$router.push({name: 'dashboard'})
        })
      }
    }
  }
</script>
