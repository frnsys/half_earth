<template>
<div class="pips">
  <div class="pips-group">
    {{pcPoints}}<img class="pip" :src="icons.political_capital">
  </div>
  <div class="pips-group" v-if="kind != 'Policy'">
    <template v-if="availablePoints > 0">
      {{availablePoints}}<img class="pip" :src="icons[icon]">
    </template>
    <template v-else>
      {{nextPointCost}}<img class="pip" :src="icons.political_capital"> <img :src="icons.arrow_right" class="pip-arrow"/> <img class="pip" :src="icons[icon]">
    </template>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';

export default {
  props: ['kind'],
  computed: {
    icon() {
      return this.kind.toLowerCase();
    },
    pcPoints() {
      return state.gameState.political_capital;
    },
    availablePoints() {
      return state.points[this.kind.toLowerCase()];
    },
    nextPointCost() {
      return game.nextPointCost(this.kind);
    }
  }
}
</script>

<style>
.pips {
  margin: 0 auto 0.5em;
  position: relative;
  text-align: center;
  font-size: 1.2em;
  color: #fff;
  user-select: none;
  display: flex;
}

.pips-group {
  border-radius: 0.2em;
  background: rgba(0,0,0,0.1);
  padding: 0.5em 0.5em 0.4em 0.5em;
  display: flex;
  align-items: center;
}
.pips-group:nth-child(2) {
  margin-left: 0.5em;
}

.pip-arrow {
  filter: invert(1);
}
</style>
