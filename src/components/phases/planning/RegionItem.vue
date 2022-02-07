<template>
<div class="region-item">
  <div class="region-item--info cell">
    <img :src="`/assets/content/images/${image.fname}`" />
    <div v-if="region.seceded" class="seceded-label">Seceded</div>
    <div>
        <div class="region-stat" v-tip="{icon: 'temperature', text: 'This region\'s current temperature range.'}">
          <img :src="icons.temperature">{{Math.round(region.temp_lo)}}-{{Math.round(region.temp_hi)}}°C
        </div>
        <div class="region-stat" v-tip="{icon: 'precipitation', text: 'This region\'s current precipitation range.'}">
          <img :src="icons.precipitation">{{Math.round(region.precip_lo)}}-{{Math.round(region.precip_hi)}}cm/yr
        </div>
    </div>
    <div v-tip="{icon: 'development', text: `This region's progress to the next income level.`}">
      <span>Development</span>
      <div class="minibar">
        <div class="minibar-fill"
          :style="{width: `${region.income == 'High' ? 100 : region.development * 100}%`}"></div>
      </div>
    </div>
    <div>
      <div>Recent Disasters</div>
      <div>
        <img :src="icons[ev.icon]" v-for="ev in events">
      </div>
    </div>
  </div>
  <div class="region-item--intensities cell">
    <IntensityIcon
      v-tip="{icon: 'habitability', text: `This region's habitability.`}"
      resource="habitability" :intensity="habitability" :invert="true" />
    <IntensityIcon
      v-tip="{icon: 'contentedness', text: `This region's contentedness.`}"
      resource="contentedness" :intensity="contentedness" :invert="true" />
    <IntensityIcon
      v-tip="{icon: 'wealth', text: `This region has ${incomeName} living standards. Higher living standards mean higher material footprints.`}"
      resource="wealth" :intensity="incomeLevel" :invert="true" />
    <IntensityIcon
      v-for="v, k in demand"
      v-tip="{text: `This region's per-capita demand level for ${display.enumDisplay(k)}. The total regions's demand is ${demand[k] < 1 ? '<1' : demand[k]}. This makes up ${demandPercent(k)} of total demand for ${display.enumDisplay(k)}.`, icon: k}"
      :resource="k" :intensity="demandIntensity(k)" />
  </div>
</div>
</template>

<script>
import state from '/src/state';
import format from '/src/display/format';
import display from '/src/display/display';
import intensity from '/src/display/intensity';
import REGIONS from '/assets/content/regions.json';
import IntensityIcon from '../../cards/IntensityIcon.vue';

export default {
  props: ['region'],
  components: {
    IntensityIcon
  },
  data() {
    return {
      ...REGIONS[this.region.id],
      events: state.annualRegionEvents[this.region.id]
    }
  },
  methods: {
    perCapitaDemand(k) {
      return this.region.demand[k]/this.region.population;
    },
    demandIntensity(k) {
      return intensity.intensity(this.perCapitaDemand(k), k);
    },
    demandPercent(k) {
      return format.demandPercent(this.region.demand, state.gameState.output_demand, k);
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
      return display.enumDisplay(this.region.income);
    },
    incomeLevel() {
      return this.region.income_level + 1;
    }
  }
}
</script>

<style>
.region-item {
  display: flex;
  justify-content: space-between;
  color: #fff;
}
.region-item-disasters img {
  width: 18px;
}
.region-stat {
  margin-right: 0.5em;
}
.region-item--info {
  margin-right: 0.5em;
  font-size: 0.7em;
  text-transform: uppercase;
  flex: 1;
  font-family: 'Inter', sans-serif;
  font-weight: 600;
}
.region-item--info > img {
  border-radius: 0.3em;
  height: 160px;
  width: 100%;
  object-fit: cover;
  object-position: top;
}
.region-item--info .minibar {
  display: inline-block;
  margin-left: 0.5em;
  height: 9px;
  border-radius: 0.4em;
  border: 1px solid #fff;
  background: #fff;
}
.region-item--info .minibar-fill {
  border-radius: 0.4em;
}
.region-item--info > * {
  margin-bottom: 0.5em;
}
.region-item--intensities .card-icon {
  width: 70px;
  display: flex;
  align-items: center;
  margin-bottom: 0.5em;
}
.region-item--intensities .card-icon img {
  width: 28px;
  margin-right: 0.5em;
}

.seceded-label {
  position: absolute;
  left: 50%;
  transform: translate(-50%, 0);
  top: 6em;
  color: #fff;
  background: red;
  padding: 0.2em;
}
</style>
