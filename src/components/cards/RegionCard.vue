<template>
<Card class="region">
  <template v-slot:header>
    <div>{{name}}</div>
    <div>{{abbrevPopulation}}<img :src="icons.population"></div>
  </template>
  <template v-slot:figure>
    <img class="card-image" :src="`/assets/content/images/${image.fname}`" />
  </template>
  <template v-slot:body>
    <div class="space-even">
      <IntensityIcon
        v-tip="{icon: 'wealth', text: `This region is ${incomeName} income. Higher incomes mean higher material footprints.`}"
        resource="wealth" :intensity="incomeLevel" />
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
import game from '/src/game';
import state from '/src/state';
import Card from './Card.vue';
import display from 'lib/display';
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
      return this.rawDemand[k]/this.population;
    },
    demandIntensity(k) {
      return display.intensity(this.perCapitaDemand(k), k);
    },
    demandPercent(k) {
      let scaledOutputDemand = display.outputs(state.gameState.output_demand);
      let percent = this.demand[k]/scaledOutputDemand[k] * 100;
      if (percent < 1) {
        return '<1%';
      } else {
        return `${percent.toFixed(1)}%`;
      }
    }
  },
  computed: {
    abbrevPopulation() {
      return display.formatNumber(this.population);
    },
    contentedness() {
      return display.scaleIntensity(this.region.outlook, 'outlook');
    },
    rawDemand() {
      return game.regionDemand(this.region);
    },
    demand() {
      return display.outputs(this.rawDemand);
    },
    habitability() {
      return display.scaleIntensity(game.regionHabitability(this.region), 'habitability');
    },
    incomeName() {
      return display.enumDisplay(this.income);
    },
    incomeLevel() {
      switch (this.income) {
        case 'Low': return 1;
        case 'LowerMiddle': return 2;
        case 'UpperMiddle': return 3;
        case 'High': return 4;
      }
      return 1;
    }
  }
}
</script>
