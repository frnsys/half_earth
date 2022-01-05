<template>
<div class="planning--page">
  <div v-for="region in regions">
    <RegionItem :region="region" />
  </div>
</div>
</template>

<script>
import state from '/src/state';
import RegionItem from './RegionItem.vue';

export default {
  components: {
    RegionItem
  },
  data() {
    return {
      regions: state.gameState.world.regions
    }
  },
}
</script>
