<template>
  <Setting
      background="assets/settings/bgs/redwood_forest.png"
      audio="assets/settings/audio/463903__burghrecords__birds-in-spring-scotland.mp3" />

  <Hud />

  <Window :title="`Planning - ${phase === 0 ? 'Targets' : 'Projects'}`">
    <div v-if="phase === 0">
      <p class="help">Set targets for the next five years. Harder targets can earn you more PC, but also risk losing more. Backtracking on a target has a PC cost.</p>
      <ul class="planning--targets">
        <li v-for="(d, vari) in state.plan.targets">
          <b>{{VARI_ICONS[vari]}}{{vari}}</b>
          <div>Current world value: {{state.world[vari].value}}</div>
          <div>
            <input type="number" step="1"
              @change="() => calculatePCWager(vari)"
              v-model="state.plan.targets[vari].value">
            <span> or {{ state.plan.targets[vari].valence > 0 ? 'higher' : 'lower' }}</span>
          </div>
          <div>
            <span v-if="state.plan.targets[vari].wager < 0">PC penalty: </span>
            <span v-else>PC wager: </span>
            <span class="pc-wager">{{state.plan.targets[vari].wager}}</span>
          </div>
        </li>
      </ul>
    </div>

    <div v-else-if="phase === 1">
      <p class="help">Set ongoing research initiatives, projects, and policies.</p>
      <ul class="planning--projects">
        <li v-for="p in state.projects">
          <Project @click="() => toggleProject(p)" :class="{selected: state.player.projects.includes(p)}" :project="p">
            <template v-slot:costs>
              <div class="meta meta-top">
                <div v-if="p.popularity < 0">😡{{pcCost(p)}}PC</div>
              </div>
            </template>
          </Project>
        </li>
      </ul>

      <Resources />
    </div>

    <div class="actions">
      <button @click="prevPhase" v-if="phase > 0">Back</button>
      <button @click="nextPhase">Done</button>
    </div>
  </Window>
</template>

<script>
import state from '../state';
import Hud from './Hud.vue';
import Resources from './Resources.vue';
import Window from './Window.vue';
import Project from './Project.vue';
import Setting from './Setting.vue';
export default {
  data() {
    return {
      state,
      phase: 0
    };
  },
  components: {
    Hud,
    Project,
    Setting,
    Window,
    Resources
  },
  methods: {
    prevPhase() {
      if (this.phase > 0) {
        this.phase--;
      }
    },
    nextPhase() {
      if (this.phase < 1) {
        this.phase++;
      } else {
        state.phase = 'IMPLEMENTATION';
      }
    },
    calculatePCWager(vari) {
      let val = state.plan.targets[vari].value;
      let valence = state.plan.targets[vari].valence;
      let mult = 1;
      if (val * valence < state.world[vari].value * valence) { // Penalty
        mult = -1;
      }
      let wager = (val - state.world[vari].value)**2;
      state.plan.targets[vari].wager = wager * mult;
    },
    pcCost(project) {
      // TODO better calculation
      return project.popularity < 0 ? project.popularity * 5 : 0;
    },
    toggleProject(project) {
      let pcCost = this.pcCost(project);
      if (state.player.projects.includes(project)) {
        state.player.projects = state.player.projects.filter((p) => p != project);
        if (project.popularity < 0) {
          state.player.political_capital += pcCost;
        }
      } else {
        if (project.popularity < 0 && state.player.political_capital >= pcCost) {
          state.player.projects.push(project);
          state.player.political_capital -= pcCost;
        } else {
          state.player.projects.push(project);
        }
      }
    }
  }
}
</script>

<style>
.planning--targets {
  display: flex;
  justify-content: space-between;
  flex-wrap: wrap;
}
</style>
