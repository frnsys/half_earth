<template>
<div class="event project-completed" :style="{backgroundImage: `url(/assets/content/images/${project.image ? project.image.fname : 'missing_image.png'})`}">
  <div class="event--body">
    <div class="arc">Project Completed</div>
    <div class="image-attribution">Image source: {{project.image ? project.image.attribution : ''}}</div>
    <div class="event--name">{{project.name}}</div>
    <div class="event--effects">
      <Effects :effects="project.effects" />
    </div>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import Effects from 'components/Effects.vue';
import PROJECTS from '/assets/content/projects.json';

export default {
  props: ['id'],
  components: {
    Effects
  },
  computed: {
    project() {
      return PROJECTS[this.id];
    },
  },
}
</script>

<style>
.project-completed {
  background-repeat: no-repeat;
  background-size: cover;
}
</style>
