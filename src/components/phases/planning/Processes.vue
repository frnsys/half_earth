<template>
<div class="plan-change-select planning--page">
  <div class="planning--page-tabs">
  <div class="unspent-warning" v-if="!allowBack">There are still unused production points!</div>
   <div class="planning-sub-tab"  @click="if (allowBack) { output = 'Electricity'; }" :class="{selected: output == 'Electricity', disabled: !allowBack}">
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
    <div :class="{disabled: !allowBack || backDisabled, highlight: backHighlighted}" @click="if (allowBack) { $emit('close'); }">Back</div>
  </div>

  <div class="available-mix-tokens">
    <div class="mix-token" v-for="_ in points" v-tip="{icon : 'mix_token', text: `One production point represents 5% of an entire production sector's productive capacity.`}"></div>
  </div>

  <AddScanner ref="addScanner" :points="points" :process="process" :addPoint="addPoint" />

  <Cards @focused="onFocus" @scrollStart="onScrollStart" @scrollEnd="onScrollEnd" :disabled="!allowScroll">
    <Draggable
      @drag="onDrag"
      @dragStop="onDragStop"
      v-for="p in processes"
      :yBounds="yBounds"
      :draggable="allowSwipe && focused == p"
      :id="p.id"
      :key="p.id"
    >
    <ProcessCard :process="p" >

    </ProcessCard>
    </Draggable>
  </Cards>

  <CardFocusArea />

  <RemoveScanner ref="removeScanner" :process="process" :removePoint="removePoint" />

  <div>
    <div class="process-mix-change-notice-wrapper" v-if="hasChanges">
      <div class="process-mix-change-notice" >
      <div>These changes will take <strong>{{changesTime}} planning cycle{{changesTime > 1 ? 's' : ''}}</strong> to take effect.</div>
      <div v-html="estimatedChanges"></div>
      </div>
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
import consts from '/src/consts.js';
import format from '/src/display/format';
import display from '/src/display/display';
import ProcessCard from 'components/cards/ProcessCard.vue';
import tutorial from '/src/tutorial';

import CardsMixin from 'components/phases/CardsMixin';
import AddScanner from 'components/scanner/process/AddScanner.vue';
import RemoveScanner from 'components/scanner/process/RemoveScanner.vue';

const lf = new Intl.ListFormat('en');

function fmtPercent(n) {
  return n.toLocaleString(undefined, {maximumFractionDigits: 1});
}

