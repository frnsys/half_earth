<template>
  <Hud />
  <Dialogue v-if="hasDialogue" v-bind="event" @done="nextEvent" />
  <div class="report">
    <h2>Report</h2>
    <div class="report--body">
      <table>
        <tr>
          <th></th>
          <th>{{state.cycleStartState.year}}</th>
          <th>{{state.gameState.world.year}}</th>
          <th><img src="/assets/icons/pips/political_capital.png"></th>
        </tr>
        <tr>
          <td><img :src="icons.warming"> Temperature</td>
          <td>{{state.cycleStartState.temperature.toFixed(1)}}</td>
          <td>{{state.gameState.world.temperature.toFixed(1)}}</td>
          <td>{{sign(politicalCapital.temperature)}}</td>
        </tr>
        <tr>
          <td><img :src="icons.contentedness"> Contentedness</td>
          <td>
            <div class="intensity-pip stat-pip" :style="{background:contentedness.start.color}" v-for="i in contentedness.start.intensity"></div>
          </td>
          <td>
            <div class="intensity-pip stat-pip" :style="{background:contentedness.end.color}" v-for="i in contentedness.end.intensity"></div>
          </td>
          <td>{{sign(politicalCapital.contentedness)}}</td>
        </tr>
        <tr>
          <td><img :src="icons.extinction_rate"> Extinction Rate</td>
          <td>
            <div class="intensity-pip stat-pip" :style="{background:extinction.start.color}" v-for="i in extinction.start.intensity"></div>
          </td>
          <td>
            <div class="intensity-pip stat-pip" :style="{background:extinction.end.color}" v-for="i in extinction.end.intensity"></div>
          </td>
          <td>{{sign(politicalCapital.extinctionRate)}}</td>
        </tr>
        <tr>
          <td><img :src="icons.emissions"> Emissions</td>
          <td>{{state.cycleStartState.emissions.toFixed(0)}}</td>
          <td>{{state.gameState.emissions.toFixed(0)}}</td>
          <td>{{sign(politicalCapital.emissions)}}</td>
        </tr>
        <tr class="report-spacer"></tr>
        <tr class="report-header">
          <td>Completed Projects</td>
        </tr>
        <tr v-if="state.cycleStartState.completedProjects.length === 0">
          <td class="report-empty">None</td>
        </tr>
        <tr v-for="project in state.cycleStartState.completedProjects">
          <td colspan="3">{{state.gameState.projects[project].name}}</td>
          <td>{{sign(PC_PER_COMPLETED_PROJECT)}}</td>
        </tr>
        <tr class="report-spacer"></tr>
        <tr class="report-header">
          <td>Completed Requests</td>
        </tr>
        <tr v-if="requestsFulfilled.length === 0">
          <td class="report-empty">None</td>
        </tr>
        <tr v-for="request in requestsFulfilled">
          <td colspan="3">{{request.name}}</td>
          <td>{{sign(request.bounty)}}</td>
        </tr>
        <tr class="report-spacer"></tr>
        <tr>
          <td colspan="3">Total Change</td>
          <td>{{sign(politicalCapitalChange)}}</td>
        </tr>
      </table>
    </div>
    <h2 v-if="lost">you lost</h2>
    <button @click="nextPhase">Next</button>
  </div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import display from 'lib/display';
import consts from '/src/consts.js';
import Hud from 'components/Hud.vue';
import EventsMixin from 'components/EventsMixin';

const PC_PER_COMPLETED_PROJECT = 20;
const CONTENTEDNESS_PC = [0, 0, 5, 10, 20];
const EXTINCTION_PC = [20, 10, 0, -5, -5, -10];

