<template>
<MiniCard>
  <template v-slot:body>
    <div>🏭</div>
    <img src="/assets/icons/pips/power.png" />
  </template>
  <template v-slot:expanded>
    <ProcessCard :process="process" />
  </template>
</MiniCard>
</template>

<script>
import MiniCard from './MiniCard.vue';
import ProcessCard from './ProcessCard.vue';

export default {
  props: ['process'],
  components: {
    MiniCard,
    ProcessCard,
  }
}
</script>
