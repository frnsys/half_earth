<template>
  <Hud />
  <Dialogue v-if="hasDialogue" v-bind="event" @done="nextEvent" />
  <div class="report">
    <div class="report-overlay" :class="{gameOver: gameOver, gameWin: gameWin}"></div>
    <h2>{{reportTitle}}</h2>
    <div class="report--body">
      <div class="report--inner">
        <section>
      <table class="report--changes">
        <tr>
          <th><strong>Changes</strong></th>
          <th><strong>{{state.cycleStartState.year}}</strong></th>
          <th><small><img :src="icons.arrow_right"></small></th>
          <th><strong>{{state.gameState.world.year}}</strong></th>
          <th><img src="/assets/icons/pips/political_capital.png"></th>
        </tr>

        <tr class="report--primary-change" v-tip="warmingTip">
          <td><img :src="icons.warming"> Temperature</td>
          <td>+{{state.cycleStartState.temperature.toFixed(1)}}°c</td>
          <th><small><img :src="icons.arrow_right"></small></th>
          <td>+{{state.gameState.world.temperature.toFixed(1)}}°c</td>
          <td><strong>{{format.sign(pc.temperature)}}</strong></td>
        </tr>
        <tr class="report--primary-change" v-tip="contentednessTip">
          <td><img :src="icons.contentedness"> Contentedness</td>
          <td>
            <IntensityBar :intensity="contentedness.start" :max="5" :invert="true" />
          </td>
          <th><small><img :src="icons.arrow_right"></small></th>
          <td>
            <IntensityBar :intensity="contentedness.end" :max="5" :invert="true" />
          </td>
          <td><strong>{{format.sign(pc.contentedness)}}</strong></td>
        </tr>
        <tr class="report--primary-change" v-tip="biodiversityTip">
          <td><img :src="icons.extinction_rate"> Extinction Rate</td>
          <td>
            <IntensityBar :intensity="extinction.start" :max="5" />
          </td>
          <th><small><img :src="icons.arrow_right"></small></th>
          <td>
            <IntensityBar :intensity="extinction.end" :max="5" />
          </td>
          <td><strong>{{format.sign(pc.extinctionRate)}}</strong></td>
        </tr>
        <tr class="report--primary-change" v-tip="emissionsTip">
          <td><img :src="icons.emissions"> Emissions</td>
          <td>{{state.cycleStartState.emissions.toFixed(1)}}</td>
          <th><small><img :src="icons.arrow_right"></small></th>
          <td>{{state.gameState.world.emissions.toFixed(1)}}</td>
          <td><strong>{{format.sign(pc.emissions)}}</strong></td>
        </tr>
        <tr class="report--primary-change" v-if="honeymoonPc">
          <td>Post-Revolution Optimism</td>
          <td></td>
          <td></td>
          <td></td>
          <td><strong>{{format.sign(honeymoonPc)}}</strong></td>
        </tr>
        <tr class="report-spacer"></tr>
        <tr class="report-header" v-if="state.cycleStartState.completedProjects.length != 0">
          <td>Completed Projects</td>
        </tr>

        <tr v-for="project in state.cycleStartState.completedProjects">
          <td colspan="4">{{state.gameState.projects[project].name}}</td>
          <td><strong>{{format.sign(consts.pcPerCompletedProject)}}</strong></td>
        </tr>
        <tr class="report-spacer" v-if="state.cycleStartState.completedProjects.length != 0"></tr>
        <tr class="report-header" v-if="requestsFulfilled.length != 0">
          <td>Completed Requests</td>
        </tr>
        <tr v-for="request in requestsFulfilled">
          <td colspan="4">{{request.name}}</td>
          <td><strong>{{format.sign(request.bounty)}}</strong></td>
        </tr>
        <tr class="report-spacer" v-if="requestsFulfilled.length != 0"></tr>
        <tr class="report--total-change">
          <td colspan="4">Total <img src="/assets/icons/pips/political_capital.png"> Change</td>
          <td>{{format.sign(pcChange)}}</td>
        </tr>
        </table>
        </section>
        <section>
        <table>
        <tr class="report-header" v-if="seatChanges.length != 0">
          <td>Parliament</td>
        </tr>
        <tr v-for="npc in seatChanges">
          <td colspan="2">{{npc.name}}</td>
          <td>{{format.sign(npc.change)}}</td>
          <td>{{npc.seats}}</td>
        </tr>
        <tr class="report-spacer" v-if="seatChanges.length != 0"></tr>
        <tr class="report-header" v-if="regionIncomeChanges.length != 0">
          <td>Regions</td>
        </tr>
        <tr v-for="r in regionIncomeChanges">
          <td colspan="4">{{r.name}} is now {{display.enumDisplay(r.income, true)}} income.</td>
        </tr>
        <tr class="report-spacer" v-if="regionIncomeChanges.length != 0"></tr>
        <tr class="report-header" v-if="regionDisasters.length != 0">
          <td>Disasters</td>
        </tr>
        <tr v-for="r in regionDisasters">
          <td>{{r.name}}</td>
          <td colspan="4" class="report-disasters">
            <img :src="icons[ev.icon]" v-for="ev in r.events">
          </td>
        </tr>

      </table>
        </section>
      <button class="btn" :class="{gameOver: gameOver, gameWin: gameWin}" @click="nextPhase">{{buttonText}}</button>
      </div>
    </div>
  </div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