export default {
  mixins: [CardsMixin],
  components: {
    ProcessCard,
    AddScanner,
    RemoveScanner,
  },
  data() {
    return {
      state,
      points: 0,
      output: 'Electricity',
      allowBack: true
    };
  },
  mounted(){
    this.$emit('page', this.output);
  },
  watch: {
    output(output) {
      this.updateFocused();
      this.$emit('page', this.output);
    }
  },
  computed: {
    backDisabled() {
      return state.tutorial < tutorial.PROCESSES_BACK;
    },
    backHighlighted() {
      return state.tutorial == tutorial.PROCESSES_BACK;
    },
    process() {
      if (this.focused !== null) {
        // console.log(this.focusedProcess.idx);
        // console.log(this.processes);
        let proc =  this.processes[this.focusedIdx];
        if (proc === undefined) {
          return this.processes[0];
        } else {
          return proc;
        }
      } else {
        // Default for loading
        return state.gameState.processes[0];
      }
    },
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
      // Total demand for each of these
      let before = {
        'emissions': state.gameState.world.emissions/1e-15, // convert from gtco2eq to g co2eq
        'energy use': state.gameState.output_demand.fuel + state.gameState.output_demand.electricity,
        'land use': state.gameState.resources_demand.land,
        'water use': state.gameState.resources_demand.water,
        'the extinction rate': state.gameState.world.extinction_rate,
      };

      // Demand for each of these just from the current set of processes
      let current = {
        'emissions': 0,
        'energy use': 0,
        'land use': 0,
        'water use': 0,
        'the extinction rate': 0,
      };
      this.processes.forEach((p) => {
        let mix_share = p.mix_share;
        let total = mix_share/20 * state.gameState.output_demand[display.enumKey(p.output)];
        current['land use'] += p.resources.land * total;
        current['water use'] += p.resources.water * total;
        current['energy use'] += (p.resources.electricity + p.resources.fuel) * total;
        current['emissions'] += format.co2eq(p.byproducts) * total;
        current['the extinction rate'] += p.byproducts.biodiversity * total;
      });

      // Changed demand for each of these, just for the current set of processes
      let changed = {
        'emissions': 0,
        'energy use': 0,
        'land use': 0,
        'water use': 0,
        'the extinction rate': 0,
      };
      this.processes.forEach((p) => {
        let mix_share = p.mix_share + (state.processMixChanges[this.output][p.id] || 0);
        let total = mix_share/20 * state.gameState.output_demand[display.enumKey(p.output)];
        changed['land use'] += p.resources.land * total;
        changed['water use'] += p.resources.water * total;
        changed['energy use'] += (p.resources.electricity + p.resources.fuel) * total;
        changed['emissions'] += format.co2eq(p.byproducts) * total;
        changed['the extinction rate'] += p.byproducts.biodiversity * total;
      });

      // Changed overall/total/global demand for each of these
      let after = {};
      Object.keys(before).forEach((k) => {
        // Subtract out previous process demand, then add in changed process demand
        after[k] = before[k] - current[k] + changed[k];
      });

      let descs = Object.keys(before).map((k) => {
        let change = 0;
        if (before[k] == 0) {
          if (after[k] > 0) {
            change = 1;
          } else if (after[k] < 0) {
            change = -1;
          } else {
            change = 0;
          }
        } else {
          change = (after[k] - before[k])/before[k];
        }
        change = Math.round(change * 100);
        if (change > 0.0) {
          return `<span class="change-increase"><strong>increase ${k} by ${change > 100 ? '⚠️' : ''}${fmtPercent(change)}%</strong></span>`;
        } else if (change < 0.0) {
          return `<span class="change-decrease"><strong>decrease ${k} by ${fmtPercent(Math.abs(change))}%</strong></span>`;
        } else {
          return null;
        }
      }).filter((desc) => desc !== null);

      if (descs.length == 0) {
        return `They won't have much effect.`;
      } else {
        return `This output's production will: ${lf.format(descs)}.`;
      }
    },
  },
  methods: {
    onFocus(idx) {
      this.onFocused(idx);
      if (!state.viewed.includes(this.process.ref_id)) {
        state.viewed.push(this.process.ref_id);
      }
    },
    yBounds() {
      return [
        this.$refs.addScanner.botY - 10,
        this.$refs.removeScanner.topY + 10 - this.cardHeight,
      ];
    },
    items(idx) {
      return this.processes[idx];
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
        if (change + 1 <= game.processMaxShare(p)) {
          this.points -= 1;
          state.processMixChanges[this.output][p.id] = change + 1;
          if (this.points == 0) {
            this.allowBack = true;
          }
        }
      }

      // Consider the process mix 'changed'
      // when all points have been assigned
      if (this.points == 0) {
        this.$emit('change');
      }
    },
  }
}
</script>

<style>
.available-mix-tokens {
  height: 24px;
  text-align: center;
  /* margin-top: -50px; */
  width: 100%;
  margin: 0 auto;
  margin-top: 5px;
  z-index: 5;
  position: absolute;
  top:72px;
}

.mix-token{
  height: 20px;
  width: 18px;
  background-color: #1B97F3;
  display: inline-block;
  box-shadow: inset -1px -1px 0px rgb(0 0 0 / 50%);
  border-left: 1px solid rgba(255,255,255,0.5);
  border-top: 1px solid rgba(255,255,255,0.5);

}

.mix-token:first-of-type {
  border-top-left-radius: 0.3em;
  border-bottom-left-radius: 0.3em;
}
.mix-token:last-of-type{
  border-top-right-radius: 0.3em;
  border-bottom-right-radius: 0.3em;
}

.process-mix-change-notice-wrapper{
  width: 100%;
  position: absolute;
  bottom: 50px;
}

.process-mix-change-notice {
  font-family: 'Inter', sans-serif;
  font-size: 0.7em;
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

.unspent-warning{
  position: absolute;
  width: 100%;
  height: 100%;
  background-color: rgba(255,255,255,0.8);
  text-align: center;
  z-index: 1;
  font-family: 'Inter', sans-serif;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  font-size: 0.5em;
  font-weight: 600;
  display: flex;
  justify-content: center;
  flex-direction: column;
  border-radius: 0 0 0.6em 0.6em !important;
  border-right: none !important;
}
</style>
