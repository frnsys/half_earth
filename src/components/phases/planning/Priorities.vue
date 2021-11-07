<template>
<div class="planning--page">
  <header>
    <img class="back" @click="$emit('close')" src="/assets/icons/back.svg">
  </header>
  <div class="planning--menu priority--menu">
    <button v-for="d, p in priorities" @click="select(p)" :class="{selected: Priority[p] == state.gameState.priority}">
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

const priorities = {
  [Priority.Scarcity]: {
    icon: 'output',
    name: 'Scarcity',
  },
  [Priority.Land]: {
    icon: 'land',
    name: 'Land Use',
  },
  [Priority.Emissions]: {
    icon: 'emissions',
    name: 'Emissions',
  },
  [Priority.Energy]: {
    icon: 'energy',
    name: 'Energy Use',
  },
  [Priority.Labor]: {
    icon: 'labor',
    name: 'Labor',
  },
  [Priority.Water]: {
    icon: 'water',
    name: 'Water Use',
  },
};

export default {
  data() {
    return {
      state,
    }
  },
  created() {
    this.Priority = Priority;
    this.priorities = priorities;
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
