<template>
<div class="planning--page">
  <header>
    <img class="back" @click="$emit('close')" src="/assets/icons/back.svg">
  </header>
  <div class="plan--changes">
    <div class="plan--change" v-for="change in changes">
      <div class="plan--action">{{change.action}}</div>
      <MiniProcess :process="change.process" />
      <div class="plan--cost"><img :src="assets.icons.political_capital">28</div>
    </div>
  </div>
  <div class="plan--charts">
    <div class="plan--charts--tabs">
      <div v-for="name, key in charts" :class="{active: key == chart}" @click="setChart(key)">
        <img :src="assets.icons[key]">{{name}}
      </div>
    </div>
    <Chart :datasets="datasets" :markers="markers" :ranges="ranges"/>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import Chart from './Chart.vue';
import MiniProcess from 'components/cards/MiniProcess.vue';
import historicalLandUse from '/assets/historical/land_use.json';
import historicalEmissions from '/assets/historical/emissions.json';

const totalLand = 104e12;

const charts = {
  'land': 'Land Use',
  'emissions': 'Emissions',
};

const startYear = 1990;
const endYear = 2122;
const years = 2122-1990;

export default {
  components: {
    Chart,
    MiniProcess,
  },
  created() {
    this.charts = charts;
  },
  data() {
    return {
      state,
      changes: [{
        action: 'Ban',
        process: state.gameState.processes[0],
      }, {
        action: 'Promote',
        process: state.gameState.processes[1],
      }, {
        action: 'Promote',
        process: state.gameState.processes[2],
      }, {
        action: 'Ban',
        process: state.gameState.processes[3],
      }, {
        action: 'Ban',
        process: state.gameState.processes[4],
      }, {
        action: 'Promote',
        process: state.gameState.processes[5],
      }, {
        action: 'Promote',
        process: state.gameState.processes[6],
      }, {
        action: 'Ban',
        process: state.gameState.processes[8],
      }],
      chart: 'land',
      ranges: {
        x: [0, years],
        y: [0, 1],
      },
    }
  },
  computed: {
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
            .map((v) => v/totalLand);
          break;
        case 'emissions':
          data = historicalEmissions.concat(state.history.emissions)
            .map((v) => v/100);
          break;
      }
      return {
        data: data.map((y, i) => ({
          x: i,
          y: y
        })),
        color: '#BC6A58',
      }
    },
    projection() {
      return {
        data: [...Array(80).keys()].map((i) => ({
          x: i + this.historical.data.length,
          y: Math.random()
        })),
        color: '#CDB6AD'
      }
    }
  },
  methods: {
    setChart(key) {
      this.chart = key;
    }
  }
}
</script>

<style>
.plan--changes {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-around;
}
.plan--change {
  width: 80px;
  text-align: center;
  margin: 0.5em 0;
}
.plan--action {
  text-transform: uppercase;
  font-size: 0.85em;
}
.plan--cost img {
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
</style>
