<template>
<div class="planning--page">
  <header>
    <img class="back" @click="$emit('close')" src="/assets/icons/back.svg">
  </header>
  <div class="planning--menu priority--menu">
    <button v-for="name, p in priorities" @click="select(p)" :class="{selected: p == priority}">
      <div>{{p == 'output' ? 'Maximize' : 'Minimize'}}</div>
      <img :src="assets.icons[p]" />
      <div>{{name}}</div>
    </button>
  </div>
</div>
</template>

<script>
const priorities = {
  'output': 'output',
  'land': 'land use',
  'emissions': 'emissions',
  'energy': 'energy use',
  'labor': 'labor',
  'water': 'water use',
};

export default {
  data() {
    return {
      priority: 'output',
    }
  },
  created() {
    this.priorities = priorities;
  },
  methods: {
    select(priority) {
      // TODO set in game
      this.priority = priority;
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
