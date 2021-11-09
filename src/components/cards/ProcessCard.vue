<template>
<Card>
  <template v-slot:header>
    <div>{{name}}</div>
    <div v-tip="{text: `This process currently produces ${produced.amount}${consts.icons[output]} and ${produced.emissions}${consts.icons['emissions']} per year.`, icon: output}">{{produced.amount}}{{consts.icons[output]}} {{produced.emissions}}{{consts.icons['emissions']}}</div>
  </template>
  <template v-slot:figure>
    <img class="card-image" :src="`/assets/content/images/${image.fname}`" />
    <img
      v-tip="{text: `This process is expected to ${expectedChange}.`, icon: changeIcons[expectedChange]}"
      class="process-trend" :src="assets.icons[changeIcons[expectedChange]]">
    <img
      v-tip="{text: `This process uses ${feedstockName}.`, icon: feedstockIcon}"
      class="process-feedstock" :src="assets.icons[feedstockIcon]">
    <div class="opposers">
      <div>Nay</div>
      <div>
        <img v-tip="{text: `The Authoritarian is opposed to this. If you ban it, your relationship will improve by +<img src='${assets.icons.relationship}' />.`, icon: 'authoritarian'}" src="/assets/characters/The Authoritarian.png">
        <img v-tip="{text: `The Economist is opposed to this process. If you ban it, your relationship will improve by +<img src='${assets.icons.relationship}' />.`, icon: 'economist'}" src="/assets/characters/The Economist.png">
        <img v-tip="{text: `The Technocrat is opposed to this process. If you ban it, your relationship will improve by +<img src='${assets.icons.relationship}' />.`, icon: 'technocrat'}" src="/assets/characters/The Technocrat.png">
      </div>
    </div>
    <div class="supporters">
      <div>Yea</div>
      <div>
        <img v-tip="{text: `The Scientist supports this. If you promote it, your relationship will improve by +<img src='${assets.icons.relationship}' />.`, icon: 'scientist'}" src="/assets/characters/The Scientist.png">
        <img v-tip="{text: `The Populist supports this. If you promote it, your relationship will improve by +<img src='${assets.icons.relationship}' />.`, icon: 'populist'}" src="/assets/characters/The Populist.png">
        <img v-tip="{text: `The Ecologist supports this. If you promote it, your relationship will improve by +<img src='${assets.icons.relationship}' />.`, icon: 'ecologist'}" src="/assets/characters/The Ecologist.png">
      </div>
    </div>
  </template>
  <template v-slot:body>
    <div class="card-actions" v-if="!!this.$slots.actions">
      <slot name="actions"></slot>
    </div>
    <div class="process-intensity">
      <IntensityIcon
        v-tip="intensityTip('energy')"
        resource="energy" :intensity="intensities.energy" />
      <IntensityIcon
        v-tip="{text: 'Labor: Together with nature, the source of all things.', icon: 'labor'}"
        resource="labor" :intensity="2" />
      <IntensityIcon
        v-tip="intensityTip('water')"
        resource="water" :intensity="intensities.water" />
      <IntensityIcon
        v-tip="intensityTip('biodiversity')"
        resource="biodiversity" :intensity="intensities.biodiversity" />
      <IntensityIcon
        v-tip="intensityTip('land')"
        resource="land" :intensity="intensities.land" />
      <IntensityIcon
        v-tip="intensityTip('emissions')"
        resource="emissions" :intensity="intensities.emissions" />
    </div>
  </template>
  <template v-slot:back>
    <p>{{description}}</p>
    <p>This process currently produces {{produced.amount}}{{consts.icons[output]}} and {{produced.emissions}}{{consts.icons['emissions']}} per year.</p>
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
import Card from './Card.vue';
import state from '/src/state';
import consts from '/src/consts';
import display from 'lib/display';
import assets from 'components/assets';
import IntensityIcon from './IntensityIcon.vue';

const changeIcons = {
  'remain steady': 'steady',
  'expand': 'improve',
  'contract': 'worsen',
};

const intensities = {
  'land': {
    'energy': [0, 0.001, 0.01, 0.1],
    'calories': [0, 0.001, 0.002, 0.01],
  },
  'labor': {
    'energy': [0, 0.001, 0.01, 0.1], // TODO
    'calories': [0, 0.001, 0.002, 0.01], // TODO
  },
  'energy': {
    'energy': [0, 0.001, 0.01, 0.1], // TODO EROI
    'calories': [0, 0.00015, 0.0005, 0.001],
  },
  'water': {
    'energy': [0, 1, 2, 5],
    'calories': [0, 1, 2, 3],
  },
  'emissions': {
    'energy': [-2000, 0, 200, 800],
    'calories': [-1, 0, 0.5, 1],
  },
  'biodiversity': {
    'energy': [0, 1, 2, 3],
    'calories': [0, 1, 2, 3],
  }
};

function intensity(val, key, type) {
  let stops = intensities[key][type];
  for (let i = 0; i < stops.length - 1; i++) {
    if (val >= stops[i] && val < stops[i+1]) {
      return i+1;
    }
  }
  return stops.length;
}

const totalLand = 104e12;

