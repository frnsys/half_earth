<template>
<MiniCard :style="style" :class="'project'">
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
    <AddScanner ref="addScanner" :project="project" />
    <RemoveScanner ref="removeScanner" :project="project" />

    <Draggable
      @drag="onDrag"
      @dragStop="onDragStop"
      :yBounds="yBounds"
      :draggable="true">
      <ProjectCard
        :project="project"
        @change="$emit('change')" />
    </Draggable>

    <footer>
      <Points :kind="project.kind" />
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
import Draggable from 'components/cards/Draggable.vue';
import PROJECTS from '/assets/content/projects.json';
import Points from 'components/scanner/project/Points.vue';
import AddScanner from 'components/scanner/project/AddScanner.vue';
import RemoveScanner from 'components/scanner/project/RemoveScanner.vue';

export default {
  props: ['project'],
  data() {
    return {
      type: this.project.kind,
      ...PROJECTS[this.project.id],
    }
  },
  components: {
    Points,
    MiniCard,
    ProjectCard,
    AddScanner,
    RemoveScanner,
    Draggable,
  },
  computed: {
    availablePoints() {
      if (this.project.kind == 'Policy') {
        return state.gameState.political_capital;
      } else {
        return state.points[this.project.kind.toLowerCase()];
      }
    },
    icon() {
      return this.project.kind.toLowerCase();
    },
    style() {
      let style = consts.groupStyle[this.project.group];
      if (!style) {
        return {border: `4px solid #fff`};
      } else {
        return {border: `4px solid ${style.background}`};
      }
    },
  },
  methods: {
    yBounds() {
      return [
        this.$refs.addScanner.botY - 10,
        this.$refs.removeScanner.topY + 10 - 430, // card height
      ];
    },
    onDrag(rect) {
      this.$refs.addScanner.checkDrag(rect);
      this.$refs.removeScanner.checkDrag(rect);
    },
    onDragStop() {
      this.$refs.addScanner.stopDrag();
      this.$refs.removeScanner.stopDrag();
    },
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

.minicard-project:hover{
  transform:scale(1.2);
}
</style>
