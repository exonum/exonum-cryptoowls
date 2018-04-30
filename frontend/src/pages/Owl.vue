<template>
  <div>
    <div class="container mt-5">
      <div class="row">
        <div class="col-sm-12">
          <h1>Owl</h1>

          <div class="row">
            <div class="col-sm-6">
              <h2 class="mt-5">Profile</h2>
              <ul class="list-group mt-3">
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Name:</strong></div>
                    <div class="col-sm-9">{{ owl.name }}</div>
                  </div>
                </li>
                <li class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>DNA:</strong></div>
                    <div class="col-sm-9">
                      <code>{{ owl.dna }}</code>
                    </div>
                  </div>
                </li>
                <li v-if="owner" class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Owner:</strong></div>
                    <div class="col-sm-9">
                      <code>
                        <router-link :to="{ name: 'user', params: { publicKey: owner } }" class="break-word">{{ owner }}</router-link>
                      </code>
                    </div>
                  </div>
                </li>
                <li v-if="lastBreeding" class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Last breeding:</strong></div>
                    <div class="col-sm-9">{{ $moment.getDate(lastBreeding) }}</div>
                  </div>
                </li>
                <li v-if="owner === keyPair.publicKey && lastBreeding" class="list-group-item">
                  <div class="row">
                    <div class="col-sm-3"><strong>Ready for breeding:</strong></div>
                    <div class="col-sm-9"><countdown v-bind:date="lastBreeding"/></div>
                  </div>
                </li>
              </ul>
            </div>
            <div class="col-sm-6">
              <owl-icon v-if="owl.dna" v-bind:dna="owl.dna" class="mt-5"/>
            </div>
          </div>
        </div>
      </div>
    </div>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import { mapState } from 'vuex'
  import Spinner from '../components/Spinner.vue'
  import OwlIcon from '../components/OwlIcon.vue'
  import Countdown from '../components/Countdown.vue'

  module.exports = {
    components: {
      Spinner,
      OwlIcon,
      Countdown
    },
    props: ['hash'],
    data() {
      return {
        owl: {},
        owner: '',
        lastBreeding: {},
        isSpinnerVisible: false
      }
    },
    computed: mapState({
      keyPair: state => state.keyPair
    }),
    methods: {
      async loadOwl() {
        this.isSpinnerVisible = true

        try {
          const data = await this.$blockchain.getOwl(this.hash)
          this.owl = data.owl
          this.owner = data.owner
          this.isSpinnerVisible = false
          this.lastBreeding = data.last_breeding
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      }
    },
    mounted() {
      this.$nextTick(function() {
        this.loadOwl()
      })
    }
  }
</script>
