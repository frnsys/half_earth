<template>
<div class="planning--page">
  <div class="minicard-grid">
    <div class="minicard-grid-item" v-for="region in regions">
      <MiniRegion :region="region" />
      <div class="minicard-grid-item-label">{{region.name}}</div>
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
