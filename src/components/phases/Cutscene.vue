<template>
<div class="cutscene" :style="{'background-image': `url('/assets/cutscenes/out/${image}')`}">
  <Dialogue v-if="hasDialogue" v-bind="event" @done="nextEvent" @advanced="advanced" />
  <button class="cutscene--skip btn" @click="nextPhase">Skip</button>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
import animate from '/src/lib/anim';
import Playlist from '/src/lib/playlist';
import EventsMixin from 'components/EventsMixin';
import intensity from '/src/display/intensity';

// One per line of dialogue
const IMAGES = [
  'pexels-lt-chan-2833366.jpg',
  'gosplant_world.jpg',
  'gosplant_world.jpg',
  'gosplant_world.jpg',
  'pexels-marco-allasio-4275996.jpg',
  'pexels-mentatdgt-1185433.jpg',
  'hasan-almasi-OwqLxCvoVxI-unsplash.jpg',
  'matthew-tenbruggencate-0HJWobhGhJs-unsplash.jpg',
  'hasan-almasi-OwqLxCvoVxI-unsplash.jpg',
  'kelly-sikkema-VpcSDucAYjw-unsplash.jpg'
];

export default {
  mixins: [EventsMixin],
  mounted() {
    this.start();
  },
  activated() {
    this.start();
  },
  data() {
    let events = game.roll.cutscene('Intro');
    return {
      idx: 0,
      ready: false,
      state,
      events,
    }
  },
  computed: {
    image() {
      return IMAGES[this.idx];
    }
  },
  methods: {
    advanced() {
      this.idx++;
    },
    afterEvents() {
      this.nextPhase();
    },
    start() {
      // Wait a beat before showing the event
      setTimeout(() => {
        this.showEvent();
      }, 1500);
    },
    nextPhase() {
      animate(1.0, 0.0, 1000, (val) => {
        this.$el.style.opacity = val;
      }, () => {
        state.phase = 'INTERSTITIAL';
      });
    }
  }
}
</script>

<style>
.cutscene {
  image-rendering: pixelated;
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  top: 0;
  background-size: cover;
  background-position: center center;
  background-repeat: no-repeat;
  animation: fade-in 0.5s;
}

.win-con, .lose-con {
  background: #111;
  color: #fff;
  border-radius: 0.2em;
  word-break: keep-all;
  white-space: nowrap;
  box-shadow: 0 0 6px 3px #111;
}

.cutscene--skip {
  position: fixed;
  right: 1em;
  bottom: 0.5em;
  z-index: 1000;
  font-size: 0.8em;
}
</style>

