<template>
<div class="planning--page">
  <Cards>
    <ProcessCard v-for="p in processes" :process="p">
      <template v-slot:actions>
        <button v-if="p.status == 'Neutral'" @click="banProcess(p)">
          &lt; Ban
          <div class="card--action--cost">
            {{banProcessCost(p)}}<img class="pip" :src="icons.political_capital">
          </div>
        </button>
        <button v-if="p.status == 'Banned'" @click="unbanProcess(p)">
          Unban &gt;
          <div class="card--action--cost">
            {{promoteProcessCost(p)}}<img class="pip" :src="icons.political_capital">
          </div>
        </button>
        <button v-if="p.status == 'Neutral'" @click="promoteProcess(p)">
          Promote &gt;
          <div class="card--action--cost">
            {{promoteProcessCost(p)}}<img class="pip" :src="icons.political_capital">
          </div>
        </button>
        <button v-if="p.status == 'Promoted'" @click="unpromoteProcess(p)">
          &lt; Unpromote
          <div class="card--action--cost">
            {{banProcessCost(p)}}<img class="pip" :src="icons.political_capital">
          </div>
        </button>
      </template>
    </ProcessCard>
  </Cards>
  <div class="production--demand planning--demand">
    <div v-for="v, k in demand" v-tip="{text: `Global demand for ${k}.`, icon: k}">
      {{demand[k]}}<img :src="icons[k]">
    </div>
    <div v-tip="{text: 'Global CO2eq emissions.', icon: 'emissions'}">{{emissions}}<img :src="icons.emissions"></div>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import costs from 'lib/costs';
import display from 'lib/display';
import Cards from './Cards.vue';
import ProcessCard from 'components/cards/ProcessCard.vue';

export default {
  components: {
    Cards,
    ProcessCard,
  },
  data() {
    return {
      state
    };
  },
  computed: {
    processes() {
      let processes = state.gameState.processes.filter((p) => !p.locked);
      processes.sort((a, b) => a.name.toLowerCase().localeCompare(b.name.toLowerCase()))
      return processes;
    },
    demand() {
      return display.outputs(state.gameState.output_demand);
    },
    emissions() {
      return display.gtco2eq(state.gameState.byproducts);
    }
  },
  methods: {
    banProcessCost(p) {
      return costs.banProcessCost(p);
    },
    promoteProcessCost(p) {
      return costs.promoteProcessCost(p);
    },
    banProcess(p) {
      let cost = this.banProcessCost(p);
      if (!p.banned && state.gameState.political_capital >= cost) {
        game.changePoliticalCapital(-cost);
        game.banProcess(p.id);
      }
    },
    unbanProcess(p) {
      let cost = this.promoteProcessCost(p);
      if (p.banned && state.gameState.political_capital >= cost) {
        game.changePoliticalCapital(-cost);
        game.unbanProcess(p.id);
      }
    },
    promoteProcess(p) {
      let cost = this.promoteProcessCost(p);
      if (!p.banned && state.gameState.political_capital >= cost) {
        game.changePoliticalCapital(-cost);
        game.promoteProcess(p.id);
      }
    },
    unpromoteProcess(p) {
      let cost = this.banProcessCost(p);
      if (!p.banned && state.gameState.political_capital >= cost) {
        game.changePoliticalCapital(-cost);
        game.unpromoteProcess(p.id);
      }
    },
  }
}
</script>
