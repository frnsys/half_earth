<template>
<Hud />
<div id="event-stream">
  <Globe id="stream-globe" ref="globe" />
  <EventSwipe v-if="event" :event="event" @selected="selectChoice" />
</div>
</template>

<script>
import game from '../../game';
import state from '../../state';
import Hud from '../Hud.vue';
import Globe from '../Globe.vue'
import EventSwipe from './EventSwipe.vue'

export default {
  data() {
    return {
      state,
      events: [],
      event: null,
      eventIdx: 0,
    };
  },
  components: {
    Hud,
    Globe,
    EventSwipe,
  },
  mounted() {
    this.nextTurn();
    this.$refs.globe.onReady = (globe) => {
      this.globe = globe;
      this.showEvent();
    };
  },
  methods: {
    async loadEvent(id) {
      return await fetch(`/assets/content/events/${id}.json`)
        .then((resp) => resp.json());
    },
    nextTurn() {
      if (state.gameState) {
        // Lose state
        if (state.gameState.political_capital <= 0) {
          alert('You\'ve lost your planning mandate! You lose');
          return;
        }

        // Go to planning phase
        if (state.gameState.world.year % 5 == 0) {
          state.phase = 'REPORT';
          return;
        }
      }

      this.eventIdx = 0;
      this.events = game.step();

      // Go to next turn if no events
      if (this.events.length === 0) this.nextTurn();
    },
    nextEvent() {
      if (this.eventIdx >= this.events.length - 1) {
        this.nextTurn();
      } else {
        this.eventIdx++;
        this.showEvent();
      }
    },
    showEvent() {
      let [eventId, regionId] = this.events[this.eventIdx];
      if (this.globe && regionId) {
        // TODO
        // Jump to tile for current event
        /* let idx = event.location; // TODO no location */
        /* if (idx) { */
        /*   this.globe.hexsphere.centerOnIndex(idx); */
        /* } */
      }
      this.loadEvent(eventId).then((ev) => {
        this.event = ev;

        // Parse/fill in variables
        let vars = [...ev.text.matchAll('{([a-z]+)}')];
        let ctx = {'region': 'PLANET EARTH'} // TODO just testing
        for (const match of vars) {
          ev.text = ev.text.replaceAll(match[0], ctx[match[1]]);
        }
      });
    },
    selectChoice(idx) {
      // TODO skipping this until we figure out dialogue or swipe events
      /* let [eventId, regionId] = this.events[this.eventIdx]; */
      /* game.selectChoice(eventId, regionId, idx); */
      this.nextEvent();
    }
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
