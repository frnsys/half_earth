<template>
<MiniCard>
  <template v-slot:body>
    <div class="minicard-background" :style="{backgroundImage: `url(/assets/content/images/${image.fname})`}" />
    <div :style="{zIndex: 1}">
      <img :src="icons[icon]" />
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
import MiniCard from './MiniCard.vue';
import ProjectCard from './ProjectCard.vue';
import ProjectMixin from 'components/phases/planning/ProjectMixin';
import PROJECTS from '/assets/content/projects.json';

export default {
  props: ['project'],
  mixins: [ProjectMixin],
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
    icon() {
      return this.project.kind.toLowerCase();
    }
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