import intensity from '/src/display/intensity';
import Hud from 'components/Hud.vue';
import EventsMixin from 'components/EventsMixin';
import IntensityBar from 'components/cards/IntensityBar.vue';


export default {
  mixins: [EventsMixin],
  components: {
    Hud,
    IntensityBar
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
    gameOver() {
      return state.gameState.game_over;
    },
    gameWin() {
      return state.gameState.world.year >= state.endYear;
    },
    reportTitle() {
      if (this.gameOver) {
        return 'Game Over';
      } else if (this.gameWin) {
        return 'Success!';
      } else {
        return 'Report';
      }
    },
    buttonText() {
      if (this.gameOver) {
        return 'Game Over';
      } else if (this.gameWin) {
        return 'Success!';
      } else {
        return 'Next';
      }
    },
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
      let end = intensity.scale(state.gameState.world.contentedness, 'world_outlook');
      return {
        start,
        end,
      }
    },
    extinction() {
      let start = intensity.scale(state.cycleStartState.extinctionRate, 'extinction');
      let end = intensity.scale(state.gameState.world.extinction_rate, 'extinction');
      return {
        start,
        end,
      }
    },
    regionIncomeChanges() {
      return state.gameState.world.regions.filter((r, i) => r.income != state.cycleStartState.regionIncomes[i]);
    },
    regionDisasters() {
      return Object.keys(state.annualRegionEvents).map((id) => {
        return {
          name: state.gameState.world.regions[id].name,
          events: state.annualRegionEvents[id],
        }
      });
    },
    seatChanges() {
      return state.cycleStartState.parliament.map((startSeats, i) => {
        let npc = state.gameState.npcs[i];
        return {
          name: npc.name,
          seats: npc.seats,
          change: Math.round(npc.seats - startSeats),
        };
      }).filter((npc) => npc.change !== 0);
    },
    honeymoonPc() {
      if (state.gameState.world.year < state.startYear + consts.honeymoonYears) {
        return consts.honeymoonPc;
      } else {
        return 0;
      }
    },
    warmingTip() {
      return {
        icon: 'warming',
        text: `The current global temperature anomaly. <strong>Increased warming</strong> will damage your political capital.`
      };
    },
    biodiversityTip() {
      return {
        icon: 'extinction_rate',
        text: `The current biodiversity pressure. <strong>Increased biodiversity pressure</strong> will cost you political capital.`
      };
    },
    contentednessTip() {
      return {
        icon: 'contentedness',
        text: `How people around the world feel about the state of things. <strong>Increasing or maintaining contentedness</strong> will gain you political capital.`
      };
    },
    emissionsTip() {
      return {
        icon: 'emissions',
        text: `Current annual emissions, in gigatonnes of CO2 equivalent. <strong>Reducing emissions</strong> will gain you political capital.`
      };
    }
  },
  methods: {
    calculateChanges() {
      this.pcChange = 0;
      let temperatureChange = parseFloat(state.gameState.world.temperature.toFixed(1)) - parseFloat(state.cycleStartState.temperature.toFixed(1));
      let contentednessChange = consts.contentednessPc[this.contentedness.end] || consts.contentednessPc[consts.contentednessPc.length - 1];
      let extinctionRateChange = consts.extinctionPc[this.extinction.end] || consts.extinctionPc[consts.extinctionPc.length - 1];
      let emissionsChange = state.gameState.world.emissions - state.cycleStartState.emissions;

      // Double temp change score for every degree above 1C
      let tempChangeMultiplier = Math.max(1, Math.max(0, Math.round(state.gameState.world.temperature) - 1) * 2);
      this.pc = {
        // Temp scored for every 0.1C change
        temperature: Math.round(temperatureChange * 10) * -consts.temperaturePc * tempChangeMultiplier,
        contentedness: Math.round(contentednessChange),
        extinctionRate: Math.round(extinctionRateChange),
        emissions: Math.round(emissionsChange * 2) * -consts.emissionsPc,
      };
      this.pcChange += Object.values(this.pc).reduce((a,b) => a + b, 0);
      this.pcChange += state.cycleStartState.completedProjects.length * consts.pcPerCompletedProject;
      this.pcChange += this.honeymoonPc;
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
    upgradeProjects() {
      Object.keys(state.queuedUpgrades).forEach((id) => {
        if (state.queuedUpgrades[id]) {
          game.upgradeProject(id);
          state.queuedUpgrades[id] = false;
        }
      });
    },
    nextPhase() {
      game.changePoliticalCapital(this.pcChange);

      if (this.gameOver) {
        game.saveMeta();
        state.phase = 'GAMEOVER';
      } else if (this.gameWin) {
        game.saveMeta();
        state.phase = 'GAMEWIN';
      } else {
        // Apply process mix changes
        Object.keys(state.processMixChanges).forEach((output) => {
          this.updateProcessMix(output);
        });
        // Apply project upgrades
        this.upgradeProjects();
        state.refundableResearchPoints = 0;
        state.phase = 'PLANNING';

        // Reset session plan changes
        state.planChanges = {};
      }
    }
  }
}
</script>

