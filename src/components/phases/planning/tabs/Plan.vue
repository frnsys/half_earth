<template>
<div class="planning--page plan">
  <Projects v-if="page == 'Add'"
    @close="close"
    @page="(p) => $emit('page', p)"
    @change="$emit('change')" />
  <Processes v-if="page == 'Processes'"
    @close="close" @change="$emit('change')" />
  <ActivePlan v-if="page == 'All'"
    @close="close"
    @add="selectPage('Add')"
    @change="$emit('change')" />
  <div v-if="page == null">
    <div class="plan--changes" :style="{maxWidth}">
      <HelpTip text="Add some cards to get started" x="50%" y="220px" :center="true" />
      <img class="plan-new-icon plan-new-projects-icon" v-if="anyNewProjects" src="/assets/new.svg" />
      <div class="plan--change">
        <div class="plan--add-change minicard" @click="selectPage('Add')" :class="{highlight: projectsHighlighted}">
          <div>
            <img :src="icons.add">
            <div class="plan--action">Add</div>
          </div>
        </div>
      </div>
      <div class="plan--change" v-for="project in activeProjects.slice(0, this.nProjects)">
        <MiniProject :project="project" :key="project.id" />
      </div>
      <div class="plan--change" v-for="i in placeholders">
        <div class="plan--change-placeholder"></div>
      </div>
      <div class="plan--change" v-if="activeProjects.length > this.slots">
        <div class="plan--change-view-all" @click="selectPage('All')">View<br />All</div>
      </div>
    </div>
    <div class="plan--production">
      <div class="plan--production-icons">
        <img class="plan-new-icon" v-if="anyNewProcesses" src="/assets/new.svg" />
        <img class="plan-alert" v-if="processesOverLimit.length > 0" :src="icons.alert" v-tip="{icon: 'alert', text: `The following processes can't produce as much as they need to: ${processesOverLimit.join(', ')}`}"/>
        <img class="plan-alert" v-if="productionShortages" :src="icons.alert" v-tip="{icon: 'alert', text: `${productionShortages}. A resource such as land or water may be overexploited and thus limiting how much processes can produce.`}"/>
      </div>
      <div class="plan--production--processes">
        <MiniProcess v-for="process in maxProcesses" :process="process" :key="process.id" />
      </div>
      <div class="plan--production-button btn" :class="{disabled: processesDisabled, highlight: processesHighlighted}" @click="selectPage('Processes')">Change Production</div>
    </div>
    <div class="plan--charts">
      <HelpTip text="The predicted effect of your current plan is shown here" x="50%" y="220px" :center="true" />
      <div class="plan--charts--tabs">
        <div v-for="name, key in charts" :class="{active: key == chart}" @click="setChart(key)">
          <img :src="icons[key]">{{name}}
        </div>
      </div>
      <Chart :datasets="datasets" :markers="markers" :ranges="ranges"/>
    </div>
    <div class="plan--ready-container">
      <button class="plan--ready" :class="{disabled: readyDisabled, highlight: readyHighlighted}" @click="enterWorld">Ready</button>
    </div>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import Chart from '../Chart.vue';
import format from '/src/display/format';
import display from '/src/display/display';
import ActivePlan from '../ActivePlan.vue';
import Processes from '../Processes.vue';
import Projects from '../Projects.vue';
import HelpTip from 'components/Help.vue';
import MiniProcess from 'components/cards/mini/MiniProcess.vue';
import MiniProject from 'components/cards/mini/MiniProject.vue';
import historicalLandUse from '/assets/historical/land_use.json';
import historicalEmissions from '/assets/historical/emissions.json';
import tutorial from '/src/tutorial';

const charts = {
  'land': 'Land Use',
  'emissions': 'Emissions',
};

const formats = {
  'land': (v) => format.landUsePercent(v),
  'emissions': (v) => v * 1e-15,
}

