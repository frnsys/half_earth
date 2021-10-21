<template>
<Hud />
<div id="event-stream">
  <div id="event-stream--year">
    {{year}}
    <div id="event-stream-timer-fill" :style="{width: `${progress}%`}"></div>
  </div>
  <Globe id="stream-globe" ref="globe" />
  <Event v-if="event" :event="event" @done="nextEvent" @select="selectChoice" />
  <div id="event-stream--toasts">
    <div class="toast" v-for="toast, i in toasts" :style="{opacity: (i+1)/(toasts.length+1)}">
      <img :src="`/assets/icons/${toast.icon}.png`"> {{toast.desc}}
    </div>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import {sign} from 'lib/util';
import Event from './Event.vue';
import Hud from 'components/Hud.vue';
import Globe from 'components/Globe.vue'
import EventsMixin from 'components/EventsMixin';
import regionsToTiles from '/assets/surface/regions_to_tiles.json';
import iconEvents from '/assets/content/icon_events.json';

const MS_PER_YEAR = 6000;

function randChoice(arr) {
  return arr[Math.floor(Math.random() * arr.length)];
}

export default {
  mixins: [EventsMixin],
  data() {
    return {
      time: 0,
      toasts: [],
      year: state.gameState.world.year,
    };
  },
  components: {
    Hud,
    Globe,
    Event,
  },
  mounted() {
    this.start();
  },
  activated() {
    this.start();
  },
  computed: {
    progress() {
      return this.time/MS_PER_YEAR * 100;
    }
  },
  methods: {
    start() {
      // Cache starting values for report
      this._startYear = state.gameState.world.year;
      state.cycleStartState = {
        year: this._startYear,
        extinctionRate: state.gameState.world.extinction_rate,
        contentedness: state.gameState.contentedness,
        temperature: state.gameState.world.temperature,
      };

      if (!this.globe) {
        this.$refs.globe.onReady = (globe) => {
          this.globe = globe;
          this.startYear();
        };
      } else {
        this.startYear();
      }
    },
    startYear() {
      this.time = 0;
      let last = performance.now();
      let iconEvents = game.rollIconEvents();
      const tick = (timestamp) => {
        let elapsed = timestamp - last;
        this.time += elapsed;

        if (this.time >= MS_PER_YEAR) {
          game.step();
          this.year = state.gameState.world.year;
          this.rollEvent();
          return;

        } else {
          // TODO need to ensure all events play out before end of year
          if (iconEvents.length > 0 && Math.random() < 0.05) {
            let [eventId, regionId] = iconEvents.shift();
            game.applyEvent(eventId, regionId);
            this.showEventOnGlobe(eventId, regionId);
          }
          last = timestamp;
        }
        requestAnimationFrame(tick);
      };
      requestAnimationFrame(tick);
    },
    rollEvent() {
      // Go to report phase
      if (state.gameState.world.year > this._startYear
        && state.gameState.world.year % 5 == 0) {
        state.phase = 'REPORT';
        return;
      }

      this.events = game.rollWorldEvents();
      this.applyEmissions();

      if (this.hasEvent) {
        this.showEvent();
      } else {
        this.startYear();
      }
    },
    afterEvents() {
      this.startYear();
    },
    applyEmissions() {
      let world = state.gameState.world;
      let emissions = {
        // Hector separates out FFI and LUC emissions
        // but we lump them together
        // Units: <https://github.com/JGCRI/hector/wiki/Hector-Units>
        'ffi_emissions': world.co2_emissions * 12/44 * 1e-15, // Pg C/y
        'CH4_emissions': world.ch4_emissions * 1e-12, // Tg/y
        'N2O_emissions': world.n2o_emissions * 1e-12, // Tg/y
      };
      this.globe.addEmissionsThenUpdate({}).then((tgav) => {
        game.setTgav(tgav);
      });
    },
    showEventOnGlobe(eventId, regionId) {
      let ev = iconEvents[eventId];
      if (this.globe && regionId) {
        // TODO distinguish inland vs coastal events
        let region = state.gameState.world.regions[regionId];
        let tiles = regionsToTiles[region.name];
        let tileIdx = randChoice(tiles.inland.concat(tiles.coasts));
        let label = sign(ev.outlookChange);
        this.globe.showIconText(ev.icon, label, tileIdx);
        this.toasts.push({
          icon: ev.icon,
          desc: `${ev.name} in ${region.name}`
        });
        if (this.toasts.length > 3) {
          this.toasts.shift();
        }
      }
    },
  },
}
</script>

<style>
#stream-globe {
  position: absolute;
  left: 0;
  top: 0;
  right: 0;
  bottom: 0;
}

#event-stream-timer-fill {
  height: 2px;
  background: red;
}
#event-stream--year {
  position: absolute;
  left: 0;
  right: 0;
  text-align: center;
  z-index: 2;
  font-size: 1.5em;
  padding: 0.4em;
  font-family: "Andada Pro";
}

#event-stream--toasts {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  padding: 1em;
  text-align: center;
  font-size: 0.8em;
}
.toast {
  padding: 0.1em 0.25em;
  border-radius: 0.2em;
  background: rgba(20,20,20,0.9);
  color: #fff;
  border: 1px solid black;
  text-align: center;
  margin: 0.15em 0;
  display: inline-block;
  line-height: 1.7;
}
.toast img {
  height: 20px;
  vertical-align: middle;
}
</style>
