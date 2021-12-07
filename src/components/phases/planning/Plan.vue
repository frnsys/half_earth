<template>
<div class="plan">
  <PlanChangeSelect v-if="page == 'Add'" @close="page = null" @page="(p) => $emit('page', p)" />
  <ProcessesSelect v-if="page == 'Processes'" @close="page = null" />
  <div class="plan--changes" v-if="page == null">
    <div class="plan--change">
      <div class="plan--action">Add</div>
      <div class="plan--add-change minicard" @click="selectPage('Add')">
        <img :src="icons.add">
      </div>
    </div>
    <div class="plan--change">
      <div class="plan--action">Processes</div>
      <div class="minicard processes-minicard" @click="selectPage('Processes')">
        <div>
          <img :src="icons.electricity" />
          <img :src="icons.fuel" />
          <img :src="icons.plant_calories" />
          <img :src="icons.animal_calories" />
        </div>
      </div>
    </div>
    <div class="plan--change" v-for="project in activeProjects">
      <div class="plan--action">
        <img v-if="project.status == 'Finished' || project.status == 'Active'" :src="icons.check">
        <template v-else>{{projectStatus(project)}}</template>
      </div>
      <MiniProject :project="project" />
      <div class="plan--note">{{project.name}}</div>
    </div>
    <div class="plan--change" v-for="process in activeProcesses">
      <div class="plan--action">{{process.status}}</div>
      <MiniProcess :process="process" />
      <div class="plan--note">{{process.name}}</div>
    </div>
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
import Chart from './Chart.vue';
import ProcessesSelect from './ProcessesSelect.vue';
import PlanChangeSelect from './PlanChangeSelect.vue';
import MiniProcess from 'components/cards/MiniProcess.vue';
import MiniProject from 'components/cards/MiniProject.vue';
import historicalLandUse from '/assets/historical/land_use.json';
import historicalEmissions from '/assets/historical/emissions.json';
import display from 'lib/display';

const charts = {
  'land': 'Land Use',
  'emissions': 'Emissions',
};

const startYear = 1990;
const endYear = 2122;
const years = 2122-1990;

const formats = {
  'land': (v) => display.landUsePercent(v),
  'emissions': (v) => v * 1e-15,
}

export default {
  components: {
    Chart,
    MiniProcess,
    MiniProject,
    ProcessesSelect,
    PlanChangeSelect,
  },
  created() {
    this.charts = charts;
  },
  data() {
    let events = game.roll.planning('Plan');
    return {
      state,
      events,
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
    activeProcesses() {
      return state.gameState.processes.filter((p) => p.status !== 'Neutral');
    },
    simulated() {
      let n = years - (this.historical.data.length - 1);
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
        color: '#5268A3',
      }, {
        text: 'Now',
        point: {x: this.historical.data.length - 1, y: 0.8},
        anchor: 'CENTER',
        background: '#FFECC7'
      }, {
        text: 'With these changes',
        point: {x: 50, y: 0.5},
        background: '#FFECC7'
      }];
    },
    historical() {
      let data = [];
      switch (this.chart) {
        case 'land':
          data = historicalLandUse.concat(state.history.land_use)
            .map((v) => display.landUsePercent(v));
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
        color: '#BC6A58',
      }
    },
    projection() {
      let data = this.simulated.map((d, i) => ({
        x: i + this.historical.data.length - 1,
        y: formats[this.chart](d[this.chart])/100
      }));
      return {
        data,
        color: '#CDB6AD'
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
}
.plan--action {
  text-transform: uppercase;
  font-size: 0.7em;
  /* center overflowed text */
  margin-left: -100%;
  margin-right: -100%;
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
  width: 16px;
  vertical-align: middle;
}
.plan--charts--tabs > div {
  border-radius: 0.2em;
  border: 1px solid #000;
  padding: 0 0.2em;
  margin: 0 0.1em;
  display: inline-block;
}
.plan--charts--tabs > div.active {
  background: #222;
  color: #fff;
}

.plan--change .plan--add-change {
  border: 1px solid #b39d72;
  background: #e6d3af;
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
  padding: 0.1em 0.25em;
  position: absolute;
  left: 50%;
  bottom: 2em;
  transform: translate(-50%, 0);
}

.processes-minicard img {
  width: 28px;
}
</style>
