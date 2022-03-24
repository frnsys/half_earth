<template>
<div class="pips">
  <template v-if="kind == 'Policy'">
    {{availablePoints}}<img class="pip" :src="icons.political_capital">
  </template>
  <template v-else>
    <template v-if="availablePoints > 0">
      {{availablePoints}}<img class="pip" :src="icons[icon]">
    </template>
    <template v-else>
      {{nextPointCost}}<img class="pip" :src="icons.political_capital"> ⮕ <img class="pip" :src="icons[icon]">
    </template>
  </template>
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
    availablePoints() {
      if (this.kind == 'Policy') {
        return state.gameState.political_capital;
      } else {
        return state.points[this.kind.toLowerCase()];
      }
    },
    nextPointCost() {
      return game.nextPointCost(this.kind);
    }
  }
}
</script>
