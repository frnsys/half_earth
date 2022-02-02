<template>
<MiniCard :style="style">
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
    <div class="mini-scanbar">
      <div class="scanbar-led scanbar-led-ok"></div>
      <div class="scanbar-led scanbar-led-bad"></div>
      <div class="card-scan-target" ref="target"></div>
    </div>

    <div class="card-withdraw-target" ref="withdrawTarget">
      Withdraw
      <div class="withdraw-bar" ref="withdrawProgress"></div>
    </div>

    <Draggable
      @drag="checkDrag"
      @dragStop="stopDrag"
      :minY="yMin"
      :maxY="yMax"
      :draggable="true">
      <ProjectCard
        :project="project"
        @change="$emit('change')" />
    </Draggable>

    <footer>
      <div class="pips">
        <div class="scan-progress" ref="scanProgress"></div>
        <template v-if="type == 'Policy'">
          {{availablePoints}}<img class="pip" :src="icons.political_capital">
        </template>
        <template v-else>
          <template v-if="availablePoints > 0">
            {{availablePoints}}<img class="pip" :src="icons[icon]">
          </template>
          <template v-else>
            {{nextPointCost}}<img class="pip" :src="icons.political_capital"> ⮕ <img class="pip" :src="icons[icon]">
          </template>
        </template>
      </div>
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
import PROJECTS from '/assets/content/projects.json';
import ScannerMixin from 'components/phases/ScannerMixin';

export default {
  mixins: [ScannerMixin],
  props: ['project'],
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
    style() {
      let style = consts.groupStyle[this.project.group];
      if (!style) {
        return {border: `4px solid #fff`};
      } else {
        return {border: `4px solid ${style.background}`};
      }
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

.mini-scanbar {
  height: 60px;
  width: 300px;
  border-radius: 0 0 0.5em 0.5em;
  position: absolute;
  top: 0;
  background: #eee;
  border-right: 2px solid rgba(0,0,0,0.6);
  border-bottom: 2px solid rgba(0,0,0,0.6);
  border-left: 2px solid #fff;
  box-shadow: 0 0 8px rgba(0,0,0,0.6);
}
.mini-scanbar .card-scan-target {
  left: 0;
  right: 0;
}

.minicard--expanded .draggable {
  z-index: -1;
  margin-top: 3em;
}

.scanbar-led {
  height: 10px;
  width: 10px;
  border-radius: 100em;
  position: absolute;
  bottom: 1em;
}
.scanbar-led-ok {
  right: 1em;
  background: #789782;
  border: 1px solid #3f704f;
}
.scan-ok .scanbar-led-ok {
  background: #5dee8d;
  border: 1px solid #c0fad3;
  box-shadow: 0 0 8px #31f471;
}
.scanbar-led-bad {
  right: 2.25em;
  background: #944d50;
  border: 1px solid #794343;
}
.scan-fail .scanbar-led-bad {
  background: #EB3941;
  border: 1px solid #f4c6c6;
  box-shadow: 0 0 8px #EB3941;
}
</style>
