<template>
  <Hud />
  <h2>REPORT</h2>
  <h2>(TODO)</h2>
  <h2 v-if="lost">you lost</h2>
  <button @click="nextPhase">Next</button>
</template>

<script>
import state from '../state';
import Hud from './Hud.vue';

export default {
  components: {
    Hud
  },
  data() {
    return {
      lost: state.gameState.political_capital <= 0,
    }
  },
  methods: {
    nextPhase() {
      if (this.lost) {
        state.phase = 'PLANNING';
      } else {
        state.phase = 'BREAK';
      }
    }
  }
}
</script>

<style scoped>
h2 {
  color: #fff;
}
</style>
