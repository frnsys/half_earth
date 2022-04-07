<template>
<div class="interstitial" :style="{'background-image': `url('/assets/environments/out/${locale.background}')`}">
  <header>
    <h3>{{state.gameState.world.year}}</h3><br />
    <h1>{{title}}</h1><br />
    <h2>{{locale.name}}</h2>
  </header>
  <div class="interstitial--summary">
    <div>People are {{contentedness}}.</div>
    <div>Biodiversity is {{biodiversity}}.</div>
    <div>The world is {{world}}.</div>
    <div>You have {{yearsLeft}} years left in your tenure.</div>
  </div>
  <Dialogue v-if="hasDialogue" v-bind="event" @done="nextEvent" />
  <div class="interstitial--next" v-if="ready && !gameOver && !gameWin">
    <button class="btn" @click="nextPhase">Continue</button>
  </div>
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

// TODO
const LOCALES = [{
  name: 'Havana',
  background: 'pexels-matthias-oben-3687869.jpg',
  ambience: 'city_noise.mp3',
}];

export default {
  mixins: [EventsMixin],
  mounted() {
    this.start();
  },
  activated() {
    this.start();
  },
  beforeUnmount() {
    this.sound.pause();
  },
  data() {
    let events = game.roll.interstitial('Start');
    return {
      ready: false,
      state,
      events,
    }
  },
  computed: {
    title() {
      let n = Math.round((state.gameState.world.year - state.startYear)/5) + 1;
      let ext = 'th';
      if (n == 1) {
        ext = 'st';
      } else if (n == 2) {
        ext = 'nd';
      } else if (n == 3) {
        ext = 'rd';
      }
      return `The ${n}${ext} Planning Session`;
    },
    locale() {
      // TODO
      return LOCALES[0];
    },
    gameOver() {
      return state.gameState.game_over;
    },
    gameWin() {
      return state.gameState.world.emission <= consts.winState.emissions
        && state.gameState.world.extinction_rate <= consts.winState.extinction
        && state.gameState.world.temperature <= consts.winState.temperature;
    },
    world() {
      let idx = intensity.scale(state.gameState.world.temperature, 'warming');
      if (state.gameState.world.emissions > 0) {
        return 'still warming';
      } else if (state.gameState.world.emissions < 0) {
        return 'recovering';
      } else if (state.gameState.world.temperature >= 2) {
        return 'becoming unbearable';
      } else if (state.gameState.world.temperature >= 3) {
        return 'hostile to life';
      }
    },
    biodiversity() {
      let idx = intensity.scale(state.gameState.world.extinction_rate, 'extinction');
      const descs = [
        'flourishing',
        'recovering',
        'stabilizing',
        'struggling',
        'suffering',
        'plummeting',
      ];
      idx = Math.min(idx, descs.length);
      return descs[idx];
    },
    contentedness() {
      let idx = intensity.scale(state.gameState.world.contentedness, 'world_outlook') - 1;
      const descs = [
        'furious',
        'upset',
        'unhappy',
        'content',
        'happy',
        'ecstatic',
      ];
      idx = Math.min(idx, descs.length);
      return descs[idx];
    },
    yearsLeft() {
      return Math.max(0, state.gameState.death_year - state.gameState.world.year);
    },
  },
  methods: {
    afterEvents() {
      if (this.gameOver || this.gameWin) {
        this.nextPhase();
      } else {
        this.ready = true;
      }
    },
    start() {
      // Wait a beat before showing the event
      setTimeout(() => {
        this.showEvent();
      }, 3500);

      // Hack to get it to crossfade loop
      let sounds = [
        `/assets/environments/ambience/${this.locale.ambience}`,
        `/assets/environments/ambience/${this.locale.ambience}`,
      ];
      this.sound = new Playlist(sounds);
      this.sound.play();
    },
    nextPhase() {
      if (this.gameOver) {
        game.saveMeta();
        animate(1.0, 0.0, 1000, (val) => {
          this.$el.style.opacity = val;
        }, () => {
          state.phase = 'GAMEOVER';
        });
      } else if (this.gameWin) {
        game.saveMeta();
        animate(1.0, 0.0, 1000, (val) => {
          this.$el.style.opacity = val;
        }, () => {
          state.phase = 'GAMEWIN';
        });
      } else {
        state.phase = 'PLANNING';
      }
    }
  }
}
</script>

<style>
.interstitial {
  image-rendering: pixelated;
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  top: 0;
  background-size: cover;
  background-position: center center;
  background-repeat: no-repeat;
}
.interstitial h1,
.interstitial h2,
.interstitial h3 {
  color: #fff;
  font-weight: normal;
  display: inline-block;
  background: #222222db;
  margin: 0.25em auto;
  box-shadow: 0 0 8px 8px #222222db;
  border-radius: 0.2em;
}
.interstitial h3 {
  font-size: 0.8em;
}
.interstitial h2 {
  font-size: 1.1em;
  font-style: italic;
}
.interstitial header {
  margin: 2em 0;
  text-align: center;
}
.interstitial--summary {
  text-align: center;
  background: #202020db;
  color: #fff;
  max-width: 480px;
  margin: 0 auto 1em auto;
  padding: 0.5em 1em;
  border-radius: 0.2em;
  box-shadow: 0 0 8px 8px #202020db;
  animation: fade-in 1.0s;
}
.interstitial--summary > div {
  margin: 1em 0;
}
.interstitial--summary > div:last-child {
  border-top: 1px solid #383838;
  padding-top: 1em;
  font-size: 0.85em;
}
.interstitial--next {
  position: fixed;
  bottom: 0.5em;
  left: 0;
  right: 0;
}
</style>
