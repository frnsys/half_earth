<template>
<Card class="region">
  <template v-slot:header>
    <div>{{name}}</div>
    <div>{{population.toLocaleString()}}<img :src="icons.population"></div>
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

      <div v-for="v, k in demand" v-tip="{text: `This regions\'s demand for ${k}. This makes up X% of total demand for ${k}.`, icon: k}">
        <div class="card-icon">
          <img :src="icons[k]"/>
          {{demand[k]}}
        </div>
      </div>
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
  computed: {
    contentedness() {
      return display.scaleIntensity(this.region.outlook, 'outlook');
    },
    demand() {
      return display.outputs(game.regionDemand(this.region));
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
