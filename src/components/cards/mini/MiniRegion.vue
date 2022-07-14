<template>
<MiniCard>
  <template v-slot:body>
    <div v-if="region.seceded" class="seceded-label">{{t('Seceded')}}</div>
    <div class="minicard-background" :style="{backgroundImage: `url(/assets/content/images/${image.fname})`}" :class="{seceded: region.seceded}" />
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
    return {
      ...REGIONS[this.region.id],
    }
  },
  components: {
    MiniCard,
    RegionCard,
  }
}
</script>

<style>
.seceded {
  opacity: 0.75;
  filter: grayscale(1);
}
.seceded-label {
  color: #fff;
  background: #222;
  border: 1px solid #fff;
  text-transform: uppercase;
  font-size: 0.8em;
  padding: 0.1em 0.2em;
  border-radius: 0.2em;
  position: relative;
  z-index: 1;
}
</style>
