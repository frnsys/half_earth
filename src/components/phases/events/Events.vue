<template>
<Hud />
<div id="event-stream">
  <div id="event-stream--year">
    {{year}}
    <div id="event-stream-timer-fill" :style="{width: `${progress}%`}"></div>
  </div>
  <Globe id="events-globe" ref="globe" :onReady="onGlobeReady" />
  <Project v-if="completedProjects.length > 0" :id="completedProjects[0]" @click="dismissProject"/>
  <Dialogue v-if="event && predialogue" v-bind="event" @done="nextEvent" />
  <Event v-else-if="event && !predialogue && completedProjects.length == 0" :event="event" @done="nextEvent" />
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
import Event from './Event.vue';
import Project from './Project.vue';
import Hud from 'components/Hud.vue';
import Globe from 'components/Globe.vue'
import EventsMixin from 'components/EventsMixin';
import regionsToTiles from '/assets/surface/regions_to_tiles.json';
import ICON_EVENTS from '/assets/content/icon_events.json';
import {sign, randChoice} from 'lib/util';
import {sendSnapshot} from '/src/log';

const MS_PER_YEAR = 2000;

function popIconEvents(arr, time) {
  let results = [];
  for (let i = arr.length - 1; i >= 0; i--) {
    if (arr[i].when <= time) {
      results.push(arr.splice(i, 1)[0]);
    }
  }
  return results;
}

export default {
  mixins: [EventsMixin],
  data() {
    let events = game.roll.world('Start');
    return {
      events,
      toasts: [],
      time: 0,
      predialogue: true,
      year: state.gameState.world.year,
      completedProjects: [],
      stopped: false,
      done: false
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
    onGlobeReady(globe) {
      this.globe = globe;
      this.globe.clear();
      this.globe.rotate = true;
      this.globe.clouds.visible = true;
      this.startYear();
    },
    start() {
      // Show any world start events
      this.stopped = false;
      if (this.hasEvent) {
        this.predialogue = true;
        this.showEvent();
      } else {
        this.predialogue = false;
      }

      // Cache starting values for report
      this._startYear = state.gameState.world.year;
      state.annualRegionEvents = {};
      state.cycleStartState = {
        year: this._startYear,
        extinctionRate: state.gameState.world.extinction_rate,
        contentedness: state.gameState.world.contentedness,
        temperature: state.gameState.world.temperature,
        emissions: state.gameState.world.emissions,
        completedProjects: [],
        regionIncomes: state.gameState.world.regions.map((r) => r.income),
        parliament: state.gameState.npcs.map((npc) => npc.seats),
      };
    },
    startYear() {
      sendSnapshot(state);

      this.time = 0;
      this.globe.resumeRotation();
      let last = performance.now();
      let iconEvents = game.roll.icon()
        .map(([eventId, regionId]) => {
          return {
            eventId,
            regionId,

            // When in the year the event occurs
            when: Math.random() * MS_PER_YEAR
          }
        });

      const tick = (timestamp) => {
        let elapsed = timestamp - last;
        last = timestamp;

        if (!this.stopped) {;
          if (!this.showingEvent) {
            this.time += elapsed;

            if (this.time >= MS_PER_YEAR) {
              this.completedProjects = game.step();
              if (this.completedProjects.length > 0) {
                this.stopped = true;
                state.cycleStartState.completedProjects = state.cycleStartState.completedProjects.concat(this.completedProjects);
              }
              this.year = state.gameState.world.year;

              // Add to historical data
              state.history.emissions.push(state.gameState.world.emissions);
              state.history.land_use.push(state.gameState.resources_demand.land);

              this.rollEvent();
              return;

            } else {
              if (iconEvents.length > 0) {
                let events = [];
                popIconEvents(iconEvents, this.time).forEach(({eventId, regionId}) => {
                  events.push({eventId, regionId});
                  let icon = this.showEventOnGlobe(eventId, regionId);
                  let ev = ICON_EVENTS[eventId];

                  // If autoclickers for this event, roll for autoclick
                  // if (icon && eventId in state.gameState.autoclickers) {
                  //   let chance = state.gameState.autoclickers[eventId];
                  //   setTimeout(() => {
                  //     if (Math.random() <= chance) {
                  //       this.globe.respondToEvent(icon.mesh, icon.hexIdx, icon.mesh.userData);
                  //     }
                  //   }, 100);
                  // }
                });
                game.applyIconEvents(events);
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
        this.stopped = true;
        this.done = true;
        if (this.completedProjects.length == 0) {
          game.stepCycle();
          state.phase = 'REPORT';
        }
        return;
      }

      this.events = game.roll.world('Main');
      this.events.forEach((ev) => {
        state.events.push(ev);
      });
      this.applyEmissions();

      if (this.hasEvent) {
        this.showEvent();
        this.globe.pauseRotation();
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
      this.globe.addEmissionsThenUpdate(emissions).then((tgav) => {
        game.setTgav(tgav);
      });
    },
    dismissProject() {
      this.completedProjects.shift();
      if (this.completedProjects.length == 0 && this.done) {
        state.phase = 'REPORT';
      } else {
        this.stopped = this.completedProjects.length > 0;
      }
    },
    showEventOnGlobe(eventId, regionId) {
      let ev = ICON_EVENTS[eventId];
      if (this.globe && regionId !== undefined && regionId !== null) {
        let history = state.annualRegionEvents[regionId] || [];
        history.push(ev);
        state.annualRegionEvents[regionId] = history;

        // TODO distinguish inland vs coastal events
        let region = state.gameState.world.regions[regionId];
        let tiles = regionsToTiles[region.name];
        let hexIdx = randChoice(tiles.inland.concat(tiles.coasts));
        // let label = sign(ev.effect.value);
        let mesh = this.globe.show({
          icon: ev.icon,
          hexIdx
        });
          /* event: ev, */
          /* region, */
        /* }); */

        // let outlook = ev.intensity * 0.05;
        let outlook = ev.intensity * 0.1;
        let pings = ev.intensity;
        // game.changeLocalOutlook(-outlook, regionId);
        game.changeHabitability(-outlook, regionId);
        let args = {icon: 'discontent', hexIdx, ping: true, iconSize: 0.35};
        this.globe.show(args);
        if (pings > 1) {
          let outlookInterval = setInterval(() => {
            if (pings <= 0) {
              clearInterval(outlookInterval);
            } else {
              pings--;
              this.globe.show(args);
            }
          }, 250);
        }

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
  background: #fadbae;
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
}

#event-stream .dialogue {
  padding: 0;
}


#event-stream--toasts {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 1.25em;
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
</style>
