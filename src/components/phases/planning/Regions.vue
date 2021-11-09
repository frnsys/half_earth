<template>
<div class="planning--page">
  <div class="space-even planning--regions">
    <div class="region--item" v-for="region in regions">
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
      regions: state.gameState.world.regions
    }
  },
}
</script>

<style>
.planning--regions {
  flex-wrap: wrap;
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
