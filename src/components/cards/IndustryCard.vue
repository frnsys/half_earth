<template>
<Card class="industry" color="#000000" background="palevioletred">
  <template v-slot:header>
    <div>{{t('Sector')}}</div>
  </template>
  <template v-slot:figure>
    <img class="card-image" :src="`/assets/content/images/${image.fname}`" />
  </template>
  <template v-slot:name>
    {{t(name)}}
  </template>
  <template v-slot:body>
    <div class="space-even">
      <template v-if="empty">
        {{t('This industry is not yet significant.')}}
      </template>
      <template v-else>
        <div v-for="v, k in totalResources" v-tip="{text: t(`This industry's demand for {output}. This makes up {percent} of total demand for {output}.`, {output: k, percent: demandPercent(k)}), icon: k}">
          <div class="card-icon">
            <img :src="icons[k]"/>
            {{totalResources[k]}}
          </div>
        </div>
        <div v-if="totalByproducts.emissions"
          v-tip="{text: t('This industry\'s non-energy CO2eq emissions.'), icon: 'emissions'}">
          <div class="card-icon">
            <img :src="icons.emissions" />
            {{totalByproducts.emissions < 1 ? '<1' : totalByproducts.emissions.toFixed(0)}}
          </div>
        </div>
      </template>
    </div>
  </template>
  <template v-slot:top-back>
    <p class="card-desc">{{t(description)}}</p>
  </template>
  <template v-slot:bot-back>
    <div class="card-image-attribution">
      Image: {{image.attribution}}
    </div>
  </template>
</Card>

</template>

<script>
import state from '/src/state';
import game from '/src/game';
import Card from './Card.vue';
import format from '/src/display/format';
import INDUSTRIES from '/assets/content/industries.json';

export default {
  props: ['industry'],
  components: {
    Card,
  },
  data() {
    return {
      ...this.industry,
      ...INDUSTRIES[this.industry.id],
    };
  },
  methods: {
    demandPercent(k) {
      return format.demandPercent(this.totalResources, state.gameState.output_demand, k);
    }
  },
  computed: {
    empty() {
      let total = Object.values(this.totalResources).reduce((acc, v) => acc + v, 0);
      return total == 0;
    },
    demand() {
      return game.industryDemand(this.industry);
    },
    totalResources() {
      let resources = Object.keys(this.industry.resources).reduce((acc, k) => {
        if (this.industry.resources[k] !== 0) {
          acc[k] = this.industry.resources[k] * this.demand;
        }
        return acc;
      }, {});
      return format.outputs(resources);
    },
    totalByproducts() {
      let byproducts = {};
      let emissions = format.co2eq(this.industry.byproducts) * this.demand * 1e-15;
      if (emissions !== 0) {
        byproducts['emissions'] = emissions;
      }
      return byproducts;
    },
  }
}
</script>
