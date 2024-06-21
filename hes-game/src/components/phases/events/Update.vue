<template>
<div class="event project-completed" :style="{backgroundImage: imageUrl}" @click="tryDone" :class="{regionup : isRegion}">
  <div class="event--body">
    <template v-if="update.type == 'Project' || update.type == 'Policy'">
      <div class="arc">{{ update.type == 'Project' ? t('Project Completed') : t('Policy Outcome') }}</div>
      <div class="image-attribution">{{t('Image:')}} {{obj.image ? obj.image.attribution : ''}}</div>
      <div class="event--name">{{t(obj.name)}}</div>
      <div class="event--effects">
        <Effects :effects="obj.activeEffects" />
      </div>
    </template>
    <template v-else-if="update.type == 'Region:Up'" >
      <div class="arc">{{t('Region Developed')}}</div>
      <div class="image-attribution">{{t('Image:')}} {{obj.image ? obj.image.attribution : ''}}</div>
      <div class="event--name">{{t(obj.name)}}</div>
      <div class="event--outcome" v-html="t(`This region's income level has increased to <strong>{income}</strong>. Demand for <img src='{iconElec}'>electricity, <img src='{iconFuel}'>fuel, <img src='{iconPCals}'>plant and <img src='{iconACals}'>animal-based food has been updated.`, {income: display.enumDisplay(obj.income, true), iconFuel: icons.fuel, iconElec: icons.electricity, iconPCals: icons.plant_calories, iconACals: icons.animal_calories})"></div>
      <div class="event--icon-changes">
        <div class="event--icon-change">
          <IntensityIcon
            v-tip="{icon: 'wealth', text: t(`This region's previous income level.`)}"
            resource="wealth" :intensity="obj.income_level" :invert="true" />
          <img :src="icons.arrow_right_light">
          <IntensityIcon
            v-tip="{icon: 'wealth', text: t(`This region's new income level.`)}"
            resource="wealth" :intensity="obj.income_level + 1" :invert="true" />
        </div>
      </div>
      <div class="event--icon-changes event--icon-changes-group">
        <div class="event--icon-change" v-for="v, k in obj.changes">
          <IntensityIcon
            v-tip="{icon: k, text: t(`This region's previous demand for {output}.`, {output: display.enumDisplay(k)})}"
            :resource="k" :intensity="v[0]" />
          <img :src="icons.arrow_right_light">
          <IntensityIcon
            v-tip="{icon: k, text: t(`This region's new demand for {output}.`, {output: display.enumDisplay(k)})}"
            :resource="k" :intensity="v[1]" />
        </div>
      </div>
    </template>
    <template v-else-if="update.type == 'Region:Down'">
      <div class="arc">{{t('Region Contracted')}}</div>
      <div class="event--name">{{t(obj.name)}}</div>
      <div class="event--outcome" v-html="t(`This region's income level has contracted to <strong>{income}</strong>. Demand for <img src='{iconElec}'>electricity, <img src='{iconFuel}'>fuel, <img src='{iconPCals}'>plant and <img src='{iconACals}'>animal-based food has been updated.`, {income: display.enumDisplay(obj.income, true), iconFuel: icons.fuel, iconElec: icons.electricity, iconPCals: icons.plant_calories, iconACals: icons.animal_calories})"></div>
      <div class="event--icon-changes">
        <div class="event--icon-change">
          <IntensityIcon
            v-tip="{icon: 'wealth', text: t(`This region's previous income level.`)}"
            resource="wealth" :intensity="obj.income_level + 2" :invert="true" />
          <img :src="icons.arrow_right_light">
          <IntensityIcon
            v-tip="{icon: 'wealth', text: t(`This region's new income level.`)}"
            resource="wealth" :intensity="obj.income_level + 1" :invert="true" />
        </div>
      </div>
      <div class="event--icon-changes event--icon-changes-group">
        <div class="event--icon-change" v-for="v, k in obj.changes">
          <IntensityIcon
            v-tip="{icon: k, text: t(`This region's previous demand for {output}.`, {output: display.enumDisplay(k)})}"
            :resource="k" :intensity="v[0]" />
          <img :src="icons.arrow_right_light">
          <IntensityIcon
            v-tip="{icon: k, text: t(`This region's new demand for {output}.`, {output: display.enumDisplay(k)})}"
            :resource="k" :intensity="v[1]" />
        </div>
      </div>
    </template>
  </div>
  <Dialogue v-if="obj.activeOutcome" :dialogue="obj.activeOutcome.dialogue" :effects="[]" @started="dialogueStart" @done="dialogueDone" />
</div>
</template>

<script>
import state from '/src/state';
import consts from '/src/consts.json';
import Effects from 'components/Effects.vue';
import Dialogue from 'components/Dialogue.vue';
import REGIONS from 'content/regions.json';
import PROJECTS from 'content/projects.json';
import {activeEffects} from '/src/display/project';
import IntensityIcon from 'components/cards/IntensityIcon.vue';
import intensity from '/src/display/intensity';

export default {
  props: ['update'],
  components: {
    Effects,
    Dialogue,
    IntensityIcon
  },
  data() {
    return {
      canClose: (this.update.type == 'Project' || this.update.type == 'Policy') ? false : true,
    }
  },
  methods: {
    dialogueStart()  {
      this.canClose = false;
    },
    dialogueDone()  {
      this.canClose = true;
    },
    tryDone() {
      if (this.canClose) {
        this.$emit('done');
      }
    }
  },
  computed:{
    obj() {
      let {id, type} = this.update;
      if (type == 'Project' || type == 'Policy') {
        let obj = state.gameState.projects[id];
        let details = PROJECTS[id];
        obj.activeOutcome = details.outcomes[obj.active_outcome];
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
    isRegion(){
      if(this.update.type == 'Region:Up' || this.update.type == 'Region:Down'){
        return true
      } else {
        return false
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
  /* font-family: 'Times Ten', serif; */
  text-align: center;
  opacity: 0.8;
  font-size: 0.8em;
  padding: 1rem;
  margin: 1rem 2rem;
  border-radius:0.5rem;
  box-shadow: inset 1px 1px 0px rgb(0 0 0 / 50%);
  border-right: 1px solid rgba(255,255,255,0.5);
  border-bottom: 1px solid rgba(255,255,255,0.5);
  line-height: 1.4;
}

.event--outcome img{
  width:14px;
  margin-bottom: -2px;
  margin-right: 2px;
  image-rendering: auto;
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

.event.regionup .event--body{
  height: auto;
  padding-bottom: 3rem;
}

.event.regionup .label{
  color: rgba(255,255,255,0.8);
  text-transform: uppercase;
  font-size: 0.6em;
  letter-spacing: 0.01em;
  font-weight: bold;
  font-family: 'Inter', sans-serif;
}
</style>
