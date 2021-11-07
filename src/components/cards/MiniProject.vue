<template>
<MiniCard>
  <template v-slot:body>
    <div class="minicard-background" :style="{backgroundImage: `url(/assets/content/images/${image.fname})`}" />
    <div :style="{zIndex: 1}">
      <div>🏭</div>
      <img src="/assets/icons/pips/power.png" />
    </div>
  </template>
  <template v-slot:expanded>
    <ProjectCard :project="project" />
  </template>
</MiniCard>
</template>

<script>
import state from '/src/state';
import MiniCard from './MiniCard.vue';
import ProjectCard from './ProjectCard.vue';

export default {
  props: ['project'],
  data() {
    return {
      ...state.projects[this.project.id],
    }
  },
  components: {
    MiniCard,
    ProjectCard,
  }
}
</script>
