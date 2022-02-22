<template>
<div class="event project-completed" :style="{backgroundImage: imageUrl}">
  <div class="event--body">
    <template v-if="update.type == 'Project'">
      <div class="arc">Project Completed</div>
      <div class="image-attribution">Image source: {{obj.image ? obj.image.attribution : ''}}</div>
      <div class="event--name">{{obj.name}}</div>
      <div class="event--outcome">{{obj.activeOutcome.text}}</div>
      <div class="event--effects">
        <Effects :effects="obj.activeEffects" />
      </div>
    </template>
    <template v-else-if="update.type == 'Region:Up'">
      <div class="arc">Region Developed</div>
      <div class="image-attribution">Image source: {{obj.image ? obj.image.attribution : ''}}</div>
      <div class="event--name">{{obj.name}}</div>
      <div class="event--outcome">This region's income level has increased to {{display.enumDisplay(obj.income, true)}}.</div>
      <div class="event--icon-changes">
        <div class="event--icon-change">
          <IntensityIcon
            v-tip="{icon: 'wealth', text: `This region previous income level.`}"
            resource="wealth" :intensity="obj.income_level" :invert="true" />
          ⟶
          <IntensityIcon
            v-tip="{icon: 'wealth', text: `This region new income level.`}"
            resource="wealth" :intensity="obj.income_level + 1" :invert="true" />
        </div>
      </div>
      <div class="event--icon-changes event--icon-changes-group">
        <div class="event--icon-change" v-for="v, k in obj.changes">
          <IntensityIcon
            v-tip="{icon: k, text: `This region previous demand for ${display.enumDisplay(k)}.`}"
            :resource="k" :intensity="v[0]" />
          ⟶
          <IntensityIcon
            v-tip="{icon: k, text: `This region previous demand for ${display.enumDisplay(k)}.`}"
            :resource="k" :intensity="v[1]" />
        </div>
      </div>
    </template>
    <template v-else-if="update.type == 'Region:Down'">
      <div class="arc">Region Contracted</div>
      <div class="event--name">{{obj.name}}</div>
      <div class="event--outcome">This region's income level has contracted to {{display.enumDisplay(obj.income, true)}}.</div>
      <div class="event--icon-changes">
        <div class="event--icon-change">
          <IntensityIcon
            v-tip="{icon: 'wealth', text: `This region previous income level.`}"
            resource="wealth" :intensity="obj.income_level + 2" :invert="true" />
          ⟶
          <IntensityIcon
            v-tip="{icon: 'wealth', text: `This region new income level.`}"
            resource="wealth" :intensity="obj.income_level + 1" :invert="true" />
        </div>
      </div>
      <div class="event--icon-changes event--icon-changes-group">
        <div class="event--icon-change" v-for="v, k in obj.changes">
          <IntensityIcon
            v-tip="{icon: k, text: `This region previous demand for ${display.enumDisplay(k)}.`}"
            :resource="k" :intensity="v[0]" />
          ⟶
          <IntensityIcon
            v-tip="{icon: k, text: `This region previous demand for ${display.enumDisplay(k)}.`}"
            :resource="k" :intensity="v[1]" />
        </div>
      </div>
    </template>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import consts from '/src/consts.json';
import Effects from 'components/Effects.vue';
import REGIONS from '/assets/content/regions.json';
import PROJECTS from '/assets/content/projects.json';
import {activeEffects} from '/src/display/project';
import IntensityIcon from 'components/cards/IntensityIcon.vue';
import intensity from '/src/display/intensity';

export default {
  props: ['update'],
  components: {
    Effects,
    IntensityIcon
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
        let level = obj.income_level;
        let prev = type === 'Region:Up' ? level-1 : level+1;
        let cur = consts.demand_levels[level];
        prev = consts.demand_levels[prev];

        obj.changes = {};
        Object.keys(cur).forEach((k) => {
          obj.changes[k] = [
            intensity.intensity(prev[k], k),
            intensity.intensity(cur[k], k)
          ];
        });
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

.event--icon-changes {
  text-align: center;
}
.event--icon-change {
  margin: 0.5em 0;
}
.event--icon-change > * {
  display: inline-block;
  vertical-align: middle;
}
.event--icon-change .card-icon {
  margin: 0 10px;
}
.event--icon-change .card-icon img {
  width: 24px;
}
.event--icon-changes-group {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-evenly;
  max-width: 320px;
  margin: 0 auto;
}
.event--icon-changes-group .event--icon-change {
  width: 48%;
  background: #282828;
  padding: 0.25em 0;
  border-radius: 0.3em;
  border: 1px solid #333;
}
.event--icon-changes-group .event--icon-change img {
  width: 20px;
}
</style>
