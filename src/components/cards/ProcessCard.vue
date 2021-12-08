<template>
<Card>
  <template v-slot:header>
    <div>{{name}}</div>
    <div v-tip="{text: `This process currently produces ${produced.amount}<img src='${icons[output]}'> and ${produced.emissions}<img src='${icons.emissions}'> per year.`, icon: output}">{{produced.amount}}<img :src="icons[output]"> {{produced.emissions}}<img :src="icons.emissions"></div>
  </template>
  <template v-slot:figure>
    <img class="card-image" :src="`/assets/content/images/${image.fname}`" />
    <div class="card-tack-ur process-mix"
      v-tip="{text: `This process is makes up ${process.mix_share*5}% of ${output} production.${hasChange ? ` At the next planning cycle it will change to ${changedMixShare*5}%.` : '' }`, icon: 'mix_token'}">
      <div class="process-mix-percents" :class="{depleted: feedstockEstimate == 0}">
        <div class="process-mix-percent">{{process.mix_share*5}}%</div>
        <template v-if="hasChange">
          <div><img :src="icons.down_arrow"></div>
          <div class="process-mix-percent">{{changedMixShare*5}}%</div>
        </template>
      </div>
      <div class="process-mix-cells">
        <div class="process-mix-cell" v-for="i in 20" :class="{
          active: i <= process.mix_share,
          depleted: feedstockEstimate == 0,
          shrink: i <= process.mix_share && i > changedMixShare,
          grow: i > process.mix_share && i <= changedMixShare
        }"/>
      </div>
    </div>
    <div class="card-tack-ul">
      <img v-if="feedstockEstimate && feedstockEstimate == 0" :src="icons.halted" class="alert-icon" />
      <img v-else-if="feedstockEstimate && feedstockEstimate < 20" :src="icons.alert" class="alert-icon" />
      <img v-if="feedstockName != 'other'"
        v-tip="{text: `This process uses ${feedstockName}.${feedstockEstimate ? (feedstockEstimate == 0 ? ` This feedstock is depleted, so this process is stopped. You should reallocate its points to other processes.` : ` At current usage rates the estimate supply is expected to last ${feedstockEstimate} years.`) : ''}`, icon: feedstockIcon}"
        class="process-feedstock" :src="icons[feedstockIcon]">
      </div>
    <div class="opposers" v-if="opposersDetailed.length > 0">
      <div>Nay</div>
      <div>
        <img v-for="npc in opposersDetailed" v-tip="{text: `${npc.name} is opposed to this. If you ban it, your relationship will improve by +<img src='${icons.relationship}' />.`, icon: npc.name}" :src="icons[npc.name]">
      </div>
    </div>
    <div class="supporters" v-if="supportersDetailed.length > 0">
      <div>Yea</div>
      <div>
        <img v-for="npc in supportersDetailed" v-tip="{text: `${npc.name} supports this. If you implement it, your relationship will improve by +<img src='${icons.relationship}' />.`, icon: npc.name}" :src="icons[npc.name]">
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
    <p>This process currently produces {{produced.amount}}<img :src="icons[output]" /> and {{produced.emissions}}<img :src="icons.emissions" /> per year.</p>
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
import NPCS from '/assets/content/npcs.json';

export default {
  props: ['process'],
  components: {
    Card,
    IntensityIcon,
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
    feedstockEstimate() {
      let feedstock = display.enumKey(this.feedstock[0]);
      if (feedstock == 'other' || feedstock == 'soil') {
        return null;
      }
      let estimate = state.gameState.feedstocks[feedstock]/state.gameState.consumed_feedstocks[feedstock];
      return Math.round(estimate);
    },
    hasChange() {
      let change = state.processMixChanges[this.process.output][this.process.id] || 0;
      return change !== 0;
    },
    changedMixShare() {
      let change = state.processMixChanges[this.process.output][this.process.id] || 0;
      return this.process.mix_share + change;
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
    supportersDetailed() {
      return this.supporters
        .filter((id) => !state.gameState.npcs[id].locked)
        .map((id) => NPCS[id]);
    },
    opposersDetailed() {
      return this.opposers
        .filter((id) => !state.gameState.npcs[id].locked)
        .map((id) => NPCS[id]);
    },
  },
  methods: {
    intensityTip(type) {
      switch (type) {
        case 'land': {
          let amount = display.landUsePercent(state.gameState.resources_demand.land);
          let rankings = state.resourceRankings['land'];
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
          let rankings = state.resourceRankings['emissions'];
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
          let rankings = state.resourceRankings['water'];
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
          let rankings = state.resourceRankings['energy'];
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
          let rankings = state.resourceRankings['biodiversity'];
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

.process-mix {
  display: flex;
}
.process-mix img {
  width: 18px;
  vertical-align: top;
}

.process-mix-percent {
  background: #222;
  color: #fff;
  padding: 0.1em 0.15em;
  border-radius: 0.2em;
  font-size: 0.8em;
}
.process-mix-percents {
  text-align: center;
}
.process-mix-percents.depleted {
  color: #aaa;
}
.process-mix-cell {
  height: 6px;
  width: 6px;
  background: #222;
  margin: 1px;
  border: 1px solid #222;
}
.process-mix-cell.active {
  background: #1B97F3;
}
.process-mix-cell.active.depleted {
  background: #6190B3;
}
.process-mix-cell.active.shrink {
  background: #F28435;
}
.process-mix-cell.grow {
  background: #43CC70;
}

.alert-icon {
	position: absolute;
	width: 16px;
	right: 0;
	bottom: 0;
	transform: translate(50%, 0);
}
</style>
