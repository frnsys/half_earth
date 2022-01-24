<template>
<div class="event project-completed" :style="{backgroundImage: `url(/assets/content/images/${project.image ? project.image.fname : 'missing_image.png'})`}">
  <div class="event--body">
    <div class="arc">Project Completed</div>
    <div class="image-attribution">Image source: {{project.image ? project.image.attribution : ''}}</div>
    <div class="event--name">{{project.name}}</div>
    <div class="event--outcome">{{outcome.text}}</div>
    <div class="event--effects">
      <Effects :effects="effects" />
    </div>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import Effects from 'components/Effects.vue';
import PROJECTS from '/assets/content/projects.json';
import {activeEffects} from '/src/display/project';

export default {
  props: ['id'],
  components: {
    Effects
  },
  data() {
    let project = state.gameState.projects[this.id];
    let details = PROJECTS[project.id];
    let activeOutcome = details.outcomes[project.active_outcome];
    return {
      project,
      outcome: activeOutcome,
      effects: activeEffects(project),
    }
  },
}
</script>

<style>
.project-completed {
  background-repeat: no-repeat;
  background-size: cover;
}
.event--outcome {
  font-size: 0.85em;
  margin: 1em 2em;
}
</style>
