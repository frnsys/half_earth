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
      <div class="priority--desc">{{description}}</div>
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
  },
  computed: {
    description() {
      return consts.priorities[consts.Priority[state.gameState.priority]].desc;
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
.priority--desc {
  background: #222;
  color: #fff;
  padding: 1em;
  max-width: 320px;
  text-align: center;
  margin: 2em 0 0 0;
  border-radius: 0.2em;
}
</style>