<style>
.report {
  background-image: url('/assets/backgrounds/report.png');
  background-colour: #FFF7D9;
  color: #000;
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow-y: scroll;
  background-size: cover;
  background-repeat: no-repeat;
  background-position: center center;
  image-rendering: pixelated;
}
.report h2 {
  font-weight: normal;
  text-align: center;
  position: relative;
  /* border-bottom: 1px solid; */
}

.report--total-change{
  font-weight: 600;
}

.report--inner{
  padding: calc(1rem - 4px);
  box-shadow: inset -1px -1px 0px rgb(0 0 0 / 50%);
  border-left: 1px solid rgba(255,255,255,0.5);
  border-top: 1px solid rgba(255,255,255,0.5);
  background-color: #FFF7D9;
  max-width: 360px;
  margin: 0 auto;
  border-radius: 1rem;
  background-image: url('/assets/watermark.png');
  background-repeat: no-repeat;
  background-position: center center;
  background-size: 200px;
}

.report section{
  padding: 0.5rem;
}

.report table {
  width: 100%;
  border-collapse: collapse;

  font-family: 'Inter';
  font-size: 0.8rem;

  image-rendering: auto;
}
.report th {
  font-weight: normal;
  padding: 4px;
}

.report th:first-of-type{
  padding-left: 0;
}

.report td{
  padding: 4px;
}

.report td:first-of-type{
  padding-left: 0;
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
  /* margin: 1em auto; */
  font-family: 'Times Ten', serif;
  position:sticky !important;
  bottom: 1rem;
  width: 100%;
}
.report--body {
  flex: 1;
  position: relative;
}

.report-empty {
  color: rgba(0,0,0,0.5);
}
.report-header {
  border-bottom: 1px solid rgba(0,0,0,1);
  /* font-weight: 600; */
}
.report-spacer {
  height: 8px;
}

.report--primary-change {
}
.report--primary-change td{
  padding-top:6px;
  padding-bottom: 6px;
  /* font-weight: 600; */
}

.report-disasters img {
  margin-right: 2px;
  height: 18px;
}
.report small{
  font-size:0.6rem;
  margin: 0 0.5em;
  opacity: 0.5;
}
.report small img {
  height: 12px;
  margin-top: -5px;
}
.report tr:first-child small img {
  margin-left: -4px;
}

.report-overlay {
  position: fixed;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
}
.report-overlay.gameOver {
  background: #ef38383b;
}
.report-overlay .gameWin {
  background: #ff66ff3d;
}

.report--body .btn.gameOver {
  background: red;
  color: #fff;
}
.report--body .btn.gameWin {
  background: #43cc70;
  color: #fff;
}

@media only screen and (min-width: 481px) {
  .report--inner{
    max-width: 430px;
  }
  .report table {
    font-size: 0.9rem;
  }
}
</style>
