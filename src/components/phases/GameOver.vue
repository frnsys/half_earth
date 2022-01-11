<template>
<div class="break">
  <Dialogue v-if="hasDialogue" v-bind="event" @done="showStart = true" />
  <div class="break--actions" v-if="showStart">
    <h2>{{message}}</h2>
    <button @click="startRun">Try Again</button>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import EventsMixin from 'components/EventsMixin';
import {randChoice} from 'lib/util';

const MESSAGES = [
  'The world can still be salvaged...',
  'This is not the end...',
];

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
      showStart: false,
      events: game.roll.break('Start')
    }
  },
  methods: {
    startRun() {
      game.newRun();
      state.phase = 'PLANNING';
    }
  },
  computed: {
    message() {
      return randChoice(MESSAGES);
    }
  }
}
</script>

<style>
.break {
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  height: 100vh;
}
.break h2 {
  margin-top: 0;
  font-family: 'Andada Pro';
  text-transform: uppercase;
  font-style: italic;
}

.break--actions {
  color: #fff;
  text-align: center;
  margin: 2em;
}

.break .dialogue {
  background: none;
}
</style>
