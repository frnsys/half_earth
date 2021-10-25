<template>
<div class="event" :style="{backgroundImage: `url(/assets/content/images/${project.image ? project.image.fname : 'missing_image.png'})`}">
  <div class="event--body">
    <div class="arc">Project Completed</div>
    <div class="image-attribution">Image source: {{project.image ? project.image.attribution : ''}}</div>
    <div class="event--name">{{project.name}}</div>
    <ul class="effects">
      <template v-for="desc in effectDescs">
        <li v-html="desc"></li>
      </template>
    </ul>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import {describeEffect} from '/src/effects';

export default {
  props: ['id'],
  computed: {
    project() {
      return state.projects[this.id];
    },
    effectDescs() {
      return this.project.effects
        .map((ev) => describeEffect(ev))
        .filter((desc) => desc !== undefined);
    }
  },
}
</script>
