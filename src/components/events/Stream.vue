<template>
<Hud />
<div id="event-stream">
  <Globe id="stream-globe" ref="globe" />
  <Dialogue v-if="event" :dialogue="event.dialogue" @done="nextEvent" @select="selectChoice" />
</div>
</template>

<script>
import game from '../../game';
import state from '../../state';
import Hud from '../Hud.vue';
import Globe from '../Globe.vue'
import EventSwipe from './EventSwipe.vue'
import EventsMixin from '../EventsMixin';

/* <EventSwipe v-if="event" :event="event" @selected="selectChoice" /> */

export default {
  mixins: [EventsMixin],
  data() {
    return {
      state,
    };
  },
  components: {
    Hud,
    Globe,
    EventSwipe,
  },
  mounted() {
    this.$refs.globe.onReady = (globe) => {
      this.globe = globe;
      this.nextTurn();
      this.showEventOnGlobe();
    };
  },
  watch: {
    eventIdx(val) {
      // Finished events, go to next turn
      if (val === null) {
        this.nextTurn();

      // Show on globe
      } else {
        this.showEventOnGlobe();
      }
    }
  },
  methods: {
    nextTurn() {
      // Go to report phase
      if (state.gameState.world.year % 5 == 0) {
        state.phase = 'REPORT';
        return;
      }

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
      console.log(emissions);
      this.globe.addEmissionsThenUpdate({}).then((tgav) => {
        console.log(`New TGAV: ${tgav}C`);
        game.setTgav(tgav);
      });
      this.showEvent();

      // Go to next turn if no events
      if (this.events.length === 0) this.nextTurn();
    },
    showEventOnGlobe() {
      let [eventId, regionId] = this.events[this.eventIdx];
      if (this.globe && regionId) {
        // TODO
        // Jump to tile for current event
        /* let idx = event.location; // TODO no location */
        /* if (idx) { */
        /*   this.globe.hexsphere.centerOnIndex(idx); */
        /* } */
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
</style>
