<template>
<Card>
  <template v-slot:header>
    <div>{{name}}</div>
    <div v-tip="{text: `This process currently produces ${produced.amount}${icons[output]} and ${produced.emissions}${icons.emissions} per year.`, icon: output}">{{produced.amount}}<img :src="icons[output]"> {{produced.emissions}}<img :src="icons.emissions"></div>
  </template>
  <template v-slot:figure>
    <img class="card-image" :src="`/assets/content/images/${image.fname}`" />
    <img
      v-tip="{text: `This process is expected to ${expectedChange}.`, icon: changeIcons[expectedChange]}"
      class="process-trend card-tack-ur" :src="icons[changeIcons[expectedChange]]">
    <img
      v-tip="{text: `This process uses ${feedstockName}.`, icon: feedstockIcon}"
      class="process-feedstock card-tack-ul" :src="icons[feedstockIcon]">
    <div class="opposers">
      <div>Nay</div>
      <div>
        <img v-tip="{text: `The Authoritarian is opposed to this. If you ban it, your relationship will improve by +<img src='${icons.relationship}' />.`, icon: 'authoritarian'}" src="/assets/characters/The Authoritarian.png">
        <img v-tip="{text: `The Economist is opposed to this process. If you ban it, your relationship will improve by +<img src='${icons.relationship}' />.`, icon: 'economist'}" src="/assets/characters/The Economist.png">
        <img v-tip="{text: `The Technocrat is opposed to this process. If you ban it, your relationship will improve by +<img src='${icons.relationship}' />.`, icon: 'technocrat'}" src="/assets/characters/The Technocrat.png">
      </div>
    </div>
    <div class="supporters">
      <div>Yea</div>
      <div>
        <img v-tip="{text: `The Scientist supports this. If you promote it, your relationship will improve by +<img src='${icons.relationship}' />.`, icon: 'scientist'}" src="/assets/characters/The Scientist.png">
        <img v-tip="{text: `The Populist supports this. If you promote it, your relationship will improve by +<img src='${icons.relationship}' />.`, icon: 'populist'}" src="/assets/characters/The Populist.png">
        <img v-tip="{text: `The Ecologist supports this. If you promote it, your relationship will improve by +<img src='${icons.relationship}' />.`, icon: 'ecologist'}" src="/assets/characters/The Ecologist.png">
      </div>
    </div>
  </template>
  <template v-slot:body>
    <div class="card-actions" v-if="!!this.$slots.actions">
      <slot name="actions"></slot>
    </div>
    <div class="process-intensity space-even">
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
    <p>This process currently produces {{produced.amount}}{{icons[output]}} and {{produced.emissions}}{{icons.emissions}} per year.</p>
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
import game from '/src/game';
import state from '/src/state';
import display from 'lib/display';
import IntensityIcon from './IntensityIcon.vue';
import PROCESSES from '/assets/content/processes.json';

const changeIcons = {
  'remain steady': 'steady',
  'expand': 'improve',
  'contract': 'worsen',
};

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
      ...PROCESSES[this.process.id],
      output: display.enumKey(this.process.output),
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
      return display.enumKey(this.feedstock[0]);
    },
    feedstockName() {
      return display.enumDisplay(this.feedstock[0]);
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
        acc[k] = display.intensity(values[k], k, type);
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
                current: this.process,
              }
            }
          }
        }
        case 'water': {
          let amount = display.waterUsePercent(state.gameState.resources_demand.water);
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
  margin: 0.5em 0;
}

.process-trend,
.process-feedstock {
  width: 24px;
  background: #222;
  border-radius: 10em;
  padding: 0.35em 0.2em;
  border: 1px solid #888;
}
.process-feedstock {
  padding: 0.2em 0.2em;
}
</style>