export default {
  components: {
    Chart,
    MiniProcess,
    MiniProject,
    Projects,
    Processes,
    ActivePlan,
    HelpTip,
  },
  mounted() {
    this.onResize();

    window.audioManager.startSoundtrack('/assets/music/5yr_plan.mp3', true);
    window.addEventListener('resize', this.onResize);

    game.saveGame();
    game.saveMeta();
  },
  beforeUnmount() {
    window.audioManager.stopSoundtrack(true);
  },
  unmounted() {
    window.removeEventListener('resize', this.onResize);
  },
  created() {
    this.charts = charts;
  },
  data() {
    let events = game.roll.planning('Plan');
    let years = state.gameState.death_year - 1990;
    return {
      state,
      events,
      years,
      slots: 5,
      page: null,
      chart: 'land',
      ranges: {
        x: [0, years],
        y: [0, 1],
      }
    }
  },
  computed: {
    productionShortages() {
      let total = 0;
      let problems = {};
      Object.keys(state.gameState.output_demand).forEach((k) => {
        let met = state.gameState.produced[k]/state.gameState.output_demand[k];
        if (met >= 0.99) {
          return
        } else if (met >= 0.85) {
          problems[k] = 'mild';
        } else if (met >= 0.75) {
          problems[k] = 'alarming';
        } else if (met >= 0.5) {
          problems[k] = 'severe';
        } else {
          problems[k] = 'critical';
        }
      });
      let keys = Object.keys(problems);
      if (keys.length > 0) {
        if (keys.length > 1) {
          return `There are multiple production shortages: ${keys.map((k) => `<b class="shortage-${problems[k]}">${display.enumDisplay(k)} (${problems[k]})</b>`).join(', ')}`;
        } else {
          return `There is a <b class="shortage-${problems[keys[0]]}">${problems[keys[0]]} ${display.enumDisplay(keys[0])}</b> production shortage`;
        }
      } else {
        return;
      }
    },
    anyNewProjects() {
      let allProjects = state.gameState.projects.filter((p) => !p.locked).map((p) => p.ref_id);
      return allProjects.some((ref_id) => !state.viewed.includes(ref_id));
    },
    anyNewProcesses() {
      let allProcesses = state.gameState.processes.filter((p) => !p.locked).map((p) => p.ref_id);
      return allProcesses.some((ref_id) => !state.viewed.includes(ref_id));
    },
    maxWidth() {
      if (this.slots == 5) {
        return '320px';
      } else if (this.slots == 7) {
        return '440px';
      } else if (this.slots == 9) {
        return '530px';
      }
    },
    placeholders() {
      return Math.max(0, this.slots - this.activeProjects.length);
    },
    nProjects() {
      if (this.activeProjects.length > this.slots) {
        return this.slots - 1; // Save one spot for "View All"
      } else {
        return this.activeProjects.length;
      }
    },
    maxProcesses() {
      let maxProcesses = {
        Electricity: null,
        Fuel: null,
        AnimalCalories: null,
        PlantCalories: null,
      };
      state.gameState.processes.forEach((p) => {
        let curMax = maxProcesses[p.output];
        if (curMax == null || p.mix_share > curMax.mix_share) {
          maxProcesses[p.output] = p;
        }
      });
      return Object.values(maxProcesses);
    },
    processesOverLimit() {
      return state.gameState.processes.filter((p) => p.mix_share > 0).filter((p) => {
        let maxShare = game.processMaxShare(p);
        return p.mix_share > maxShare;
      }).map((p) => p.name);
    },
    activeProjects() {
      return state.gameState.projects.filter((p) => p.status == 'Active' || p.status == 'Finished' || p.status == 'Building');
    },
    simulated() {
      let n = this.years - (this.historical.data.length - 1);
      return game.simulate(n);
    },
    datasets() {
      return [
        this.historical,
        this.projection,
      ]
    },
    markers() {
      return [{
        x: this.historical.data.length - 1,
        color: '#000000',
      }, {
        text: 'Now',
        point: {x: this.historical.data.length - 1, y: 0.8},
        anchor: 'CENTER',
        background: '#FFECC7'
      }, {
        text: 'Under current plan',
        point: {x: 50, y: 0.5},
        background: '#FFECC7'
      }];
    },
    historical() {
      let data = [];
      switch (this.chart) {
        case 'land':
          data = historicalLandUse.concat(state.history.land_use)
            .map((v) => format.landUsePercent(v));
          break;
        case 'emissions':
          data = historicalEmissions.concat(state.history.emissions);
          break;
      }
      return {
        data: data.map((y, i) => ({
          x: i,
          y: y/100
        })),
        color: '#aaaaaa',
      }
    },
    projection() {
      let data = this.simulated.map((d, i) => ({
        x: i + this.historical.data.length - 1,
        y: formats[this.chart](d[this.chart])/100
      }));
      return {
        data,
        color: '#FE4400'
      }
    },
    processesDisabled() {
      return state.tutorial < tutorial.PROCESSES;
    },
    processesHighlighted() {
      return state.tutorial == tutorial.PROCESSES;
    },
    readyDisabled() {
      return state.tutorial < tutorial.READY;
    },
    readyHighlighted() {
      return state.tutorial == tutorial.READY;
    },
    projectsHighlighted() {
      return state.tutorial == tutorial.PROJECTS;
    }
  },
  methods: {
    enterWorld() {
      if (state.tutorial == tutorial.READY) {
        state.tutorial++;
      }
      game.saveGame();
      game.saveMeta();
      state.phase = 'EVENTS';
    },
    close() {
      if (this.page == 'Add' && state.tutorial == tutorial.PROJECTS_BACK) {
        state.tutorial++;
      } else if (this.page == 'Processes' && state.tutorial == tutorial.PROCESSES_BACK) {
        state.tutorial++;
      }
      this.page = null;
      this.$emit('page', 'Plan');
    },
    projectStatus(p) {
      if (p.kind == 'Research' && p.status == 'Building') {
        return 'Researching';
      } else {
        return p.status;
      }
    },
    setChart(key) {
      this.chart = key;
    },
    selectPage(p) {
      this.page = p;
      this.$emit('page', p);
    },
    onResize() {
      if (window.innerWidth > 680) {
        this.slots = 9;
      } else if (window.innerWidth > 560) {
        this.slots = 7;
      } else {
        this.slots = 5;
      }
    },
  }
}
</script>

