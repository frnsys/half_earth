<template>
<MiniCard>
  <template v-slot:body>
    <div class="minicard-background" :style="{backgroundImage: `url(/assets/content/images/${image.fname})`}" />
  </template>
  <template v-slot:expanded>
    <RegionCard :region="region" />
  </template>
</MiniCard>
</template>

<script>
import MiniCard from './MiniCard.vue';
import RegionCard from './RegionCard.vue';
import REGIONS from '/assets/content/regions.json';

export default {
  props: ['region'],
  data() {
    let data = REGIONS[this.region.id];
    return {
      ...data,
      ...this.region,
    }
  },
  components: {
    MiniCard,
    RegionCard,
  }
}
</script>
