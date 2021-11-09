<template>
<div class="planning--page">
  <Cards>
    <ProcessCard v-for="p in processes" :process="p">
      <template v-slot:actions>
        <button v-if="p.status == 'Neutral'" @click="banProcess(p)">
          &lt; Ban
          <div class="card--action--cost">
            {{banProcessCost(p)}}<img class="pip" src="/assets/icons/pips/political_capital.png">
          </div>
        </button>
        <button v-if="p.status == 'Banned'" @click="unbanProcess(p)">
          Unban &gt;
          <div class="card--action--cost">
            {{promoteProcessCost(p)}}<img class="pip" src="/assets/icons/pips/political_capital.png">
          </div>
        </button>
        <button v-if="p.status == 'Neutral'" @click="promoteProcess(p)">
          Promote &gt;
          <div class="card--action--cost">
            {{promoteProcessCost(p)}}<img class="pip" src="/assets/icons/pips/political_capital.png">
          </div>
        </button>
        <button v-if="p.status == 'Promoted'" @click="unpromoteProcess(p)">
          &lt; Unpromote
          <div class="card--action--cost">
            {{banProcessCost(p)}}<img class="pip" src="/assets/icons/pips/political_capital.png">
          </div>
        </button>
      </template>
    </ProcessCard>
  </Cards>
  <div class="production--demand planning--demand">
    <div v-for="v, k in demand" v-tip="{text: `Global demand for ${k}.`, icon: k}">
      {{demand[k]}}{{consts.icons[k]}}
    </div>
    <div v-tip="{text: 'Global CO2eq emissions.', icon: 'emissions'}">{{emissions}}{{consts.icons['emissions']}}</div>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import display from 'lib/display';
import Cards from './Cards.vue';
import ProcessCard from 'components/cards/ProcessCard.vue';
import {nearestMultiple} from '/src/lib/util';

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
      return state.gameState.processes.filter((p) => !p.locked);
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
      return Math.max(nearestMultiple(Math.round((100*p.mix_share)**(3/4)), 5), 5);
    },
    promoteProcessCost(p) {
      return Math.max(nearestMultiple(Math.round((100*(1-p.mix_share))**(3/4)), 5), 5);
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
