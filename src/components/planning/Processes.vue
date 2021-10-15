<template>
  <div class="planning--page">
    <header>
      <img class="back" @click="$emit('close')" src="/assets/icons/back.svg">
      <div class="pips">
        <div class="pips--label">Political Capital</div>
        <template v-for="i in state.points['Policy'].available">
          <img class="pip" src="/assets/placeholders/pip3.png">
        </template>
      </div>
    </header>
    <Cards>
      <template v-for="p in processes">
        <Card
          :title="p.name"
          :image="imageForProcess(p)">
          <template v-slot:header>
            <div>{{p.output}}</div>
            <div v-if="p.banned">Banned</div>
            <div>{{(p.mix_share*100).toFixed(1)}}%</div>
          </template>
          <template v-slot:back>
            {{state.processes[p.id].description}}
          </template>
          <template v-slot:extras>
            <button v-if="!p.banned" @click="banProcess(p)">
              Ban
              <div class="card--action--cost">
                <img class="pip" src="/assets/placeholders/pip3.png">
                <img class="pip" src="/assets/placeholders/pip3.png">
              </div>
            </button>
            <button v-if="p.banned" @click="unbanProcess(p)">
              Unban
              <div class="card--action--cost">
                <img class="pip" src="/assets/placeholders/pip3.png">
                <img class="pip" src="/assets/placeholders/pip3.png">
              </div>
            </button>
          </template>
        </Card>
      </template>
    </Cards>
  </div>
</template>

<script>
import game from '../../game';
import state from '../../state';
import Cards from './Cards.vue';
import Card from './Card.vue';

// Currently hardcoded,
// but should probably be determined in some way?
const PC_COST = 2;

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
    banProcess(p) {
      if (!p.banned && state.points['Policy'].available >= PC_COST) {
        state.points['Policy'].available -= PC_COST;
        game.banProcess(p.id);
      }
    },
    unbanProcess(p) {
      if (p.banned && state.points['Policy'].available >= PC_COST) {
        state.points['Policy'].available -= PC_COST;
        game.unbanProcess(p.id);
      }
    }
  }
}
</script>

<style scoped>
.card {
  background: #3AB56B;
}
</style>
