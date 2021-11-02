<template>
<Hud />
<div id="event-stream">
  <div id="event-stream--year">
    {{year}}
    <div id="event-stream-timer-fill" :style="{width: `${progress}%`}"></div>
  </div>
  <Globe id="events-globe" ref="globe" />
  <Project v-if="completedProjects.length > 0" :id="completedProjects[0]" @click="() => completedProjects.shift()"/>
  <Dialogue v-if="event && predialogue" :dialogue="event.dialogue" @done="nextEvent" @select="selectChoice" />
  <Event v-else-if="event && !predialogue" :event="event" @done="nextEvent" @select="selectChoice" />
  <div id="event-stream--toasts">
    <div class="toast" v-for="toast, i in toasts" :style="{opacity: (i+1)/(toasts.length+1)}">
      <div class="toast--body"><img :src="`/assets/icons/pips/${toast.icon}.png`"> {{toast.desc}}</div>
    </div>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import {sign} from 'lib/util';
import Event from './Event.vue';
import Project from './Project.vue';
import Hud from 'components/Hud.vue';
import Globe from 'components/Globe.vue'
import EventsMixin from 'components/EventsMixin';
import regionsToTiles from '/assets/surface/regions_to_tiles.json';
import iconEvents from '/assets/content/icon_events.json';

const MS_PER_YEAR = 10000;

function randChoice(arr) {
  return arr[Math.floor(Math.random() * arr.length)];
}

export default {
  mixins: [EventsMixin],
  data() {
    let events = game.rollWorldStartEvents();
    return {
      events,
      time: 0,
      toasts: [],
      predialogue: true,
      year: state.gameState.world.year,
      completedProjects: [],
      stopped: false
    };
  },
  components: {
    Hud,
    Globe,
    Event,
    Project,
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
      // Show any world start events
      this.stopped = false;
      if (this.hasEvent) {
        this.predialogue = true;
        console.log(this.events);
        this.showEvent();
      } else {
        this.predialogue = false;
      }

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
      console.log('ICON EVENTS:');
      console.log(iconEvents);
      const tick = (timestamp) => {
        if (this.stopped) return;
        let elapsed = timestamp - last;
        last = timestamp;

        if (!this.showingEvent) {
          this.time += elapsed;

          if (this.time >= MS_PER_YEAR) {
            this.completedProjects = game.step();
            this.year = state.gameState.world.year;

            this.rollEvent();
            return;

          } else {
            // TODO need to ensure all events play out before end of year
            /* if (iconEvents.length > 0 && Math.random() < 0.05) { */
            if (iconEvents.length > 0 && Math.random() < 1) {
              let [eventId, regionId] = iconEvents.shift();
              game.applyEvent(eventId, regionId);
              let icon = this.showEventOnGlobe(eventId, regionId);

              // If autoclickers for this event, roll for autoclick
              if (icon && eventId in state.gameState.autoclickers) {
                let chance = state.gameState.autoclickers[eventId];
                setTimeout(() => {
                  if (Math.random() <= chance) {
                    this.globe.respondToEvent(icon.mesh, icon.hexIdx, icon.mesh.userData);
                  }
                }, 100);
              }
            }
          }
        }
        requestAnimationFrame(tick);
      };
      requestAnimationFrame(tick);
    },
    rollEvent() {
      // Go to report phase
      if (state.gameState.world.year > this._startYear
        && state.gameState.world.year % 5 == 0) {
        console.log(`Stopping on year: ${state.gameState.world.year}`);
        this.stopped = true;
        state.phase = 'REPORT';
        return;
      }

      this.events = game.rollWorldEvents();
      console.log('Rolled world events:');
      console.log(this.events);
      this.applyEmissions();

      if (this.hasEvent) {
        this.showEvent();
      } else {
        this.startYear();
      }
    },
    afterEvents() {
      this.predialogue = false;
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
      console.log('Applying emissions:');
      console.log(emissions);
      this.globe.addEmissionsThenUpdate(emissions).then((tgav) => {
        game.setTgav(tgav);
      });
    },
    showEventOnGlobe(eventId, regionId) {
      let ev = iconEvents[eventId];
      if (this.globe && regionId !== undefined && regionId !== null) {
        // TODO distinguish inland vs coastal events
        let region = state.gameState.world.regions[regionId];
        let tiles = regionsToTiles[region.name];
        let hexIdx = randChoice(tiles.inland.concat(tiles.coasts));
        // let label = sign(ev.effect.value);
        let mesh = this.globe.showIcon(ev.icon, hexIdx, {
          event: ev,
          region,
        });

        let outlook = ev.intensity + 1;
        game.changeLocalOutlook(-outlook, regionId);
        this.globe.pingIcon('discontent', hexIdx);
        let outlookInterval = setInterval(() => {
          if (outlook <= 0) {
            clearInterval(outlookInterval);
          } else {
            outlook--;
            this.globe.pingIcon('discontent', hexIdx);
          }
        }, 250);

        this.toasts.push({
          icon: ev.icon,
          desc: `${ev.name} in ${region.name}`
        });
        if (this.toasts.length > 3) {
          this.toasts.shift();
        }
        return {hexIdx, mesh};
      }
    },
  },
}
</script>

<style>
#events-globe {
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
.toast--body {
  display: inline-block;
  padding: 0.1em 0.25em;
  border-radius: 0.2em;
  background: rgba(20,20,20,0.9);
  color: #fff;
  border: 1px solid black;
  text-align: center;
  margin: 0.15em 0;
  line-height: 1.7;
}
.toast img {
  height: 20px;
  vertical-align: middle;
}

#event-stream .dialogue {
  background: rgba(255,255,255,0.25);
}
</style>
