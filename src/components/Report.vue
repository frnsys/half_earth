<template>
  <Hud />
  <h2>REPORT</h2>
  <h2>(TODO)</h2>
  <button @click="nextPhase">Next</button>
</template>

<script>
import state from '../state';
import Hud from './Hud.vue';

export default {
  components: {
    Hud
  },
  methods: {
    nextPhase() {
        state.phase = 'PLANNING';
    }
  }
}
</script>

<style scoped>
h2 {
  color: #fff;
}
</style>
