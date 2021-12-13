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
          <td>{{format.sign(pc.temperature)}}</td>
        </tr>
        <tr>
          <td><img :src="icons.contentedness"> Contentedness</td>
          <td>
            <div class="intensity-pip stat-pip" :style="{background:contentedness.start.color}" v-for="i in contentedness.start.intensity"></div>
          </td>
          <td>
            <div class="intensity-pip stat-pip" :style="{background:contentedness.end.color}" v-for="i in contentedness.end.intensity"></div>
          </td>
          <td>{{format.sign(pc.contentedness)}}</td>
        </tr>
        <tr>
          <td><img :src="icons.extinction_rate"> Extinction Rate</td>
          <td>
            <div class="intensity-pip stat-pip" :style="{background:extinction.start.color}" v-for="i in extinction.start.intensity"></div>
          </td>
          <td>
            <div class="intensity-pip stat-pip" :style="{background:extinction.end.color}" v-for="i in extinction.end.intensity"></div>
          </td>
          <td>{{format.sign(pc.extinctionRate)}}</td>
        </tr>
        <tr>
          <td><img :src="icons.emissions"> Emissions</td>
          <td>{{state.cycleStartState.emissions.toFixed(0)}}</td>
          <td>{{state.gameState.emissions.toFixed(0)}}</td>
          <td>{{format.sign(pc.emissions)}}</td>
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
          <td>{{format.sign(consts.pcPerCompletedProject)}}</td>
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
          <td>{{format.sign(request.bounty)}}</td>
        </tr>
        <tr class="report-spacer"></tr>
        <tr>
          <td colspan="3">Total Change</td>
          <td>{{format.sign(pcChange)}}</td>
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
import intensity from '/src/display/intensity';
import consts from '/src/consts.js';
import Hud from 'components/Hud.vue';
import EventsMixin from 'components/EventsMixin';


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
      pc: {},
    }
  },
  computed: {
    requestsFulfilled() {
      return game.checkRequests().map(([kind, id, active, bounty]) => {
        let text;
        if (kind == 'Project') {
          let project = state.gameState.projects[id];
          text = `Completed Request: ${active ? 'Implement' : 'Stop'} ${project.name}`;
        } else if (kind == 'Process') {
          let process = state.gameState.processes[id];
          text = `Completed Request: ${active ? 'Unban' : 'Ban'} ${process.name}`;
        }
        this.pcChange += bounty;
        return {text, bounty};
      });
    },
    contentedness() {
      let start = intensity.scale(state.cycleStartState.contentedness, 'world_outlook');
      let end = intensity.scale(state.gameState.contentedness, 'world_outlook');
      return {
        start: {
          intensity: start,
          color: intensity.color(start, true)
        },
        end: {
          intensity: end,
          color: intensity.color(end, true)
        }
      }
    },
    extinction() {
      let start = intensity.scale(state.cycleStartState.extinctionRate, 'extinction');
      let end = intensity.scale(state.gameState.world.extinction_rate, 'extinction');
      return {
        start: {
          intensity: start,
          color: intensity.color(start, false)
        },
        end: {
          intensity: end,
          color: intensity.color(end, false)
        }
      }
    }
  },
  methods: {
    calculateChanges() {
      this.pcChange = 0;
      let temperatureChange = parseFloat(state.gameState.world.temperature.toFixed(1)) - parseFloat(state.cycleStartState.temperature.toFixed(1));
      let contentednessChange = consts.contentednessPc[this.contentedness.end.intensity] || consts.contentednessPc[consts.contentednessPc.length - 1];
      let extinctionRateChange = consts.extinctionPc[this.extinction.end.intensity] || consts.extinctionPc[consts.extinctionPc.length - 1];
      let emissionsChange = state.gameState.emissions - state.cycleStartState.emissions;
      this.pc = {
        temperature: Math.round(temperatureChange * -10),
        contentedness: Math.round(contentednessChange),
        extinctionRate: Math.round(extinctionRateChange),
        emissions: Math.round(-emissionsChange),
      };
      this.pcChange += Object.values(this.pc).reduce((a,b) => a + b, 0);
      this.pcChange += state.cycleStartState.completedProjects.length * consts.pcPerCompletedProject;
    },
    updateProcessMix(output) {
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
            totalChanges--;
          } else if (change > 0 && addPoints > 0) {
            addPoints -= 1;
            changes[processId] -= 1;
            game.changeProcessMixShare(processId, 1);
            totalChanges--;
          }
        });
      }
    },
    nextPhase() {
      game.changePoliticalCapital(this.pcChange);

      if (state.gameState.game_over) {
        game.saveMeta();
        state.phase = 'BREAK';
      } else if (state.gameState.world.year >= state.endYear) {
        game.saveMeta();
        state.phase = 'END';
      } else {
        // Apply process mix changes
        Object.keys(state.processMixChanges).forEach((output) => {
          this.updateProcessMix(output);
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
