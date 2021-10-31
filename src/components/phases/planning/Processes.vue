<template>
  <div class="planning--page">
    <header>
      <img class="back" @click="$emit('close')" src="/assets/icons/back.svg">
    </header>
    <Cards>
      <template v-for="p in processes">
        <Card
          :title="p.name"
          :flag="p.status == 'Banned' || p.status == 'Promoted' ? p.status : null"
          :image="imageForProcess(p)">
          <template v-slot:header>
            <div>{{p.output}}</div>
            <div>{{(p.mix_share*100).toFixed(1)}}%</div>
          </template>
          <template v-slot:front>
            <div class="card--actions">
              <button v-if="p.status == 'Neutral'" @click="banProcess(p)">
                Ban
                <div class="card--action--cost">
                  {{banProcessCost(p)}}<img class="pip" src="/assets/icons/pips/political_capital.png">
                </div>
              </button>
              <button v-if="p.status == 'Banned'" @click="unbanProcess(p)">
                Unban
                <div class="card--action--cost">
                  {{promoteProcessCost(p)}}<img class="pip" src="/assets/icons/pips/political_capital.png">
                </div>
              </button>
              <button v-if="p.status == 'Neutral'" @click="promoteProcess(p)">
                Promote
                <div class="card--action--cost">
                  {{promoteProcessCost(p)}}<img class="pip" src="/assets/icons/pips/political_capital.png">
                </div>
              </button>
              <button v-if="p.status == 'Promoted'" @click="unpromoteProcess(p)">
                Unpromote
                <div class="card--action--cost">
                  {{banProcessCost(p)}}<img class="pip" src="/assets/icons/pips/political_capital.png">
                </div>
              </button>
            </div>
          </template>
          <template v-slot:back>
            <div class="card--back--body">
              <div class="card--body">
                {{state.processes[p.id].description}}
              </div>
            </div>
            <div class="image-attribution">
              Source image: {{state.processes[p.id].image.attribution}}
            </div>
          </template>
        </Card>
      </template>
    </Cards>
  </div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import Card from './Card.vue';
import Cards from './Cards.vue';
import {nearestMultiple} from '/src/lib/util';

export default {
  components: {
    Card,
    Cards,
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
  },
  methods: {
    imageForProcess(p) {
      let image = state.processes[p.id].image;
      if (image.fname) {
        return `/assets/content/images/${image.fname}`;
      } else {
        return '/assets/placeholders/project.png';
      }
    },
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

<style scoped>
.card {
  border: 6px solid #3AB56B;
}
</style>
