<template>
<div class="plan-change-select planning--page">
  <div class="planning--page-tabs">
   <div class="project-tab" @click="type = 'Research'" :class="{selected: type == 'Research'}">
      <img :src="icons.research" />
      <div>Research</div>
    </div>
   <div class="project-tab" @click="type = 'Initiative'" :class="{selected: type == 'Initiative'}">
      <img :src="icons.initiative" />
      <div>Infrastructure</div>
    </div>
   <div class="project-tab" @click="type = 'Policy'" :class="{selected: type == 'Policy'}">
      <img :src="icons.policy" />
      <div>Policies</div>
    </div>
    <div @click="$emit('close')">Back</div>
  </div>
  <div class="card-drag-target"></div>

  <Cards>
    <ProjectCard v-for="i in projectOrder"
      :key="projects[i].id"
      :project="projects[i]"
      @change="$emit('change')" />
  </Cards>
  <footer>
    <template v-if="type == 'Policy'">
      <div class="pips">
        {{availablePoints}}<img class="pip" :src="icons.political_capital">
      </div>
    </template>

    <template v-if="type == 'Research'">
      <div class="pips">
        {{availablePoints}}<img class="pip" :src="icons.research">
      </div>
      <div class="pips pips--buy" @click="buyPoint">
        {{nextPointCost}}<img class="pip" :src="icons.political_capital"> ⮕ <img class="pip" :src="icons.research">
      </div>
    </template>

    <template v-if="type == 'Initiative'">
      <div class="pips">
        {{availablePoints}}<img class="pip" :src="icons.initiative">
      </div>
      <div class="pips pips--buy" @click="buyPoint">
        {{nextPointCost}}<img class="pip" :src="icons.political_capital"> ⮕ <img class="pip" :src="icons.initiative">
      </div>
    </template>

  </footer>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
import Cards from 'components/cards/Cards.vue';
import ProjectCard from 'components/cards/ProjectCard.vue';

export default {
  components: {
    Cards,
    ProjectCard,
  },
  data() {
    return {
      state,
      type: 'Research',
      projectOrder: [],
    };
  },
  computed: {
    projectOrder() {
      let projects = state.gameState.projects
        .filter((p) => p.kind == this.type && !p.locked);

      let idxs = projects.map((p, i) => i);
      idxs.sort((a, b) => projects[a].name.toLowerCase().localeCompare(projects[b].name.toLowerCase()))
      return idxs;
    },
    projects() {
      return state.gameState.projects.filter((p) => p.kind == this.type && !p.locked);
    },
    availablePoints() {
      if (this.type == 'Policy') {
        return state.gameState.political_capital;
      } else {
        return state.points[this.type.toLowerCase()];
      }
    },
    nextPointCost() {
      return consts.pointCost;
    },
  },
  methods: {
    buyPoint() {
      let cost = this.nextPointCost;
      if (cost <= state.gameState.political_capital) {
        game.changePoliticalCapital(-cost);
        state.points[this.type.toLowerCase()]++;
      }
    }
  }
}
</script>
