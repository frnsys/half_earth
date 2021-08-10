<template>
  <Globe ref="globe" />
  <Hud />

  <div id="event-wrapper">
    <button @click="prevEvent">&lt;</button>
    <Event v-if="activeEvent >= 0" :event="state.events[activeEvent]" />
    <div v-else>
      <p>Emergency Measures</p>
      <em class="note">Make changes to the plan; these cost PC to use</em>
    </div>
    <button @click="nextEvent">&gt;</button>
  </div>

  <div class="actions">
    <button v-if="state.events.every((ev) => ev.selectedResponse !== null)" @click="nextTurn">Next Year</button>
  </div>
</template>

<script>
import state from '../state';
import Hud from './Hud.vue';
import Globe from './Globe.vue'
import Event from './Event.vue';
export default {
  data() {
    return {
      activeEvent: 0,
      state
    };
  },
  components: {
    Hud,
    Event,
    Globe,
  },
  mounted() {
    this.$refs.globe.globe.onReady((globe) => {
      this.globe = globe;

      // Jump to tile for current event
      let idx = state.events[this.activeEvent].location;
      globe.hexsphere.centerOnIndex(idx);

      // Jump to event on clicked tile, if any
      globe.hexsphere.onClick((tile) => {
        let event = state.events.find((ev) => ev.location == tile.idx);
        if (event) {
          let idx = state.events.indexOf(event);
          this.activeEvent = idx;
        }
      });
    });
  },
  methods: {
    nextTurn() {
      state.player.year++;

      // Reset events
      // TODO this is just for testing...we should actually generate new events
      state.events.forEach((ev) => {
        ev.selectedResponse = null;
      });

      // Lose state
      if (state.player.political_capital <= 0) {
        alert('You\'ve lost your planning mandate! You lose');
      }

      if (state.player.year % 5 == 0) {
        state.phase = 'REPORT';
      }
    },
    nextEvent() {
      this.activeEvent++;
      if (this.activeEvent > this.state.events.length - 1) {
        this.activeEvent = -1;
      }
      if (this.activeEvent >= 0 && this.globe) {
        let idx = state.events[this.activeEvent].location;
        this.globe.hexsphere.centerOnIndex(idx);
      }
    },
    prevEvent() {
      this.activeEvent--;
      if (this.activeEvent < -1) {
        this.activeEvent = this.state.events.length - 1;
      }
      if (this.activeEvent >= 0 && this.globe) {
        let idx = state.events[this.activeEvent].location;
        this.globe.hexsphere.centerOnIndex(idx);
      }
    },
    jumpToEvent(event) {
      console.log(this.$refs.globe.hexphere);
    },
    toggleResponse(response) {
      // TODO
      let ev = this.state.events[this.activeEvent];
      if (ev.selectedResponse == response) {
        ev.selectedResponse = null;
      } else {
        ev.selectedResponse = response;
      }
      this.updateEstimates();
    },
  }
}
</script>

<style>
#event-wrapper {
  margin: 1em 0;
  position: fixed;
  bottom: 120px;
  width: 100%;
  display: flex;
  justify-content: space-between;
}
</style>

