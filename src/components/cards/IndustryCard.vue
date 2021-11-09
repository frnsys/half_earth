<template>
<Card class="industry">
  <template v-slot:header>
    <div>{{name}}</div>
  </template>
  <template v-slot:figure>
    <img class="card-image" :src="`/assets/content/images/${image.fname}`" />
  </template>
  <template v-slot:body>
    <div class="space-even">
      <div v-for="v, k in totalResources" v-tip="{text: `This industry\'s demand for ${k}. This makes up ${demandPercent(k)} of total demand for ${k}.`, icon: k}">
        <div class="card-icon">
          <img :src="icons[k]"/>
          {{totalResources[k]}}
        </div>
      </div>
      <div v-if="byproducts.emissions"
        v-tip="{text: 'This industry\'s non-energy CO2eq emissions.', icon: 'emissions'}">
        {{totalByproducts.emissions}}{{icons.emissions}}</div>
    </div>
  </template>
  <template v-slot:back>
    TODO active effects
    <div class="card-image-attribution">
      Image: {{image.attribution}}
    </div>
  </template>
  <template v-slot:footer>
    <div>GOSPLANT</div>
  </template>
</Card>

</template>

<script>
import state from '/src/state';
import game from '/src/game';
import Card from './Card.vue';
import display from 'lib/display';
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
      let percent = this.totalResources[k]/state.gameState.output_demand[k] * 100;
      if (percent < 1) {
        return '<1%';
      } else {
        return `${percent.toFixed(1)}%`;
      }
    }
  },
  computed: {
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
      return display.outputs(resources);
    },
    totalByproducts() {
      let byproducts = {};
      let emissions = display.gtco2eq(this.industry.byproducts);
      if (emissions !== 0) {
        byproducts['emissions'] = emissions * this.demand;
      }
      return byproducts;
    },
  }
}
</script>
