<template>
<div class="planning--page plan">
  <Projects v-if="page == 'Add'"
    @close="page = null"
    @page="(p) => $emit('page', p)"
    @change="$emit('change')" />
  <Processes v-if="page == 'Processes'"
    @close="page = null" @change="$emit('change')" />
  <div class="plan--changes" v-if="page == null">
    <div class="plan--change">
      <div class="plan--add-change minicard" @click="selectPage('Add')">
        <div>
          <img :src="icons.add">
          <div class="plan--action">Add</div>
        </div>
      </div>
    </div>
    <div class="plan--change" v-for="project in activeProjects">
      <MiniProject :project="project" />
    </div>
  </div>
  <div class="plan--production">
    <div @click="selectPage('Processes')">Change Production</div>
  </div>
  <div class="plan--charts">
    <div class="plan--charts--tabs">
      <div v-for="name, key in charts" :class="{active: key == chart}" @click="setChart(key)">
        <img :src="icons[key]">{{name}}
      </div>
    </div>
    <Chart :datasets="datasets" :markers="markers" :ranges="ranges"/>
  </div>
  <button class="plan--ready" @click="enterWorld">Ready</button>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import Chart from '../Chart.vue';
import format from '/src/display/format';
import Processes from '../Processes.vue';
import Projects from '../Projects.vue';
import MiniProcess from 'components/cards/mini/MiniProcess.vue';
import MiniProject from 'components/cards/mini/MiniProject.vue';
import historicalLandUse from '/assets/historical/land_use.json';
import historicalEmissions from '/assets/historical/emissions.json';

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
  },
  created() {
    this.charts = charts;
  },
  data() {
    let events = game.roll.planning('Plan');
    let years = state.endYear - 1990;
    return {
      state,
      events,
      years,
      page: null,
      chart: 'land',
      ranges: {
        x: [0, years],
        y: [0, 1],
      }
    }
  },
  computed: {
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
    enterWorld() {
      state.phase = 'EVENTS';
    }
  },
  methods: {
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
    }
  }
}
</script>

<style>
.plan {
  background: url('/assets/backgrounds/plan.jpg');
  background-size: cover;
  background-repeat: no-repeat;
}
.plan--changes {
  display: flex;
  justify-content: space-between;
  align-items: center;
  align-content: start;
  height: 360px;
  flex-wrap: wrap;
  overflow-y: scroll;
  flex-direction: column;
}
.plan--change {
  width: 80px;
  text-align: center;
  margin: 0.5em 0.25em;
}
.plan--change .minicard {
  background: #222;
  border: 4px solid #fff;
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
}

.plan--change .plan--add-change {
  background: #F0D4CC;
  box-shadow: inset 1px 1px 0px rgb(0 0 0 / 50%);
  border-right: 1px solid rgba(255,255,255,0.5);
  border-bottom: 1px solid rgba(255,255,255,0.5);
  border-top: none;
  border-left: none;
}
.plan--change .plan--add-change img {
  width: 32px;
  margin: 0 auto;
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

.plan--ready {
  font-size: 1.3em;
  padding: 2em 1em;
  position: absolute;
  right: 1em;
  bottom: 0em;
  background: red;
  border-radius: 10em;
  border-right: 2px solid rgba(0,0,0,0.5);
  border-bottom: 2px solid rgba(0,0,0,0.5);
  border-top: 1px solid rgba(255,255,255,0.5);
  border-left: 2px solid rgba(255,255,255,0.5);
  box-shadow: 1px 2px 4px rgb(0 0 0 / 50%);
  color: #fff;
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
  background: #ff6b56;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding-top: 1em; /* Space for the hud */
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
  border-bottom: 1px solid;
}
.planning--page-tabs img {
  width: 32px;
}
.planning--page-tabs > div {
  flex: 1;
  padding: 0.5em;
  text-align: center;
  line-height: 1;
  border-right: 1px solid;
  font-size: 0.7em;
}
.planning--page-tabs > div:last-child {
  border-right: none;
}
.planning--page-tabs > div.selected {
  background: #eed793;
}
.planning--page-tabs .disabled {
  opacity: 0.5;
}

.plan--production {
  background: #F0D4CC;
  border-radius: 0.4em;
  box-shadow: inset 1px 1px 0px rgb(0 0 0 / 50%);
  border-right: 1px solid rgba(255,255,255,0.5);
  border-bottom: 1px solid rgba(255,255,255,0.5);
  padding: 1em;
  text-align: center;
  height: 220px;
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  margin-bottom: 1em;
}

.plan--production > div {
  background: #fff;
  padding: 1em 0.9em;
  border-radius: 0.5em;
  box-shadow: 1px 1px 0px rgb(0 0 0 / 50%);
  max-width: 180px;
  margin: 0 auto;
  cursor: pointer;
}
</style>
