<template>
<MiniCard>
  <template v-slot:body>
    <div class="minicard-background" :style="{backgroundImage: `url(/assets/content/images/${image.fname})`}" />
    <div :style="{zIndex: 1}">
      <div>🏭</div>
      <img src="/assets/icons/pips/power.png" />
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

<style>
.minicard-background {
  background-size: cover;
  background-position: center center;
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  z-index: 0;
}
</style>