export default {
  props: ['process'],
  components: {
    Card,
    IntensityIcon,
  },
  created() {
    this.changeIcons = changeIcons;
  },
  data() {
    return {
      state,
      ...this.process,
      ...state.processes[this.process.id],
      output: consts.outputs.keys[this.process.output],
    };
  },
  computed: {
    produced() {
      let baseAmount = state.gameState.produced_by_process[this.id];
      let amount = display.output(baseAmount, this.output);
      amount = amount > 0 ? Math.max(amount, 1) : amount;

      let emissions = display.gtco2eq(this.byproducts, baseAmount);
      emissions = emissions > 0 ? Math.max(emissions, 1) : emissions;
      return {
        emissions,
        amount
      };
    },
    feedstockIcon() {
      return display.enumToSlug(this.feedstock[0]);
    },
    feedstockName() {
      return display.enumToDisplay(this.feedstock[0]);
    },
    intensities() {
      let type =
        (this.output == 'electricity' || this.output == 'fuel')
        ? 'energy' : 'calories';
      let values = {
        emissions: display.co2eq(this.byproducts),
        biodiversity: this.byproducts.biodiversity,
        energy: this.resources.electricity + this.resources.fuel,
        land: this.resources.land,
        water: this.resources.water,
        // TODO labor
      };
      let intensities = Object.keys(values).reduce((acc, k) => {
        acc[k] = intensity(values[k], k, type);
        return acc;
      }, {});
      return intensities;
    },
    expectedChange() {
      // Kind of annoying, but grab this way
      // for reactivity
      let process = this.state.gameState.processes[this.id];
      if (process.status == 'Banned' && process.mix_share > 0) {
        return 'contract';
      } else if (process.status == 'Promoted') {
        return 'expand';
      } else {
        switch (process.change) {
          case 'Neutral': return 'remain steady';
          case 'Expanding': return 'expand';
          case 'Contracting': return 'contract';
        }
      }
    },
  },
  methods: {
    intensityTip(type) {
      switch (type) {
        case 'land': {
          let amount = display.landUsePercent(state.gameState.resources_demand.land);
          let rankings = state.gameState.resourceRankings['land'];
          return {
            icon: 'land',
            text: `Land: They're not making anymore of it. You're using ${amount.toFixed(0)}% of land.`,
            card: {
              type: 'Resource',
              data: {
                icon: 'land',
                name: 'Top Users',
                rankings,
                format: (v) => `${(v/totalLand*100).toFixed(0)}%`,
                current: this.process,
              }
            }
          }
        }
        case 'emissions': {
          let amount = state.gameState.emissions;
          let rankings = state.gameState.resourceRankings['emissions'];
          return {
            icon: 'emissions',
            text: `Emissions: A shroud around the earth. You're emitting ${amount.toFixed(1)}Gt per year.`,
            card: {
              type: 'Resource',
              data: {
                icon: 'emissions',
                name: 'Top Emitters',
                rankings,
                format: (v) => `${(v * 1e-15).toFixed(1)}Gt`,
                current: this.process,
              }
            }
          }
        }
        case 'water': {
          const totalWater = 45500000000000000.0;
          let amount = state.gameState.resources_demand.water/totalWater * 100;
          let rankings = state.gameState.resourceRankings['water'];
          return {
            icon: 'water',
            text: `Water: The giver of life. You're using ${amount.toFixed(0)}% of water resources.`,
            card: {
              type: 'Resource',
              data: {
                icon: 'water',
                name: 'Top Users',
                rankings,
                format: (v) => `${(v/totalWater*100).toFixed(0)}%`,
                current: this.process,
              }
            }
          }
        }
        case 'energy': {
          let amount = (state.gameState.output_demand.fuel + state.gameState.output_demand.electricity) * 1e-9;
          let rankings = state.gameState.resourceRankings['energy'];
          return {
            icon: 'energy',
            text: `Energy: Something something. You're using ${amount.toFixed(0)}TWh of energy.`,
            card: {
              type: 'Resource',
              data: {
                icon: 'energy',
                name: 'Top Users',
                rankings,
                format: (v) => `${(v*1e-9).toFixed(1)}TWh`,
                current: this.process,
              }
            }
          }
        }
        case 'biodiversity': {
          let amount = state.gameState.world.extinction_rate;
          let rankings = state.gameState.resourceRankings['biodiversity'];
          return {
            icon: 'biodiversity',
            text: `Biodiversity: The co-inhabitants of the planet. The current biodiversity threat index is ${amount.toFixed(0)}.`,
            card: {
              type: 'Resource',
              data: {
                icon: 'biodiversity',
                name: 'Top Threats',
                rankings,
                format: (v) => `${v.toFixed(0)}`,
                current: this.process,
              }
            }
          }
        }
      }
    }
  }
}
</script>

<style>
.process-intensity {
  display: flex;
  justify-content: space-evenly;
  margin: 0.5em 0;
}

.process-trend,
.process-feedstock {
  width: 24px;
  background: #222;
  border-radius: 10em;
  position: absolute;
  top: 0.5em;
  padding: 0.35em 0.2em;
  border: 1px solid #888;
}
.process-trend {
  right: 0.5em;
}
.process-feedstock {
  left: 0.5em;
  padding: 0.2em 0.2em;
}
</style>
