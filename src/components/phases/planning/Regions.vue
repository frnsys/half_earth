<template>
<div class="planning--page">
  <div class="planning--regions">
    <div class="region--item" v-for="region in state.gameState.world.regions">
      <MiniRegion :region="region" />
      <div class="region--item-name">{{region.name}}</div>
    </div>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import MiniRegion from 'components/cards/MiniRegion.vue';

export default {
  components: {
    MiniRegion
  },
  data() {
    return {
      state
    }
  },
  methods: {
  },
  computed: {
  }
}
</script>

<style>
.planning--regions {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-evenly;
}
.region--item {
  width: 80px;
  margin: 1em 0;
}
.region--item-name {
  text-align: center;
  font-size: 0.8em;
}
</style>
