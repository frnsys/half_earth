<template>
<MiniCard :style="style">
  <template v-slot:body>
    <div class="minicard-background" :style="{backgroundImage: `url(/assets/content/images/${image.fname})`}" />
    <div :style="{zIndex: 1}">
      <img :src="icons[icon]" />
      <div v-if="project.status == 'Building'" class="project-progress">
        <div class="project-progress-fill" :style="{width: `${project.progress*100}%`}" />
      </div>
      <div v-if="project.status == 'Finished' || project.status == 'Active'" class="project-check"><img :src="icons.check" /></div>
    </div>
  </template>
  <template v-slot:expanded>
    <header>{{project.status}}</header>
    <ProjectCard :project="project" />
    <footer>
      <div class="pips">
        {{availablePoints}}<img class="pip" :src="icons[icon]">
      </div>
      <div class="pips pips--buy" @click="buyPoint" v-if="type !== 'Policy'">
        {{nextPointCost}}<img class="pip" :src="icons.political_capital"> ⮕ <img class="pip" :src="icons[icon]">
      </div>
    </footer>
  </template>
</MiniCard>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
import MiniCard from './MiniCard.vue';
import ProjectCard from '../ProjectCard.vue';
import PROJECTS from '/assets/content/projects.json';

export default {
  props: ['project'],
  data() {
    return {
      type: this.project.kind,
      ...PROJECTS[this.project.id],
    }
  },
  components: {
    MiniCard,
    ProjectCard,
  },
  computed: {
    style() {
      let style = consts.groupStyle[this.project.group];
      if (!style) {
        return {border: `4px solid #fff`};
      } else {
        return {border: `4px solid ${style.background}`};
      }
    },
    icon() {
      return this.project.kind.toLowerCase();
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

<style>
.project-progress {
  height: 9px;
  width: 80%;
  background: #fff;
  border: 1px solid #fff;
  margin: 0 auto;
  border-radius: 1em;
}
.project-progress-fill {
  background: #2FE863;
  height: 7px;
  border-radius: 0.4em;
}
.project-check {
  border-radius: 1em;
  background: rgba(0,0,0,0.4);
  position: absolute;
  right: 0.2em;
  bottom: 0.2em;
  padding: 0.2em 0.5em 0 0.5em;
}
.project-check img {
  width: 16px !important;
}
</style>
