<template>
<Card class="industry">
  <template v-slot:header>
    <div>{{name}}</div>
  </template>
  <template v-slot:figure>
    <img class="card-image" :src="`/assets/content/images/${image.fname}`" />
  </template>
  <template v-slot:body>
    <div class="icon-stats">
      <div v-for="v, k in totalResources" v-tip="{text: `This industry\'s demand for ${k}. This makes up X% of total demand for ${k}.`, icon: k}">
        <div class="resource-icon">
          <img :src="assets.icons[k]"/>
          {{totalResources[k]}}
        </div>
      </div>
      <div v-if="byproducts.emissions" v-tip="{text: 'This industry\'s non-energy CO2eq emissions.', icon: 'emissions'}">{{totalByproducts.emissions}}{{consts.icons['emissions']}}</div>
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
import game from '/src/game';
import Card from './Card.vue';
import {slugify} from 'lib/util';
import display from 'lib/display';
import INDUSTRIES from '/assets/content/industries.json';

export default {
  props: ['industry'],
  components: {
    Card,
  },
  data() {
    let data = INDUSTRIES[this.industry.id];
    return {
      ...data,
      ...this.industry,
    };
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
    }
  }
}
</script>

<style>
.resource-icon {
  width: 30px;
  text-align: center;
}
</style>

