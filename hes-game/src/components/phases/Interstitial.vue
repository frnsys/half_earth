<template>
<div class="interstitial" :style="{'background-image': `url('/assets/environments/out/${locale.background}')`}">
  <div class="interstitial--inner">
  <header>
    <h3>{{state.gameState.world.year}}</h3><br />
    <h1>{{title}}</h1><br />
    <h2>{{t(locale.name)}}</h2>
  </header>
  <div class="interstitial--summary">
    <div>{{t(`People are ${contentedness}`)}}.</div>
    <div>{{t(`Biodiversity is ${biodiversity}`)}}.</div>
    <div>{{t(`The world is ${world}`)}}.</div>
    <div>{{t(`Parliament ${parliament}`)}}.</div>
    <div>{{t('You have {yearsLeft} years left in your tenure.', {yearsLeft})}}</div>
  </div>
  <Dialogue v-if="hasDialogue" v-bind="event" @done="nextEvent" />
  <div class="interstitial--image-credit">{{t('Image:')}} {{locale.credit}}</div>
  <div class="interstitial--next" v-if="ready">
    <button class="btn" @click="nextPhase">{{t('Continue')}}</button>
  </div>
  </div>
</div>
</template>

<script>
import t from '/src/i18n';
import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
import animate from '/src/lib/anim';
import EventsMixin from 'components/EventsMixin';
import intensity from '/src/display/intensity';

// List from Troy:
// Bandung, Hanoi, Mexico City, Budapest, Thiruvananthapuram, Luanda, Ayn Issa, Ferrara, Vienna, Beijing, Aden, Caracas, Algiers, Belgrade, Moscow, Managua, Buenos Aires, Trier, Prague, Porto Alegre, Seattle/Burlington/Bronx, Dar es Salaam

const LOCALES = [{
  name: 'Havana',
  background: 'pexels-matthias-oben-3687869.jpg',
  ambience: 'city_noise.mp3',
  credit: 'Matthias Oben',
}, {
  name: 'Ouagadougou',
  background: '2560px-Ouagadougou_BCEAO_day.jpg',
  ambience: 'city_noise.mp3',
  credit: 'Wegmann, CC BY-SA 3.0, via Wikimedia Commons',
}, {
  name: 'Port-au-Prince',
  background: 'robin-canfield-CkCV7vTmmz4-unsplash.jpg',
  ambience: 'city_noise.mp3',
  credit: 'Robin Canfield',
}, {
  name: 'San Cristóbal de las Casas',
  background: '1536px-Street_Scene_with_Church_Cupola_-_San_Cristobal_de_las_Casas_-_Chiapas_-_Mexico.jpg',
  ambience: 'city_noise.mp3',
  credit: 'Adam Jones, CC BY 2.0, via Wikimedia Commons',
}, {
  name: 'Paris',
  background: 'pexels-pierre-blache-3073666.jpg',
  ambience: 'city_noise.mp3',
  credit: 'Pierre Blaché',
}, {
  name: 'Bandung',
  background: 'Street_Braga,_Bandung_City,_West_Java,_Indonesia.jpg',
  ambience: 'city_noise.mp3',
  credit: 'PACARNYAKEYES, CC BY-SA 4.0, via Wikimedia Commons',
}, {
  name: 'Seattle',
  background: '2560px-Seattle_4.jpg',
  ambience: 'city_noise.mp3',
  credit: 'Daniel Schwen, CC BY-SA 4.0, via Wikimedia Commons',
}, {
  name: 'Hanoi',
  background: '2560px-Vietnam,_Hanoi,_Streets_of_central_Hanoi_2.jpg',
  ambience: 'city_noise.mp3',
  credit: '© Vyacheslav Argenberg / http://www.vascoplanet.com/, CC BY 4.0, via Wikimedia Commons',
}, {
  name: 'Dar es Salaam',
  background: 'Dar_es_Salaam_before_dusk.jpg',
  ambience: 'city_noise.mp3',
  credit: 'Muhammad Mahdi Karim, GFDL 1.2, via Wikimedia Commons',
}, {
  name: 'Ayn Issa',
  background: '2560px-Another_Year_Without_Daesh.jpg',
  ambience: 'city_noise.mp3',
  credit: 'Combined Joint Task Force - Operation Inherent Resolve/Sgt. Raymond Boyington, Public domain, via Wikimedia Commons',
}, {
  name: 'Algiers',
  background: '2560px-Martyrs_Memorial,_A_cloudy_day_in_Algiers.jpg',
  ambience: 'city_noise.mp3',
  credit: 'EL Hacene Boulkroune, CC BY-SA 4.0, via Wikimedia Commons',
}, {
  name: 'Managua',
  background: 'Old_Managua_Cathedral_from_Highway_2.jpg',
  ambience: 'city_noise.mp3',
  credit: 'Byralaal, CC BY-SA 4.0, via Wikimedia Commons',
}, {
  name: 'Prague',
  background: '2560px-Vltava_river_in_Prague.jpg',
  ambience: 'city_noise.mp3',
  credit: 'Dmitry A. Mottl, CC BY-SA 4.0, via Wikimedia Commons',
}, {
  name: 'Havana',
  background: 'pexels-matthias-oben-3687869.jpg',
  ambience: 'city_noise.mp3',
  credit: 'Matthias Oben',
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
    window.audioManager.stopAtmosphere(true);
  },
  data() {
    let events = game.roll.interstitial('Start');
    if (game.gameWon()) {
      events = events.concat(game.roll.interstitial('Win'));
    }
    return {
      ready: false,
      state,
      events,
    }
  },
  computed: {
    number() {
      return Math.round((state.gameState.world.year - state.startYear)/5) + 1;
    },
    title() {
      let n = this.number;
      let ext = 'th';
      if (n == 1) {
        ext = 'st';
      } else if (n == 2) {
        ext = 'nd';
      } else if (n == 3) {
        ext = 'rd';
      }
      return t(`The {n}{ext} Planning Session`, {n, ext: t(ext)});
    },
    locale() {
      let idx = this.number - 1 % LOCALES.length;
      return LOCALES[idx];
    },
    gameOver() {
      return state.gameState.game_over;
    },
    gameWin() {
      return game.gameWon();
    },
    parliament() {
      if (state.gameState.political_capital <= 20) {
        return 'is conspiring against you';
      } else if (state.gameState.political_capital <= 200) {
        return 'is ready to work with you';
      } else {
        return 'trusts you completely';
      }
    },
    world() {
      let idx = intensity.scale(state.gameState.world.temperature, 'warming');
      if (state.gameState.world.emissions > 0) {
        return 'still warming';
      } else if (state.gameState.world.emissions <= 0) {
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
      idx = Math.max(0, Math.min(idx, descs.length - 1));
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
      idx = Math.max(0, Math.min(idx, descs.length - 1));
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
        this.ready = true;
        this.showEvent();
      }, 1200);
      window.audioManager.stopSoundtrack(true);
      window.audioManager.startAtmosphere(`/assets/environments/ambience/${this.locale.ambience}`, true)
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
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.interstitial--inner{
  padding-bottom: 1rem;
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
  margin: 1em 0 2em 0;
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
  animation: fade-in 0.75s;
}

.interstitial--image-credit {
  text-align: left;
  position: absolute;
  left: 0.5em;
  bottom: 0.5em;
  color: #fff;
  font-size: 0.7em;
  max-width: 200px;
  text-shadow: 0 0 2px black;
}

.btn{
  font-family: 'Times Ten', serif;
}

</style>
