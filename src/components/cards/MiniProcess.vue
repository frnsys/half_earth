<template>
<MiniCard>
  <template v-slot:body>
    <div class="minicard-background" :style="{backgroundImage: `url(/assets/content/images/${image.fname})`}" />
    <div :style="{zIndex: 1}">
      <img v-if="process.output == 'Electricity' || process.output == 'Fuel'" src="/assets/icons/pips/power.png" />
      <img v-else src="/assets/icons/pips/food.png" />
    </div>
  </template>
  <template v-slot:expanded>
    <ProcessCard :process="process" />
  </template>
</MiniCard>
</template>

<script>
import state from '/src/state';
import MiniCard from './MiniCard.vue';
import ProcessCard from './ProcessCard.vue';

export default {
  props: ['process'],
  data() {
    return {
      ...state.processes[this.process.id],
    }
  },
  components: {
    MiniCard,
    ProcessCard,
  }
}
</script>
