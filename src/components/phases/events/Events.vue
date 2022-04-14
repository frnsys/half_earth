<template>
<Hud />
<div id="event-stream">
  <div id="event-stream--year">
    {{state.gameState.world.year}}
    <div id="event-stream-timer-fill" :style="{width: `${progress}%`}"></div>
  </div>
  <Globe id="events-globe" ref="globe" :onReady="onGlobeReady" :style="{'background-color': warmingColour}" />
  <Update v-if="updates.length > 0 && !this.skipping" :update="updates[0]" @done="dismissUpdate"/>
  <Dialogue v-if="event && predialogue" v-bind="event" @done="nextEvent" />
  <Event v-else-if="event && !predialogue && updates.length == 0" :event="event" @done="nextEvent" />
  <div id="event-stream--toasts">
    <div class="toast" v-for="toast, i in toasts" :style="{opacity: (i+1)/(toasts.length+1)}">
      <div class="toast--body"><img :src="`/assets/icons/pips/${toast.icon}.png`"> {{toast.desc}}</div>
    </div>
  </div>
  <button class="events--skip btn" @click="skip">Skip</button>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
import Event from './Event.vue';
import Update from './Update.vue';
import Hud from 'components/Hud.vue';
import Globe from 'components/Globe.vue'
import EventsMixin from 'components/EventsMixin';
import regionsToTiles from '/assets/surface/regions_to_tiles.json';
import EVENTS from '/assets/content/events.json';
import ICON_EVENTS from '/assets/content/icon_events.json';
import {sign, randChoice} from 'lib/util';
import {sendSnapshot} from '/src/log';

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
      state,
      events,
      toasts: [],
      time: 0,
      predialogue: true,
      updates: [],
      stopped: false,
      done: false,
      skipping: false,
    };
  },
  components: {
    Hud,
    Globe,
    Event,
    Update,
  },
  mounted() {
    this.start();
  },
  activated() {
    this.start();
  },
  computed: {
    progress() {
      return this.time/this.msPerYear * 100;
    },
    msPerYear() {
      return this.skipping ? 10 : consts.msPerYear;
    },
    warmingColour(){
      if(state.cycleStartState){
      var temp = state.cycleStartState.temperature;
      if(temp <= 0) temp = 0.1;
      var r = 250;
      var g = Math.round(255/temp);
      var b = Math.round(230/temp);

      if(g >= 255) g = 255; r = 240;
      if(b >= 255) b = 255; r = 240;

      return 'rgb(' + r + ',' + g + ',' + b + ')';

      }
      else{
        return '#fadbae';
      }
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
      state.worldEvents = [];
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
            when: Math.random() * this.msPerYear
          }
        });

      const tick = (timestamp) => {
        let elapsed = timestamp - last;
        last = timestamp;

        if (!this.stopped) {;
          if (!this.showingEvent) {
            this.time += elapsed;

            if (this.time >= this.msPerYear) {
              this.year = state.gameState.world.year;
              let {completedProjects, regionChanges} = game.step();
              this.updates = completedProjects.map((id) => ({
                id, type: 'Project',
              })).concat(regionChanges[0].map((id) => ({
                id, type: 'Region:Up',
              }))).concat(regionChanges[1].map((id) => ({
                id, type: 'Region:Down',
              })));
              if ((this.year + 1) % 5 == 0) {
                let policyOutcomes = game.rollNewPolicyOutcomes().map((id) => ({
                  id, type: 'Policy',
                }));
                this.updates = this.updates.concat(policyOutcomes);
              }
              if (this.updates.length > 0 && !this.skipping) {
                this.stopped = true;
              }
              if (completedProjects.length > 0) {
                state.cycleStartState.completedProjects = state.cycleStartState.completedProjects.concat(completedProjects);
              }

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
        if (this.updates.length == 0 || this.skipping) {
          game.stepCycle();
          state.phase = 'REPORT';
        }
        return;
      }

      this.events = game.roll.world('Main');
      this.events.forEach(([evId, regionId]) => {
        let ev = EVENTS[evId];
        state.events.push([evId, regionId, ev['ref_id']]);
        state.worldEvents.push(evId);
      });
      this.applyEmissions();

      if (this.hasEvent && !this.skipping) {
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

      // Set an upper cap to the amount of emissions we pass to hector,
      // because very large numbers end up breaking it.
      let emissions_factor = 1.0;
      if (state.gameState.world.emissions !== 0) {
        emissions_factor = Math.min(1.0, consts.maxEmissions/Math.abs(state.gameState.world.emissions));
      }

      let emissions = {
        // Hector separates out FFI and LUC emissions
        // but we lump them together
        // Units: <https://github.com/JGCRI/hector/wiki/Hector-Units>
        'ffi_emissions': world.co2_emissions * 12/44 * 1e-15 * emissions_factor, // Pg C/y
        'CH4_emissions': world.ch4_emissions * 1e-12 * emissions_factor, // Tg/y
        'N2O_emissions': world.n2o_emissions * 1e-12 * emissions_factor, // Tg/y
      };
      this.globe.addEmissionsThenUpdate(emissions).then((tgav) => {
        game.setTgav(tgav);
      });
    },
    dismissUpdate() {
      this.updates.shift();
      if (!this.updates.length > 0 && this.done) {
        state.phase = 'REPORT';
      } else {
        this.stopped = this.updates.length > 0;
      }
    },
    showEventOnGlobe(eventId, regionId) {
      let ev = ICON_EVENTS[eventId];
      if (this.globe && regionId !== undefined && regionId !== null) {
        let history = state.annualRegionEvents[regionId] || [];
        history.push(ev);
        state.annualRegionEvents[regionId] = history;

        let region = state.gameState.world.regions[regionId];
        let tiles = regionsToTiles[region.name];
        let cands = ev.name == 'Severe Hurricane' ? tiles.coasts : tiles.inland.concat(tiles.coasts);
        let hexIdx = randChoice(cands);
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
    skip() {
      this.skipping = true;
    }
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
  /* background: #fadbae; */

  background-image: url('/assets/backgrounds/globe.png');
  background-blend-mode: multiply;
  background-size: cover;
  background-repeat: no-repeat;
  background-position: center center;
  image-rendering: pixelated;
}

#event-stream-timer-fill {
  height: 4px;
  /* background: rgba(0,0,0,0.6); */
  background-color: #fff;
  border-radius: 2px;
}
#event-stream--year {
  position: absolute;
  left: 0;
  right: 0;
  text-align: center;
  z-index: 2;
  font-size: 1.5em;
  padding: 0.4em;
  bottom:0.5em;
}

#event-stream .dialogue {
  padding: 0;
}


#event-stream--toasts {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 4em;
  padding: 1em;
  text-align: center;
  font-size: 0.8em;
  z-index: 0;
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

.events--skip {
  position: fixed;
  right: 1em;
  bottom: 0.5em;
  z-index: 9;
  font-size: 0.8em;
}

</style>
