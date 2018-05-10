<template>
  <span v-if="countdown > 0">in a {{ countdown }} seconds</span>
  <span v-else>{{ text }}</span>
</template>

<script>
  const TICK = 1000

  module.exports = {
    name: 'countdown',
    props: ['from', 'timeout', 'text'],
    data() {
      return {
        now: new Date().getTime(),
        expires: this.from + this.timeout * 1000
      }
    },
    computed: {
      countdown() {
        return this.$moment.diff(this.expires, this.now)
      }
    },
    watch: {
      from() {
        this.stopCountdown()
        this.now = new Date().getTime(),
        this.expires = this.from + this.timeout
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
