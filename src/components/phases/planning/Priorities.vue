<template>
<div class="planning--page">
  <div class="planning--menu priority--menu">
    <button v-for="d, p in consts.priorities" @click="select(Priority[p])" :class="{selected: Priority[p] == priority}">
      <img :src="assets.icons[d.icon]" />
      <div>{{d.name}}</div>
    </button>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import {Priority} from 'half-earth-engine';

export default {
  data() {
    return {
      state,
    }
  },
  created() {
    this.Priority = Priority;
  },
  computed: {
    priority() {
      let existing = state.planChanges.find((change) => change.type == 'priority');
      if (existing === undefined) {
        return state.gameState.priority;
      } else {
        return existing.priority;
      }
    }
  },
  methods: {
    select(priority) {
      // See if the priority is changed from the currently set one
      let unchanged = priority == state.gameState.priority;

      // Find existing priority change
      let existing = state.planChanges.findIndex((change) => change.type == 'priority');
      let exists = existing >= 0;
      if (exists) {
        if (unchanged) {
          state.planChanges.splice(existing, 1);
        } else {
          state.planChanges[existing].priority = priority;
        }
      } else if (!exists && !unchanged) {
        state.planChanges.push({
          action: 'Prioritize',
          type: 'priority',
          priority,
        });
      }

    }
  }
}
</script>

<style>
.priority--menu button.selected {
  background: #e6e3e3;
  border-top: 2px solid #5D5D5D;
  border-left: 2px solid #5D5D5D;
  border-right: 2px solid #F1F1F1;
  border-bottom: 2px solid #F1F1F1;
}
</style>
