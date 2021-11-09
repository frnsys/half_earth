<template>
<MiniCard>
  <template v-slot:body>
    <div class="minicard-background" :style="{backgroundImage: `url(/assets/content/images/${image.fname})`}" />
    <div :style="{zIndex: 1}">
      <img :src="icons[icon]" />
    </div>
  </template>
  <template v-slot:expanded>
    <ProcessCard :process="process" />
  </template>
</MiniCard>
</template>

<script>
import MiniCard from './MiniCard.vue';
import ProcessCard from './ProcessCard.vue';
import PROCESSES from '/assets/content/processes.json';

export default {
  props: ['process'],
  data() {
    return {
      ...PROCESSES[this.process.id],
    }
  },
  components: {
    MiniCard,
    ProcessCard,
  },
  computed: {
    icon() {
      if (this.process.output == 'Electricity' || this.process.output == 'Fuel') {
        return 'energy';
      } else {
        return 'food';
      }
    }
  }
}
</script>