<style>
.plan {
  background: url('/assets/backgrounds/plan.png');
  background-size: cover;
  background-repeat: no-repeat;
  background-position: center center;
  image-rendering: pixelated;
}
.plan--changes {
  display: flex;
  justify-content: space-between;
  height: 300px;
  flex-wrap: wrap;
  margin: 0 auto;
  position: relative;
}
.plan--change {
  width: 90px;
  text-align: center;
  margin: 0.5em 0;
}
.plan--change .minicard {
  background: #222;
  box-shadow: 1px 1px 2px rgba(0,0,0,0.25);
}
.plan--change .minicard img {
  width: 36px;
}
.plan--action {
  text-transform: uppercase;
  font-size: 0.7em;
  font-family: 'Inter', sans-serif;
  /* center overflowed text */
  margin-left: -100%;
  margin-right: -100%;
  color: #726060;
}
.plan--action img {
  height: 12px;
}

.plan--note {
  font-size: 0.8em;
  white-space: nowrap;
  text-overflow: ellipsis;
  width: 100%;
  overflow: hidden;
}
.plan--note img {
  width: 16px;
  vertical-align: middle;
}

.plan--charts {
  margin-top: 0.5em;
  position: relative;
}
.plan--charts--tabs {
  display: flex;
  justify-content: space-around;
  margin-bottom: 0.5em;
}
.plan--charts--tabs img {
  width: 18px;
  vertical-align: middle;
  margin-right: 3px;
  margin-top: -2px;
}
.plan--charts--tabs > div {
  background: #fff;
  padding: 0.3em 0.5em;
  margin: 0 0.1em;
  border-radius: 0.5em;
  display: inline-block;
  box-shadow: 1px 1px 0px rgb(0 0 0 / 50%);
}
.plan--charts--tabs > div.active {
  background: #CDABA1;
  box-shadow: inset 1px 1px 0px rgb(0 0 0 / 50%);
  border-right: 1px solid rgba(255,255,255,0.5);
  border-bottom: 1px solid rgba(255,255,255,0.5);
}
.plan--charts .chart {
  background: url('/assets/grid.png') #F0D4CC;
  background-size: 60px;
  border-radius: 0.4em;
  box-shadow: inset 1px 1px 0px rgb(0 0 0 / 50%);
  border-right: 1px solid rgba(255,255,255,0.5);
  border-bottom: 1px solid rgba(255,255,255,0.5);
  margin-bottom: 1em;
}

