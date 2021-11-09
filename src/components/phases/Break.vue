<template>
<Dialogue v-if="event" :dialogue="event.dialogue" @done="nextEvent" @select="selectChoice" />
<div class="break">
  <h1>this takes place between runs</h1>
  <button @click="nextPhase">New Run</button>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import EventsMixin from 'components/EventsMixin';

export default {
  mixins: [EventsMixin],
  mounted() {
    this.showEvent();
  },
  activated() {
    this.showEvent();
  },
  data() {
    return {
      events: game.rollBreaksEvents()
    }
  },
  methods: {
    startRun() {
      game.newRun();
      state.phase = 'PLANNING';
    }
  }
}
</script>

<style>
</style>