export default {
  mixins: [EventsMixin],
  components: {
    Hud
  },
  mounted() {
    this.showEvent();
    this.calculateChanges();
  },
  activated() {
    this.showEvent();
    this.calculateChanges();
  },
  data() {
    let events = game.roll.report('Start');
    return {
      state,
      events,
      politicalCapital: {},
      PC_PER_COMPLETED_PROJECT,
    }
  },
  computed: {
    requestsFulfilled() {
      return game.checkRequests().map(([kind, id, active, bounty]) => {
        // TODO should show who gave it to you?
        let text;
        if (kind == 'Project') {
          let project = state.gameState.projects[id];
          text = `Completed Request: ${active ? 'Implement' : 'Stop'} ${project.name}`;
        } else if (kind == 'Process') {
          let process = state.gameState.processes[id];
          text = `Completed Request: ${active ? 'Unban' : 'Ban'} ${process.name}`;
        }
        this.politicalCapitalChange += bounty;
        return {text, bounty};
      });
    },
    contentedness() {
      let start = display.scaleIntensity(state.cycleStartState.contentedness, 'world_outlook');
      let end = display.scaleIntensity(state.gameState.contentedness, 'world_outlook');
      return {
        start: {
          intensity: start,
          color: display.intensityColor(start, true)
        },
        end: {
          intensity: end,
          color: display.intensityColor(end, true)
        }
      }
    },
    extinction() {
      let start = display.scaleIntensity(state.cycleStartState.extinctionRate, 'extinction');
      let end = display.scaleIntensity(state.gameState.world.extinction_rate, 'extinction');
      return {
        start: {
          intensity: start,
          color: display.intensityColor(start, false)
        },
        end: {
          intensity: end,
          color: display.intensityColor(end, false)
        }
      }
    }
  },
  methods: {
    calculateChanges() {
      this.politicalCapitalChange = 0;
      let temperatureChange = parseFloat(state.gameState.world.temperature.toFixed(1)) - parseFloat(state.cycleStartState.temperature.toFixed(1));
      let contentednessChange = CONTENTEDNESS_PC[this.contentedness.end.intensity];
      let extinctionRateChange = EXTINCTION_PC[this.extinction.end.intensity];
      let emissionsChange = state.gameState.emissions - state.cycleStartState.emissions;
      this.politicalCapital = {
        temperature: Math.round(temperatureChange * -10),
        contentedness: Math.round(contentednessChange),
        extinctionRate: Math.round(extinctionRateChange),
        emissions: Math.round(-emissionsChange),
      };
      this.politicalCapitalChange += this.politicalCapital.temperature;
      this.politicalCapitalChange += this.politicalCapital.contentedness;
      this.politicalCapitalChange += this.politicalCapital.extinctionRate;
      this.politicalCapitalChange += this.politicalCapital.emissions;
      this.politicalCapitalChange += state.cycleStartState.completedProjects.length * PC_PER_COMPLETED_PROJECT;
    },
    nextPhase() {
      console.log(`Total change: ${this.politicalCapitalChange}`);
      game.changePoliticalCapital(this.politicalCapitalChange);

      if (state.gameState.game_over) {
        game.saveMeta();
        state.phase = 'BREAK';
      } else if (state.gameState.world.year >= state.endYear) {
        game.saveMeta();
        state.phase = 'END';
      } else {
        // Apply process mix changes
        Object.keys(state.processMixChanges).forEach((output) => {
          // TODO This can probably be cleaned up
          let removePoints = consts.processPointsPerCycle;
          let addPoints = consts.processPointsPerCycle;
          let changes = state.processMixChanges[output];
          let totalChanges = Object.values(state.processMixChanges[output]).reduce((acc, change) => {
            return acc + Math.abs(change);
          }, 0);
          while (removePoints > 0 && addPoints > 0 && totalChanges > 0) {
            Object.keys(changes).forEach((processId) => {
              let change = changes[processId]
              if (change < 0 && removePoints > 0) {
                changes[processId] += 1;
                removePoints -= 1;
                game.changeProcessMixShare(processId, -1);
              } else if (change > 0 && addPoints > 0) {
                addPoints -= 1;
                changes[processId] -= 1;
                game.changeProcessMixShare(processId, 1);
              }
            });
            totalChanges = Object.values(state.processMixChanges[output]).reduce((acc, change) => {
              return acc + Math.abs(change);
            }, 0);
          }
        });
        state.phase = 'PLANNING';
      }
    }
  }
}
</script>

<style>
.report {
  background: wheat;
  color: #000;
  flex: 1;
  display: flex;
  flex-direction: column;
}
.report h2 {
  font-family: 'Andada Pro';
  font-weight: normal;
  text-align: center;
  border-bottom: 1px solid;
}
.report table {
  width: 100%;
  max-width: 360px;
  margin: 0 auto;
  border-collapse: collapse;
}
.report th {
  font-weight: normal;
}
.report img {
  height: 20px;
  vertical-align: middle;
}
.report td,
.report th {
  text-align: left;
}
.report button {
  display: block;
  margin: 1em auto;
}
.report--body {
  flex: 1;
}

.report-empty {
  color: #888;
}
.report-header {
  border-bottom: 1px solid #000;
}
.report-spacer {
  height: 12px;
}
</style>