.plan--change .plan--add-change {
  background: #F0D4CC;
  box-shadow: inset 1px 1px 0px rgb(0 0 0 / 50%);
  border-right: 1px solid rgba(255,255,255,0.5);
  border-bottom: 1px solid rgba(255,255,255,0.5);
  border-top: none;
  border-left: none;
  /* border-style: outset; */
  /* border-color: #ffeedc; */
}
.plan--add-change:hover{
  background: #FCEAE5;
  box-shadow: 0 0 8px var(--colour-pink);
}
.plan--change .plan--add-change img {
  width: 24px;
  margin: 0 auto;
}
.plan--change-placeholder {
  border: 1px dashed rgba(0,0,0,0.5);
  height: 130px;
  border-radius: 0.7em;
}
.plan--change-view-all {
  height: 130px;
  border-radius: 6px;
  background: #fff;
  display: flex;
  align-items: center;
  justify-content: space-around;
  font-size: 1.3em;
  box-shadow: 1px 1px 2px rgba(0,0,0,0.25);
}

.plan > header {
  display: flex;
  border-bottom: 1px solid #000;
}
.plan > header div {
  flex: 1;
  text-align: center;
  padding: 0.25em;
  border-right: 1px solid #000;
}
.plan > header div:last-child {
  border-right: none;
}

.plan--ready-container {
  position: absolute;
  max-width: 680px;
  left: 50%;
  width: 100%;
  transform: translate(-50%, 0);
  text-align: right;
  bottom: 0.5em;
}
.plan--ready {
  font-family: 'W95FA';
  font-size: 1.3rem;
  /* padding: 2em 1em; */
  width: 7rem;
  height: 7rem;
  right: 0.5rem;
  bottom: 0.5rem;
  background: red;
  border-radius: 50%;
  border-right: 2px solid rgba(0,0,0,0.5);
  border-bottom: 2px solid rgba(0,0,0,0.5);
  border-top: 1px solid rgba(255,255,255,0.5);
  border-left: 2px solid rgba(255,255,255,0.5);
  box-shadow: 1px 2px 4px rgb(0 0 0 / 50%);
  color: #fff;
  z-index: 2;
}
.plan--ready.disabled {
  filter: grayscale(1);
  opacity: 0.5;
  pointer-events: none;
}
.plan--ready.highlight {
  animation-duration: 0.75s;
  animation-name: highlight;
  animation-iteration-count: infinite;
  animation-direction: alternate;
}
.plan--ready:hover {
  background: red;
  transform: scale(1.05);
}

.processes-minicard img {
  width: 28px;
}

