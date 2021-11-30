<template>
<div class="planning--page">
  <div class="planning--dashboard">
    <div class="dashboard--item">
      <div class="minicard">
        {{`${(state.gameState.population * 1e-9).toFixed(1)}b`}}
      </div>
      <img :src="icons.population" />
      <div class="dashboard--item-name">Population</div>
    </div>
    <div class="dashboard--item">
      <div class="minicard">
      {{`${((state.gameState.resources_demand.land/104e12)*100).toFixed(0)}%`}}
      </div>
      <img :src="icons.land" />
      <div class="dashboard--item-name">Land Use</div>
    </div>
    <div class="dashboard--item">
      <div class="minicard">
      {{`${state.gameState.emissions.toFixed(1)}Gt`}}
      </div>
      <img :src="icons.emissions" />
      <div class="dashboard--item-name">Emissions</div>
    </div>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import MiniCard from 'components/cards/MiniCard.vue';

export default {
  components: {
    MiniCard
  },
  data() {
    return {
      state
    }
  },
}
</script>

<style>
.dashboard--item {
  margin: 1.5em 1em;
  position: relative;
  width: 120px;
  text-align: center;
}
.dashboard--item .minicard {
  color: #fff;
  font-size: 1.5em;
  width: 120px;
  height: 80px;
}
.dashboard--item img {
  position: absolute;
  top: 0;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 32px;
}

.planning--dashboard .minicard {
  background: #222;
}
</style>
