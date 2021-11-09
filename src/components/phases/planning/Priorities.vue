<template>
<div class="plan-change-select">
  <header>
    <div>Production Priority</div>
    <div @click="$emit('close')">Close</div>
  </header>
  <div class="planning--page">
    <div class="planning--menu priority--menu">
      <button v-for="d, p in consts.priorities" @click="select(p)" :class="{selected: consts.Priority[p] == state.gameState.priority}">
        <img :src="icons[d.icon]" />
        <div>{{d.name}}</div>
      </button>
    </div>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';

export default {
  data() {
    return {
      state,
    }
  },
  methods: {
    select(priority) {
      game.setPriority(priority);
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
