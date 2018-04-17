<template>
  <span v-if="countdown > 0">in a {{ countdown }} seconds</span>
  <span v-else>Ready!</span>
</template>

<script>
  const TICK = 1000
  const TIMEOUT = 60000

  module.exports = {
    name: 'countdown',
    props: ['date'],
    data() {
      return {
        now: new Date().getTime(),
        expires: this.$moment.toTimestamp(this.date) + TIMEOUT
      }
    },
    computed: {
      countdown() {
        return this.$moment.diff(this.expires, this.now)
      }
    },
    watch: {
      date() {
        this.stopCountdown()
        this.now = new Date().getTime(),
        this.expires = this.$moment.toTimestamp(this.date) + TIMEOUT
        this.startCountdown()
      }
    },
    methods: {
      startCountdown() {
        this.interval = setInterval(() => {
          if (this.now < this.expires) {
            this.now = this.now + TICK
          } else {
            this.stopCountdown()
          }
        }, TICK);
      },

      stopCountdown() {
        clearInterval(this.interval)
      }
    },
    mounted() {
      this.$nextTick(function() {
        this.startCountdown()
      })
    },
    destroyed() {
      this.$nextTick(function() {
        this.stopCountdown()
      })
    }
  }
</script>
