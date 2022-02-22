<template>
<div class="event project-completed" :style="{backgroundImage: imageUrl}">
  <div class="event--body">
    <template v-if="update.type == 'Project'">
      <div class="arc">Project Completed</div>
      <div class="image-attribution">Image source: {{image ? image.attribution : ''}}</div>
      <div class="event--name">{{obj.name}}</div>
      <div class="event--outcome">{{obj.activeOutcome.text}}</div>
      <div class="event--effects">
        <Effects :effects="effects" />
      </div>
    </template>
    <template v-else-if="update.type == 'Region:Up'">
      <div class="arc">Region Developed</div>
      <div class="image-attribution">Image source: {{image ? image.attribution : ''}}</div>
      <div class="event--name">{{obj.name}}</div>
      <div class="event--outcome">This region's development level has increased to {{display.enumDisplay(obj.income, true)}}.</div>
    </template>
    <template v-else-if="update.type == 'Region:Down'">
      <div class="arc">Region Contracted</div>
      <div class="event--name">{{obj.name}}</div>
      <div class="event--outcome">This region's development level has contracted to {{display.enumDisplay(obj.income, true)}}.</div>
    </template>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import Effects from 'components/Effects.vue';
import REGIONS from '/assets/content/regions.json';
import PROJECTS from '/assets/content/projects.json';
import {activeEffects} from '/src/display/project';

export default {
  props: ['update'],
  components: {
    Effects
  },
  computed:{
    obj() {
      let {id, type} = this.update;
      if (type == 'Project') {
        let obj = state.gameState.projects[id];
        let details = PROJECTS[id];
        obj.activeOutcome = details.outcomes[project.active_outcome];
        obj.activeEffects = activeEffects(obj),
        obj.image = details.image;
        return obj;
      } else if (type.startsWith('Region')) {
        let obj = state.gameState.world.regions[id];
        let details = REGIONS[id];
        obj.image = details.image;
        return obj;
      }
    },
    imageUrl() {
      if (this.obj.image){
        if(this.obj.image.fname){
          return 'url(/assets/content/images/' + this.obj.image.fname + ')'
        } else {
          return 'url(/assets/missing_content.png)'
        }
      } else {
        return 'url(/assets/missing_content.png)'
      }
    },
  }
}
</script>

<style>

.project-completed .event--name{
  font-family: 'Times Ten', serif;
  text-align: center;
  font-size: 1.5em;
  padding: 0.5em 0.5em;
  line-height: 110%;
}

.project-completed {
  background-repeat: no-repeat;
  background-size: cover;
  background-position: center;
}
.event--outcome {
  font-size: 0.85em;
  margin: 1em 2em;
}
</style>