.plan-change-select {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  z-index: 2;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 28px 0 0 0 !important;
  background: url('/assets/backgrounds/plan.png');
  background-size: cover;
  background-repeat: no-repeat;
  background-position: cetner center;
  overflow-y: hidden !important;
  image-rendering: pixelated;
}
.plan-change-select > header {
  color: #fff;
  text-align: right;
  padding: 0.5em;
  display: flex;
  justify-content: space-between;
}
.plan-change-select > header > div:first-child {
  text-decoration: underline;
}
.planning--page-tabs {
  display: flex;
  justify-content: space-between;
  border-radius: 0 0 0.3em 0.3em;
  background: #fff;
  box-shadow: 0 1px 2px rgba(0,0,0,0.5);
}
.planning--page-tabs img {
  width: 16px;
  image-rendering: auto;
  display: inline-block;
}
.planning--page-tabs > div {
  flex: 1;
  padding: 0.5em;
  text-align: center;
  line-height: 1;
  border-right: 1px solid;
  /* display: flex; */
  /* align-items: center; */
}
.planning--page-tabs > div:last-child {
  border-right: none;
  display: flex;
  align-items: center;
  text-align: center;
  justify-content: space-around;
}
.planning--page-tabs > div.selected {
  background: var(--colour-pink);
}
.planning--page-tabs > div:hover {
  background: #d4e4a7;
}
.planning--page-tabs .disabled {
  opacity: 0.5;
  pointer-events: none;
}
.planning--page-tabs .highlight {
  animation-duration: 0.75s;
  animation-name: highlight;
  animation-iteration-count: infinite;
  animation-direction: alternate;
}


.plan--production, .plan--charts {
  max-width: 360px;
  margin: 0 auto;
}

.plan--production {
  background: #F0D4CC;
  border-radius: 0.4em;
  box-shadow: inset 1px 1px 0px rgb(0 0 0 / 50%);
  border-right: 1px solid rgba(255,255,255,0.5);
  border-bottom: 1px solid rgba(255,255,255,0.5);
  padding: 1em;
  text-align: center;
  min-height: 200px;
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  margin-bottom: 1em;
  position: relative;
}
.plan--production-bg {
  position: absolute;
  top: 1em;
  bottom: 1em;
  right: 1em;
  left: 1em;
  border-radius: 0.3em;
  background: url(/assets/backgrounds/production.jpg);
  mix-blend-mode: multiply;
  background-size: cover;
  background-position: center;
  image-rendering: pixelated;
}

.plan--production-button {
  padding: 1em 0.9em;
  max-width: 180px;
  margin: 0 auto;
  cursor: pointer;
  z-index: 1;
}

.planning-sub-tab {
  font-family: 'Inter', sans-serif;
  font-size: 0.7em;
}
.plan-change-select .planning--page-tabs {
  position: relative;
  z-index: 10;
  width: calc(100% - 1em);
  max-width: 360px;
  margin: 0 auto;
}
.planning--page-tabs div:first-child {
  border-radius: 0 0 0 0.3em;
}
.planning--page-tabs div:last-child {
  border-radius: 0 0 0.3em 0;
}

.plan--add-change.highlight {
  animation-duration: 0.75s;
  animation-name: highlight;
  animation-iteration-count: infinite;
  animation-direction: alternate;
}

.plan--production-icons {
  position: absolute;
  z-index: 1;
  left: -1em;
  top: 0;
}
.plan--production-icons img {
  display: block;
  margin-bottom: 8px;
}
.plan-new-icon {
  width: 48px;
  transform: rotate(-16deg);
  animation-duration: 0.75s;
  animation-name: new-pulse;
  animation-iteration-count: infinite;
}
.plan-alert {
  width: 36px;
  margin: 0 7px;
  animation-duration: 0.75s;
  animation-name: pulse;
  animation-iteration-count: infinite;
}
.plan-new-projects-icon {
  position: absolute;
  z-index: 1;
  left: -1em;
  top: 0;
}

.plan--production--processes {
  display: flex;
  margin-bottom: 1em;
}
.plan--production--processes .minicard {
  height: 110px;
  margin: 0 0.25em;
  border: 1px dashed rgba(0,0,0,0.5);
}

.shortage-alarming {
  color: #efea38;
}
.shortage-severe {
  color: #ef9038;
}
.shortage-critical {
  color: #ef3838;
}

@keyframes new-pulse {
  from {
    transform: rotate(-16deg) scale(1.);
  }
  50% {
    transform: rotate(-16deg) scale(1.05);
  }
  to {
    transform: rotate(-16deg) scale(1);
  }
}
</style>
