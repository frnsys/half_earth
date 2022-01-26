<template>
<Card class="region">
  <template v-slot:header>
    <div>{{name}}</div>
    <div>{{format.formatNumber(population)}}<img :src="icons.population"></div>
  </template>
  <template v-slot:figure>
    <img class="card-image" :src="`/assets/content/images/${image.fname}`" />
    <div class="card-tack-ur">
      <div class="region-stat" v-tip="{icon: 'warming', text: 'This region\'s current temperature range.'}">
        <img :src="icons.warming">{{Math.round(temp_lo)}}-{{Math.round(temp_hi)}}°C
      </div>
      <br />
      <div class="region-stat" v-tip="{icon: 'precipitation', text: 'This region\'s current precipitation range.'}">
        <img :src="icons.precipitation">{{Math.round(precip_lo)}}-{{Math.round(precip_hi)}}cm/yr
      </div>
    </div>
    <div v-if="seceded" class="card-tack-cb">Seceded</div>
  </template>
  <template v-slot:body>
    <div class="space-even">
      <IntensityIcon
        v-tip="{icon: 'wealth', text: `This region has ${incomeName} living standards. Higher living standards mean higher material footprints.`}"
        resource="wealth" :intensity="incomeLevel" :invert="true" />
      <IntensityIcon
        v-tip="{icon: 'habitability', text: `This region's habitability.`}"
        resource="habitability" :intensity="habitability" :invert="true" />
      <IntensityIcon
        v-tip="{icon: 'contentedness', text: `This region's contentedness.`}"
        resource="contentedness" :intensity="contentedness" :invert="true" />
      <IntensityIcon
        v-for="v, k in demand"
        v-tip="{text: `This region's per-capita demand level for ${k}. The total regions's demand is ${demand[k] < 1 ? '<1' : demand[k]}. This makes up ${demandPercent(k)} of total demand for ${k}.`, icon: k}"
        :resource="k" :intensity="demandIntensity(k)" />
    </div>
  </template>
  <template v-slot:back>
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
import Card from './Card.vue';
import format from '/src/display/format';
import display from '/src/display/display';
import intensity from '/src/display/intensity';
import IntensityIcon from './IntensityIcon.vue';
import REGIONS from '/assets/content/regions.json';

export default {
  props: ['region'],
  components: {
    Card,
    IntensityIcon,
  },
  data() {
    let data = REGIONS[this.region.id];
    return {
      ...data,
      ...this.region,
    };
  },
  methods: {
    perCapitaDemand(k) {
      return this.region.demand[k]/this.population;
    },
    demandIntensity(k) {
      return intensity.intensity(this.perCapitaDemand(k), k);
    },
    demandPercent(k) {
      return format.demandPercent(this.demand, state.gameState.output_demand, k);
    }
  },
  computed: {
    contentedness() {
      return intensity.scale(this.region.outlook, 'outlook');
    },
    demand() {
      return format.outputs(this.region.demand);
    },
    habitability() {
      return intensity.scale(this.region.habitability, 'habitability');
    },
    incomeName() {
      return display.enumDisplay(this.income);
    },
    incomeLevel() {
      return this.income_level + 1;
    }
  }
}
</script>

<style>
.region .card-tack-ur {
  text-align: right;
}
.region .card-tack-cb {
  color: #fff;
  background: #222;
  border: 1px solid #fff;
  font-family: 'Andada Pro';
  text-transform: uppercase;
  font-size: 0.8em;
  padding: 0.1em 0.2em;
  border-radius: 0.2em;
}
.region-stat {
  color: #fff;
  padding: 0.3em 0.4em 0.2em 0.4em;
  border-radius: 0.5em;
  font-size: 0.9em;
  margin-bottom: 0.1em;
  display: inline-block;
  text-transform: uppercase;
  background: rgba(0,0,0,0.35);
}
.region-stat img {
  height: 10px;
  vertical-align: middle;
  margin-right: 3px;
  margin-top: -2px;
}
</style>
