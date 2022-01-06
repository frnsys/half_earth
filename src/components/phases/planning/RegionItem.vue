<template>
<div class="region-item">
  <div v-if="region.seceded" class="seceded-label">Seceded</div>
  <div class="region-item-header">
    {{region.name}}
    <div>
      <div class="region-stat" v-tip="{icon: 'precipitation', text: 'This region\'s current precipitation range.'}">
        <img :src="icons.precipitation">{{Math.round(region.precip_lo)}}-{{Math.round(region.precip_hi)}}cm/yr
      </div>
      <div class="region-stat" v-tip="{icon: 'warming', text: 'This region\'s current temperature range.'}">
        <img :src="icons.warming">{{Math.round(region.temp_lo)}}-{{Math.round(region.temp_hi)}}°C
      </div>
    </div>
  </div>
  <div class="region-item-body">
    <div class="region-item-image" :style="{backgroundImage: `url(/assets/content/images/${image.fname})`}" :class="{seceded: region.seceded}" />
    <div>
      <div class="space-even">
        <IntensityIcon
          v-tip="{icon: 'habitability', text: `This region's habitability.`}"
          resource="habitability" :intensity="habitability" :invert="true" />
        <IntensityIcon
          v-tip="{icon: 'contentedness', text: `This region's contentedness.`}"
          resource="contentedness" :intensity="contentedness" :invert="true" />
        <div class="region-item-development">
          <IntensityIcon
            v-tip="{icon: 'wealth', text: `This region has ${incomeName} living standards. Higher living standards mean higher material footprints.`}"
            resource="wealth" :intensity="incomeLevel" :invert="true" />
          <div v-tip="{icon: 'development', text: `This region's progress to the next income level.`}">
            <div class="minibar-label">Development</div>
            <div class="minibar">
              <div class="minibar-fill"
                :style="{width: `${region.income == 'High' ? 100 : region.development * 100}%`}"></div>
            </div>
          </div>
        </div>
      </div>
      <div class="space-even">
        <IntensityIcon
          v-for="v, k in demand"
          v-tip="{text: `This region's per-capita demand level for ${k}. The total regions's demand is ${demand[k] < 1 ? '<1' : demand[k]}. This makes up ${demandPercent(k)} of total demand for ${k}.`, icon: k}"
          :resource="k" :intensity="demandIntensity(k)" />
      </div>
    </div>
  </div>
  <div class="region-item-disasters" v-if="events && events.length > 0">
    <div>Recent Disasters</div>
    <div>
      <img :src="icons[ev.icon]" v-for="ev in events">
    </div>
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
  padding: 1em;
}
.region-item-header {
  display: flex;
  justify-content: space-between;
  border-bottom: 1px solid;
}
.region-stat {
  margin-left: 0.5em;
}
.region-item-body {
  display: flex;
  justify-content: space-between;
}
.region-item-body .space-even {
  align-items: center;
}
.region-item-body > div:last-child {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
}
.region-item-development {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.region-item-development .card-icon {
  margin-right: 0.5em;
}

.seceded {
  opacity: 0.75;
  filter: grayscale(1);
}
.seceded-label {
  color: #fff;
  background: #222;
  border: 1px solid #fff;
  font-family: 'Andada Pro';
  text-transform: uppercase;
  font-size: 0.8em;
  padding: 0.1em 0.2em;
  border-radius: 0.2em;
  position: relative;
  z-index: 1;
}

.region-item-image {
  width: 120px;
  height: 120px;
  background-size: cover;
  background-position: center center;
}

.region-item-disasters {
  background: rgba(0,0,0,0.1);
  padding: 0.25em 0.5em;
}
.region-item-disasters > div:first-child {
  font-size: 0.75em;
}
.region-item-disasters img {
  width: 18px;
}
</style>
