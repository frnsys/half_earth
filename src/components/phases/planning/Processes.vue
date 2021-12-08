<template>
<div class="planning--page">
  <Cards>
    <ProcessCard v-for="p in processes" :process="p">
      <template v-slot:actions>
        <button :disabled="changedMixShare(p) === 0" @click="removePoint(p)">
          -<img class="pip" :src="icons.mix_token">
        </button>
        <button :disabled="points === 0" @click="addPoint(p)">
          +<img class="pip" :src="icons.mix_token">
        </button>
      </template>
    </ProcessCard>
  </Cards>
  <div>
    <div class="available-mix-tokens">
        <img v-for="_ in points" class="pip" :src="icons.mix_token">
    </div>
    <div class="process-mix-change-notice" v-if="hasChanges">These changes will take one planning cycle to take effect.</div>
    <div class="production--demand planning--demand">
      <div v-for="v, k in demand" v-tip="{text: `Global demand for ${k}.`, icon: k}">
        {{demand[k]}}<img :src="icons[k]">
      </div>
      <div v-tip="{text: 'Global CO2eq emissions.', icon: 'emissions'}">{{emissions}}<img :src="icons.emissions"></div>
    </div>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import display from 'lib/display';
import Cards from './Cards.vue';
import ProcessCard from 'components/cards/ProcessCard.vue';

export default {
  props: ['output'],
  components: {
    Cards,
    ProcessCard,
  },
  data() {
    return {
      state,
      points: 0,
    };
  },
  computed: {
    processes() {
      let processes = state.gameState.processes.filter((p) => !p.locked && p.output === this.output);
      processes.sort((a, b) => a.name.toLowerCase().localeCompare(b.name.toLowerCase()))
      return processes;
    },
    demand() {
      return display.outputs(state.gameState.output_demand);
    },
    emissions() {
      return display.gtco2eq(state.gameState.byproducts);
    },
    hasChanges() {
      return Object.values(state.processMixChanges[this.output]).filter((change) => change != 0).length > 0;
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
        /* game.changeProcessMixShare(p.id, -1); */
        this.points += 1;
        state.processMixChanges[this.output][p.id] = change - 1;
        this.$emit('allowBack', false);
      }
    },
    addPoint(p) {
      if (this.points > 0) {
        /* game.changeProcessMixShare(p.id, 1); */
        let change = state.processMixChanges[this.output][p.id] || 0;
        this.points -= 1;
        state.processMixChanges[this.output][p.id] = change + 1;
        if (this.points == 0) {
          this.$emit('allowBack', true);
        }
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
  margin: 0.5em 1em 0 1em;
  text-align: center;
}
</style>
