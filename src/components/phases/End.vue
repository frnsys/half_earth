<template>
<div class="break">
  <Dialogue v-if="hasDialogue" v-bind="event" @done="show = true" />
  <div class="break--actions" v-if="show">
    <h2>Well Played!</h2>
    <button @click="startRun">Try Again?</button>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import EventsMixin from 'components/EventsMixin';
import {randChoice} from 'lib/util';

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
      show: false,
      events: game.roll.end('Start')
    }
  },
  methods: {
    startRun() {
      game.newRun();
      state.phase = 'PLANNING';
    }
  },
}
</script>
