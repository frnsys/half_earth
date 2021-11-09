<template>
<div class="event" :style="{backgroundImage: `url(/assets/content/images/${project.image ? project.image.fname : 'missing_image.png'})`}">
  <div class="event--body">
    <div class="arc">Project Completed</div>
    <div class="image-attribution">Image source: {{project.image ? project.image.attribution : ''}}</div>
    <div class="event--name">{{project.name}}</div>
    <Effects :effects="project.effects" />
  </div>
</div>
</template>

<script>
import state from '/src/state';
import display from 'lib/display';
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
