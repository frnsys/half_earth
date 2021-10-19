<template>
<Hud />
<div id="event-stream-timer">
  <div id="event-stream-timer-fill" :style="{width: `${this.time}%`}"></div>
</div>
<div id="event-stream">
  <Globe id="stream-globe" ref="globe" />
  <Dialogue v-if="event && event.dialogue" :dialogue="event.dialogue" @done="nextEvent" @select="selectChoice" />
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import Hud from '../Hud.vue';
import Globe from '../Globe.vue'
import EventSwipe from './EventSwipe.vue'
import EventsMixin from '../EventsMixin';
import regionsToTiles from '/assets/surface/regions_to_tiles.json';
import iconEvents from '/assets/content/icon_events.json';

/* <EventSwipe v-if="event" :event="event" @selected="selectChoice" /> */

function randChoice(arr) {
  return arr[Math.floor(Math.random() * arr.length)];
}

export default {
  mixins: [EventsMixin],
  data() {
    return {
      time: 0,
      state,
    };
  },
  components: {
    Hud,
    Globe,
    EventSwipe,
  },
  mounted() {
    state.cycleStartState = {
      extinctionRate: state.gameState.world.extinction_rate,
      contentedness: state.gameState.contentedness,
      temperature: state.gameState.world.temperature,
    };

    this.$refs.globe.onReady = (globe) => {
      this.globe = globe;
      this.nextTurn();
      this.runYear();
    };

  },
  watch: {
    eventIdx(val) {
      console.log(`eventIdx changed: ${val}`);
      // Finished events, go to next turn
      if (val === null) {
        this.nextTurn();
      }
    }
  },
  methods: {
    runYear() {
      const tick = () => {
        this.time += 0.1;
        if (this.time >= 100) {
          // TODO roll event
        } else {
          requestAnimationFrame(tick);
        }
      };
      requestAnimationFrame(tick);
    },
    nextTurn() {
      // Go to report phase
      if (state.gameState.world.year % 5 == 0) {
        state.phase = 'REPORT';
        return;
      }

      console.log('ICON EVENTS');
      game.rollIconEvents().forEach((ev) => {
        this.showEventOnGlobe(ev[0], ev[1]);
      });

      this.eventIdx = 0;
      this.events = game.step();
      let emissions = {
        // Hector separates out FFI and LUC emissions
        // but we lump them together
        // Units: <https://github.com/JGCRI/hector/wiki/Hector-Units>
        'ffi_emissions': state.gameState.world.co2_emissions * 12/44 * 1e-15, // Pg C/y
        'CH4_emissions': state.gameState.world.ch4_emissions * 1e-12, // Tg/y
        'N2O_emissions': state.gameState.world.n2o_emissions * 1e-12, // Tg/y
      };
      this.globe.addEmissionsThenUpdate({}).then((tgav) => {
        game.setTgav(tgav);
      });
      this.showEvent();

      // Go to next turn if no events
      console.log(`Events: ${this.events.length}`);
      if (this.events.length === 0) this.nextTurn();
    },
    showEventOnGlobe(eventId, regionId) {
      console.log(`Showing globe event: ${eventId}, ${regionId}`);
      let ev = iconEvents[eventId];
      if (this.globe && regionId) {
        // TODO distinguish inland vs coastal events
        let region = state.gameState.world.regions[regionId];
        let tiles = regionsToTiles[region.name];
        let tileIdx = randChoice(tiles.inland.concat(tiles.coasts));
        console.log(`Chose tileIdx: ${tileIdx} for region ${region.name}`)
        console.log(`From tiles: ${tiles.inland.concat(tiles.coast)}`)
        let v = ev.outlookChange;
        let str = `${v < 0 ? '' : '+'}${v}`;
        this.globe.showIconText(ev.icon, str, tileIdx);
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

#event-stream-timer {
  width: 100%;
  height: 2px;
  background: #aaa;
  position: relative;
  z-index: 1;
}
#event-stream-timer-fill {
  height: 2px;
  background: red;
}
</style>
