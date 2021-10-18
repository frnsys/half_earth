<template>
<Interstitial v-if="event" :dialogue="event.dialogue" @done="nextEvent" @select="selectChoice" />
<div class="break">
  <h1>this takes place between runs</h1>
  <button @click="nextPhase">New Run</button>
</div>
</template>

<script>
import game from '../game';
import state from '../state';
import EventsMixin from './EventsMixin';

export default {
  mixins: [EventsMixin],
  data() {
    let events = game.rollBreaksEvents();
    return {
      events,
      eventIdx: events.length > 0 ? 0 : null
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
