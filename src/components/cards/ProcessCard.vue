<template>
<Card>
  <template v-slot:header>
    <div>{{name}}</div>
    <div v-tip="outputTip">{{produced.amount}}<img :src="icons[output]"> {{produced.emissions}}<img :src="icons.emissions"></div>
  </template>
  <template v-slot:figure>
    <img class="card-image" :src="`/assets/content/images/${image.fname}`" />
    <div class="card-tack-ur process-mix"
      v-tip="changeTip">
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
    <div class="card-tack-ul process-details">
      <div>
        <img v-if="feedstockEstimate && feedstockEstimate == 0" :src="icons.halted" class="alert-icon" />
        <img v-else-if="feedstockEstimate && feedstockEstimate < 20" :src="icons.alert" class="alert-icon" />
        <img v-if="feedstockName != 'other'"
          v-tip="{text: `This process uses ${feedstockName}. ${feedstockEstimateDesc}`, icon: feedstockIcon}"
          class="process-feedstock" :src="icons[feedstockIcon]">
        <div class="feedstock-remaining" v-if="feedstockName != 'other' && feedstockName != 'soil'">
          <div :class="`feedstock-remaining-fill feedstock-remaining-fill--${feedstockLevel}`"></div>
        </div>
      </div>
      <div>
        <img class="process--feature" v-for="feature in featureIcons" :src="icons[feature.icon]" v-tip="{icon: feature.icon, text: feature.text}"/>
      </div>
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
import format from '/src/display/format';
import factors from '/src/display/factors';
import display from '/src/display/display';
import intensity from '/src/display/intensity';
import IntensityIcon from './IntensityIcon.vue';
import PROCESSES from '/assets/content/processes.json';
import NPCS from '/assets/content/npcs.json';
import icons from '/src/components/icons';

const FEATURE_DESCS = {
  'IsSolar': 'This process relies on the sun.',
  'IsIntermittent': 'This process is intermittent.',
  'IsNuclear': 'This process is nuclear.',
  'IsCombustion': 'This process involves combustion.',
  'IsFossil': 'This process uses fossil fuels.',
  'IsCCS': 'This process captures and stores carbon.',
  'UsesLivestock': 'This process uses livestock.',
  'UsesPesticides': 'This process use pesticides.',
  'UsesSynFertilizer': 'This process uses synthetic fertilizers.',
};

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
    outputTip() {
      return {
        icon: this.output,
        text: `This process currently produces ${this.produced.amount}<img src='${icons[this.output]}'> and ${this.produced.emissions}<img src='${icons.emissions}'> per year.`
      }
    },
    changeTip() {
      return {
        icon: 'mix_token',
        text: `This process currently makes up ${this.process.mix_share*5}% of ${this.output} production.`
      };
    },
    featureIcons() {
      return this.features.map((feat) => {
        return {
          icon: feat,
          text: FEATURE_DESCS[feat]
        };
      });
    },
    produced() {
      let baseAmount = state.gameState.produced_by_process[this.id];
      let amount = format.output(baseAmount, this.output);
      amount = amount > 0 ? Math.max(amount, 1) : amount;

      let emissions = format.gtco2eq(this.byproducts, baseAmount);
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
    feedstockEstimateDesc() {
      if (this.feedstockEstimate == null) {
        return '';
      } else if (this.feedstockEstimate == 0) {
        return 'This feedstock is depleted, so this process is stopped. You should reallocate its points to other processes.';
      } else if (isFinite(this.feedstockEstimate)) {
        return `At current usage rates the estimated supply is expected to last ${this.feedstockEstimate} years.`;
      } else {
        return `At current usage rates the estimated supply is expected to last indefinitely.`;
      }
    },
    feedstockLevel() {
      let feedstock = display.enumKey(this.feedstock[0]);
      if (feedstock == 'other' || feedstock == 'soil') {
        return 'high';
      } else if (this.feedstockEstimate < 20) {
        return 'low';
      } else if (this.feedstockEstimate < 50) {
        return 'mid';
      } else if (this.feedstockEstimate < 80) {
        return 'high';
      } else {
        return 'very-high';
      }
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
        emissions: format.co2eq(this.byproducts),
        biodiversity: this.byproducts.biodiversity,
        energy: this.resources.electricity + this.resources.fuel,
        land: this.resources.land,
        water: this.resources.water,
        // TODO labor
      };
      let intensities = Object.keys(values).reduce((acc, k) => {
        acc[k] = intensity.intensity(values[k], k, type);
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
          let amount = format.landUsePercent(state.gameState.resources_demand.land);
          return factors.tips.land(
            `Land: They're not making anymore of it. You're using ${amount.toFixed(0)}% of land.`,
            this.process);
        }
        case 'emissions': {
          let amount = state.gameState.emissions;
          return factors.tips.emissions(
            `Emissions: A shroud around the earth. You're emitting ${amount.toFixed(1)}Gt per year.`,
            this.process);
        }
        case 'water': {
          let amount = format.waterUsePercent(state.gameState.resources_demand.water);
          return factors.tips.water(
            `Water: The giver of life. You're using ${amount.toFixed(0)}% of water resources.`,
            this.process);
        }
        case 'energy': {
          let amount = (state.gameState.output_demand.fuel + state.gameState.output_demand.electricity) * 1e-9;
          return factors.tips.energy(
            `Energy: Something something. You're using ${amount.toFixed(0)}TWh of energy.`,
            this.process);
        }
        case 'biodiversity': {
          let amount = state.gameState.world.extinction_rate;
          return factors.tips.biodiversity(
            `Biodiversity: The co-inhabitants of the planet. The current biodiversity threat index is ${amount.toFixed(0)}.`,
            this.process);
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

.process-details {
  display: flex;
}

.feedstock-remaining {
  height: 5px;
  background: #aaa;
  width: 24px;
  border-radius: 1em;
  outline: 1px solid #555;
  margin-top: -2px;
  overflow: hidden;
}
.feedstock-remaining-fill {
  height: 100%;
}
.feedstock-remaining-fill--low {
  background: #EF3838;
  width: 20%;
}
.feedstock-remaining-fill--mid {
  background: #FBC011;
  width: 50%;
}
.feedstock-remaining-fill--high {
  background: #43CC70;
  width: 80%;
}
.feedstock-remaining-fill--very-high {
  background: #43CC70;
  width: 95%;
}

.process--feature {
  height: 24px;
  background: #222;
  border-radius: 1.2em;
  padding: 0.2em;
  margin-left: 2px;
  border: 1px solid #888;
}

</style>
