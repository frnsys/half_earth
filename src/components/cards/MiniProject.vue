<template>
<MiniCard>
  <template v-slot:body>
    <div class="minicard-background" :style="{backgroundImage: `url(/assets/content/images/${image.fname})`}" />
    <div :style="{zIndex: 1}">
      <img v-if="project.kind == 'Research'" src="/assets/icons/pips/research.png" />
      <img v-if="project.kind == 'Initiative'" src="/assets/icons/pips/initiative.png" />
      <img v-if="project.kind == 'Policy'" src="/assets/icons/pips/political_capital.png" />
      <div v-if="project.status == 'Building'" class="project-progress">
        <div class="project-progress-fill" :style="{width: `${project.progress*100}%`}" />
      </div>
    </div>
  </template>
  <template v-slot:expanded>
    <header>{{project.status}}</header>
    <ProjectCard :project="project" />
    <footer>
      <div class="pips">
        {{availablePoints}}<img class="pip" src="/assets/icons/pips/research.png">
      </div>
      <div class="pips pips--buy" @click="buyPoint">
        {{nextPointCost}}<img class="pip" src="/assets/icons/pips/political_capital.png"> ⮕ <img class="pip" src="/assets/icons/pips/research.png">
      </div>
    </footer>
  </template>
</MiniCard>
</template>

<script>
import state from '/src/state';
import MiniCard from './MiniCard.vue';
import ProjectCard from './ProjectCard.vue';
import ProjectMixin from 'components/phases/planning/ProjectMixin';

export default {
  props: ['project'],
  mixins: [ProjectMixin],
  data() {
    return {
      type: this.project.kind,
      ...state.projects[this.project.id],
    }
  },
  components: {
    MiniCard,
    ProjectCard,
  }
}
</script>

<style>
.project-progress {
  height: 5px;
  width: 80%;
  background: #aaa;
  margin: 0 auto;
  border-radius: 1em;
}
.project-progress-fill {
  background: #41C56D;
  height: 5px;
  border-radius: 1em;
}
</style>
