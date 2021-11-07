<template>
<div class="planning--page">
  <div class="planning--dashboard">
    <div>Population: {{state.gameState.population.toLocaleString()}}</div>
    <div>Land Use: {{parseInt((state.gameState.resources_demand.land * 1e-6).toFixed(0)).toLocaleString()}}km2</div>
  </div>
</div>
</template>

<script>
import state from '/src/state';

export default {
  data() {
    return {
      state
    }
  }
}
</script>

<style>
</style>
