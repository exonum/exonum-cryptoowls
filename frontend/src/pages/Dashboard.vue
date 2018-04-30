<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>My cabinet</h1>

          <div class="row mt-5">
            <div class="col-sm-6">
              <h2>My profile</h2>
              <user-summary v-bind:user="user" class="mt-3"/>
              <button class="btn btn-lg btn-block btn-primary mt-3" @click.prevent="issue">Issue funds</button>
            </div>
            <div class="col-sm-6">
              <h2>Incubator</h2>
              <form class="mt-3" @submit.prevent="makeOwl">
                <div class="form-group">
                  <label class="control-label">Name:</label>
                  <input v-model="name" type="text" class="form-control" placeholder="Enter name" maxlength="260" required>
                </div>
                <div class="form-group">
                  <label class="control-label">Father:</label>
                  <select v-model="father" class="form-control" required>
                    <option v-for="owl in owls" class="form-control" :value="$blockchain.getOwlHash(owl.owl)">{{ owl.owl.name }}</option>
                  </select>
                </div>
                <div class="form-group">
                  <label class="control-label">Mother:</label>
                  <select v-model="mother" class="form-control" required>
                    <option v-for="owl in owls" class="form-control" :value="$blockchain.getOwlHash(owl.owl)">{{ owl.owl.name }}</option>
                  </select>
                </div>
                <button type="submit" class="btn btn-lg btn-block btn-primary">Incubate</button>
              </form>
            </div>
          </div>

          <h2 class="mt-5">My owls</h2>
          <owl-list v-bind:owls="owls"/>

          <h2 class="mt-5">My auctions</h2>
          <ul class="list-group mt-3">
            <li class="list-group-item font-weight-bold">
              <div class="row">
                <div class="col-sm-3">Auction</div>
                <div class="col-sm-3">Owl</div>
                <div class="col-sm-3">Start price</div>
                <div class="col-sm-3">Duration</div>
              </div>
            </li>
            <li v-for="auction in auctions" class="list-group-item">
              <div class="row">
                <div class="col-sm-3">
                  <code>
                    <router-link :to="{ name: 'auction', params: { hash: auction.id } }" class="break-word">{{ auction.id }}</router-link>
                  </code>
                </div>
                <div class="col-sm-3">
                  <code>
                    <router-link :to="{ name: 'owl', params: { hash: auction.owl_id } }" class="break-word">{{ auction.owl_id }}</router-link>
                  </code>
                </div>
                <div class="col-sm-3">{{ auction.start_price }}</div>
                <div class="col-sm-3">{{ auction.duration }}</div>
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
  import { mapState } from 'vuex'
  import Spinner from '../components/Spinner.vue'
  import UserSummary from '../components/UserSummary.vue'
  import OwlList from '../components/OwlList.vue'

  module.exports = {
    components: {
      Spinner,
      UserSummary,
      OwlList
    },
    data() {
      return {
        user: {},
        owls: [],
        auctions: [],
        isSpinnerVisible: false
      }
    },
    computed: mapState({
      keyPair: state => state.keyPair
    }),
    methods: {
      async loadUser() {
        if (this.keyPair === null) {
          this.$store.commit('logout')
          this.$router.push({name: 'auth'})
          return
        }

        this.isSpinnerVisible = true

        try {
          this.user = await this.$blockchain.getUser(this.keyPair.publicKey)
          this.isSpinnerVisible = false
          this.loadOwls()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
          this.$store.commit('logout')
          this.$router.push({name: 'auth'})
        }
      },

      async loadOwls() {
        this.isSpinnerVisible = true

        try {
          this.owls = await this.$blockchain.getUserOwls(this.keyPair.publicKey)
          this.isSpinnerVisible = false
          this.loadAuctions()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async loadAuctions() {
        this.isSpinnerVisible = true

        try {
          this.auctions = await this.$blockchain.getUserAuctions(this.keyPair.publicKey)
          this.isSpinnerVisible = false
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async issue() {
        this.isSpinnerVisible = true

        try {
          await this.$blockchain.issue(this.keyPair)
          this.isSpinnerVisible = false
          this.$notify('success', 'Transaction accepted')
          this.loadUser()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async makeOwl() {
        this.isSpinnerVisible = true

        try {
          await this.$blockchain.makeOwl(this.keyPair, this.name, this.mother, this.father)
          this.isSpinnerVisible = false
          this.$notify('success', 'Transaction accepted')
          this.loadUser()
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      }
    },
    mounted() {
      this.$nextTick(function() {
        this.loadUser()
      })
    }
  }
</script>
