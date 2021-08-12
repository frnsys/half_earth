<template>
  <Setting
      background="assets/settings/bgs/redwood_forest.png"
      audio="assets/settings/audio/463903__burghrecords__birds-in-spring-scotland.mp3" />

  <Hud />

  <Window :title="`${state.player.year}-${state.player.year+5} Planning - ${phase === 0 ? 'Targets' : 'Projects'}`">
    <div v-if="phase === 0" :set="vari = targetVars[targetVar]" class="planning--target">
      <h2>{{VARI_ICONS[vari]}}{{vari}}</h2>
      <Fader
        :steps="10"
        :value="state.plan.targets[vari].value"
        :current="state.world[vari].value"
        :reverse="state.plan.targets[vari].valence < 0"
        :minLabel="state.world[vari].labels.min"
        :maxLabel="state.world[vari].labels.max"
        @change="(value) => updateTarget(vari, value)"
        />
      <div class="planning--ambition">
        <span v-if="state.plan.targets[vari].wager < 0" class="planning--warning">Regression</span>
        <template v-else>
          <span v-if="state.plan.targets[vari].wager < 1">Business as Usual</span>
          <span v-else-if="state.plan.targets[vari].wager < 4">Milquetoast</span>
          <span v-else-if="state.plan.targets[vari].wager < 9">Modest</span>
          <span v-else-if="state.plan.targets[vari].wager < 16">Ambitious</span>
          <span v-else>Impossible</span>
        </template>
        <div class="planning--pc-wager">
          <template v-if="state.plan.targets[vari].wager < 0">
            {{state.plan.targets[vari].wager}}PC penalty
            <Tip>People will not like this backtracking and you'll lose support.</Tip>
          </template>
          <template v-else>
            {{state.plan.targets[vari].wager}}PC stake
            <Tip>If you reach this target, you'll earn this much PC. If you fail, you'll lose this much.</Tip>
          </template>
        </div>
      </div>
      <figure>
        <Projection
          :startYear="state.time.start"
          :endYear="state.time.end"
          :pastValues="state.world[vari].history.concat(state.world[vari].value)"
          :currentTargetValue="state.plan.targets[vari].value"
          :finalTargetValue="state.world[vari].preindustrial" />
      </figure>
      <p class="help">
        Set targets for the next five years. Harder targets can earn you more PC, but also risk losing more. Backtracking on a target has a PC cost.
      </p>
      <p class="help">
        Should backtracking targets be allowed at all? Can you only set targets in the "forward" direction from the current value?
      </p>
    </div>

    <div v-else-if="phase === 1">
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

      <p class="help">Set ongoing research initiatives, projects, and policies.</p>
    </div>

    <div class="actions">
      <button @click="prev" v-if="phase > 0">Back</button>
      <button @click="prev" v-if="phase == 0 && targetVar > 0">Back</button>
      <button @click="next" v-if="phase < 1">Next</button>
      <button @click="next" v-if="phase == 1">Done</button>
    </div>
  </Window>
</template>

<script>
import state from '../../state';
import Hud from '../Hud.vue';
import Window from '../Window.vue';
import Fader from './Fader.vue';
import Project from './Project.vue';
import Projection from './Projection.vue';
import Setting from '../Setting.vue';
import Tip from '../Tip.vue';

const targetVars = Object.keys(state.plan.targets);

export default {
  created() {
    this.targetVars = targetVars;
  },
  data() {
    return {
      state,
      phase: 0,
      targetVar: 0
    };
  },
  components: {
    Hud,
    Tip,
    Project,
    Setting,
    Window,
    Fader,
    Projection
  },
  methods: {
    prev() {
      if (this.phase > 0) {
        this.phase--;
      } else {
        this.targetVar--;
      }
    },
    next() {
      if (this.phase < 1) {
        if (this.targetVar >= targetVars.length - 1) {
          this.phase++;
        } else {
          this.targetVar++;
        }
      } else {
        state.phase = 'IMPLEMENTATION';
      }
    },
    updateTarget(vari, value) {
      this.state.plan.targets[vari].value = value;
      this.calculatePCWager(vari);
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
.planning--target h2 {
  text-align: center;
  font-weight: normal;
  margin: 0;
  font-size: 1em;
}
.planning--ambition {
  text-align: center;
  margin: 0.5em 0 1em 0;
  font-size: 0.9em;
}
.planning--pc-wager {
  text-align: center;
  font-size: 0.8em;
  color: #888;
}
.planning--warning {
  color: red;
}
.planning--projects {
  white-space: nowrap;
  overflow-x: scroll;
  width: 100%;
}
.planning--projects li {
  display: inline-block;
  margin: 0 0.5em;
  vertical-align: top;
}
</style>
