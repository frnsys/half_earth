<template>
<div class="plan-change-select planning--page">
  <div class="planning--page-tabs">
   <div class="planning-sub-tab" @click="if (allowBack) { output = 'Electricity'; }" :class="{selected: output == 'Electricity', disabled: !allowBack}">
      <img :src="icons.electricity" />
      <div>Electricity</div>
    </div>
    <div class="planning-sub-tab" @click="if (allowBack) { output = 'Fuel'; }" :class="{selected: output == 'Fuel', disabled: !allowBack}">
      <img :src="icons.fuel" />
      <div>Fuel</div>
    </div>
    <div class="planning-sub-tab" @click="if (allowBack) { output = 'PlantCalories'; }" :class="{selected: output == 'PlantCalories', disabled: !allowBack}">
      <img :src="icons.plant_calories" />
      <div>Crops</div>
    </div>
    <div class="planning-sub-tab" @click="if (allowBack) { output = 'AnimalCalories'; }" :class="{selected: output == 'AnimalCalories', disabled: !allowBack}">
      <img :src="icons.animal_calories" />
      <div>Livestock</div>
    </div>
    <div :class="{disabled: !allowBack}" @click="if (allowBack) { $emit('close'); }">Back</div>
  </div>

  <Cards>
    <ProcessCard v-for="p in processes" :process="p" :key="p.id">
      <template v-slot:actions>
        <button :disabled="changedMixShare(p) === 0" @click="(ev) => { ev.stopPropagation(); removePoint(p) }">
          -<img class="pip" :src="icons.mix_token">
        </button>
        <button :disabled="points === 0" @click="(ev) => { ev.stopPropagation(); addPoint(p) }">
          +<img class="pip" :src="icons.mix_token">
        </button>
      </template>
    </ProcessCard>
  </Cards>

  <CardFocusArea />

  <div>
    <div class="available-mix-tokens">
        <img v-for="_ in points" class="pip" :src="icons.mix_token">
    </div>
    <div class="process-mix-change-notice" v-if="hasChanges">
      <div>These changes will take {{changesTime}} planning cycle{{changesTime > 1 ? 's' : ''}} to take effect.</div>
      <div v-html="estimatedChanges"></div>
    </div>
    <div class="production--demand planning--demand">
      <div v-for="v, k in demand" v-tip="factors.tips[k](`Global demand for ${display.enumDisplay(k)}.`)">
        {{demand[k]}}<img :src="icons[k]">
      </div>
      <div v-tip="factors.tips.emissions('Current annual emissions, in gigatonnes of CO2 equivalent.')">{{emissions}}<img :src="icons.emissions"></div>
    </div>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import Cards from 'components/cards/Cards.vue';
import consts from '/src/consts.js';
import format from '/src/display/format';
import ProcessCard from 'components/cards/ProcessCard.vue';
import CardFocusArea from 'components/cards/CardFocusArea.vue';

const lf = new Intl.ListFormat('en');

function fmtPercent(n) {
  return n.toLocaleString(undefined, {maximumFractionDigits: 1});
}

export default {
  components: {
    Cards,
    ProcessCard,
    CardFocusArea
  },
  data() {
    return {
      state,
      points: 0,
      output: 'Electricity',

      allowBack: true
    };
  },
  computed: {
    processes() {
      let processes = state.gameState.processes.filter((p) => !p.locked && p.output === this.output);
      processes.sort((a, b) => a.name.toLowerCase().localeCompare(b.name.toLowerCase()))
      return processes;
    },
    demand() {
      return format.outputs(state.gameState.output_demand);
    },
    emissions() {
      return format.gtco2eq(state.gameState.byproducts);
    },
    hasChanges() {
      return Object.values(state.processMixChanges[this.output]).filter((change) => change != 0).length > 0;
    },
    changesTime() {
      return Math.ceil(this.changingPoints/consts.processPointsPerCycle);
    },
    changingPoints() {
      return Math.ceil(Object.values(state.processMixChanges[this.output]).reduce((acc, change) => {
        return acc + Math.abs(change);
      }, 0)/2);
    },
    estimatedChanges() {
      if (this.points !== 0) return '';

      let current = {
        'emissions': 0,
        'energy use': 0,
        'land use': 0,
        'water use': 0,
        'the extinction rate': 0,
      };
      this.processes.forEach((p) => {
        let mix_share = p.mix_share;
        current['land use'] += p.resources.land * mix_share;
        current['water use'] += p.resources.water * mix_share;
        current['energy use'] += (p.resources.electricity + p.resources.fuel) * mix_share;
        current['emissions'] += format.co2eq(p.byproducts) * mix_share;
        current['the extinction rate'] += p.byproducts.biodiversity * mix_share;
      });

      let changed = {
        'emissions': 0,
        'energy use': 0,
        'land use': 0,
        'water use': 0,
        'the extinction rate': 0,
      };
      this.processes.forEach((p) => {
        let mix_share = p.mix_share + (state.processMixChanges[this.output][p.id] || 0);
        changed['land use'] += p.resources.land * mix_share;
        changed['water use'] += p.resources.water * mix_share;
        changed['energy use'] += (p.resources.electricity + p.resources.fuel) * mix_share;
        changed['emissions'] += format.co2eq(p.byproducts) * mix_share;
        changed['the extinction rate'] += p.byproducts.biodiversity * mix_share;
      });

      let descs = Object.keys(current).map((k) => {
        let change = 0;
        if (current[k] == 0) {
          if (changed[k] > 0) {
            change = 1;
          } else if (changed[k] < 0) {
            change = -1;
          } else {
            change = 0;
          }
        } else {
          change = (changed[k] - current[k])/current[k];
        }
        change = Math.round(change * 100);
        if (change > 0.0) {
          return `<span class="change-increase">increase ${k} by ${change > 100 ? '⚠️' : ''}${fmtPercent(change)}%</span>`;
        } else if (change < 0.0) {
          return `<span class="change-decrease">decrease ${k} by ${fmtPercent(Math.abs(change))}%</span>`;
        } else {
          return null;
        }
      }).filter((desc) => desc !== null);

      if (descs.length == 0) {
        return `They won't have much effect.`;
      } else {
        return `This output's production will: ${lf.format(descs)}.`;
      }
    }
  },
  methods: {
    changedMixShare(p) {
      let change = state.processMixChanges[this.output][p.id] || 0;
      return p.mix_share + change;
    },
    removePoint(p) {
      let change = state.processMixChanges[this.output][p.id] || 0;
      if (p.mix_share + change > 0) {
        this.points += 1;
        state.processMixChanges[this.output][p.id] = change - 1;
        this.allowBack = false;
      }
    },
    addPoint(p) {
      if (this.points > 0) {
        let change = state.processMixChanges[this.output][p.id] || 0;
        this.points -= 1;
        state.processMixChanges[this.output][p.id] = change + 1;
        if (this.points == 0) {
          this.allowBack = true;
        }
      }

      // Consider the process mix 'changed'
      // when all points have been assigned
      if (this.points == 0) {
        this.$emit('change');
      }
    }
  }
}
</script>

<style>
.available-mix-tokens {
  height: 24px;
  text-align: center;
}

.process-mix-change-notice {
  font-size: 0.75em;
  background: #222;
  color: #fff;
  padding: 0.25em;
  border-radius: 0.2em;
  margin: 0.5em auto 0;
  text-align: center;
  max-width: 320px;
}

.plan-change-select header .disabled {
  opacity: 0.5;
}

.change-decrease {
  color: #2FE863;
}
.change-increase {
  color: #EF3838;
}
</style>
