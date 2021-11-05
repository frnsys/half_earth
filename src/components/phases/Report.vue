<template>
  <Hud />
  <Dialogue v-if="event && event.dialogue" :dialogue="event.dialogue" :effects="event.effects" @done="nextEvent" @select="selectChoice" />
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
          <td>Temperature</td>
          <td>{{state.cycleStartState.temperature.toFixed(1)}}</td>
          <td>{{state.gameState.world.temperature.toFixed(1)}}</td>
          <td>{{sign(politicalCapital.temperature)}}</td>
        </tr>
        <tr>
          <td>Contentedness</td>
          <td>{{state.cycleStartState.contentedness.toFixed(0)}}</td>
          <td>{{state.gameState.contentedness.toFixed(0)}}</td>
          <td>{{sign(politicalCapital.contentedness)}}</td>
        </tr>
        <tr>
          <td>Extinction Rate</td>
          <td>{{state.cycleStartState.extinctionRate.toFixed(0)}}</td>
          <td>{{state.gameState.world.extinction_rate.toFixed(0)}}</td>
          <td>{{sign(politicalCapital.extinctionRate)}}</td>
        </tr>
        <tr v-for="request in requestsFulfilled">
          <td colspan="3">{{request.name}}</td>
          <td>{{sign(request.bounty)}}</td>
        </tr>
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
    let events = game.rollReportEvents();
    return {
      state,
      events,
      politicalCapital: {}
    }
  },
  computed: {
    requestsFulfilled() {
      return game.check_requests().map(([kind, id, active, bounty]) => {
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
    }
  },
  methods: {
    calculateChanges() {
      this.politicalCapitalChange = 0;
      let temperatureChange = parseFloat(state.gameState.world.temperature.toFixed(1)) - parseFloat(state.cycleStartState.temperature.toFixed(1));
      let contentednessChange = parseFloat(state.gameState.contentedness.toFixed(0)) - parseFloat(state.cycleStartState.contentedness.toFixed(0));
      let extinctionRateChange = state.gameState.world.extinction_rate - state.cycleStartState.extinctionRate;
      this.politicalCapital = {
        temperature: Math.round(temperatureChange * -10),
        contentedness: Math.round(contentednessChange/3),
        extinctionRate: Math.round(-extinctionRateChange),
      };
      console.log('PC changes:');
      console.log(this.politicalCapital);
      this.politicalCapitalChange += this.politicalCapital.temperature;
      this.politicalCapitalChange += this.politicalCapital.contentedness;
      this.politicalCapitalChange += this.politicalCapital.extinctionRate;
    },
    nextPhase() {
      console.log(`Total change: ${this.politicalCapitalChange}`);
      game.changePoliticalCapital(this.politicalCapitalChange);

      let lost = state.gameState.political_capital <= 0;
      if (lost) {
        state.phase = 'BREAK';
      } else {
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
  text-align: right;
}
.report td:first-child {
  text-align: left;
}
.report button {
  display: block;
  margin: 1em auto;
}
.report--body {
  flex: 1;
}
</style>
