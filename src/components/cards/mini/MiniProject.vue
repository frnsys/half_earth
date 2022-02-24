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
    <transition appear name="appear-popdown">
    <div class="mini-scanbar" ref="target">
      <div class="scanbar-base">
        <div class="scan-progress-bar" ref="scanProgress"></div>
      </div>
      <div class="scanbar-led scanbar-led-ok"></div>
      <div class="scanbar-led scanbar-led-bad"></div>
      <div class="card-scan-target" ></div>
    </div>
    </transition>

    <div class="card-withdraw-target" ref="withdrawTarget">
      {{ refundable ? 'Undo' : (canDowngrade ? 'Downgrade' : 'Withdraw') }}
      <div class="withdraw-bar" ref="withdrawProgress"></div>
    </div>

    <transition appear name="appear-bounceup">
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
    </transition>

    <footer>
      <div class="pips">
        <!-- <div class="scan-progress" ref="scanProgress"></div> -->
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


.minicard-project:hover{
  transform:scale(1.2);
}
</style>
