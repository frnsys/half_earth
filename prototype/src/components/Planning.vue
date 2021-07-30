<template>
  <div v-if="phase === 0">
    <h2>PLANNING.TARGETS</h2>
    <p class="help">Set targets for the next five years. Harder targets can earn you more PC, but also risk losing more. Backtracking on a target has a PC cost.</p>
  </div>
  <div v-else-if="phase === 1">
    <h2>PLANNING.RESEARCH</h2>
    <p class="help">Set ongoing research initiatives. These use hand slots.</p>
  </div>
  <div v-else-if="phase === 2">
    <h2>PLANNING.HAND</h2>
    <p class="help">Choose what projects to have prepared for the next five years. Unpopular projects have a PC cost to put into your hand.</p>
  </div>

  <div class="stats">
    <div>Year: {{state.player.year}}</div>
    <div>Political Capital: {{state.player.political_capital}}</div>
  </div>

  <div v-if="phase === 0">
    <ul>
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

  <div v-else-if="phase > 0">
    <div class="hand-slots">Hand: <span v-for="(_, i) in state.consts.MAX_HAND_SIZE">{{i < handSize() ? '▮' : '▯'}}</span></div>

    <div v-if="phase === 1">
      <ul>
        <li v-for="r in state.research">
          <Card @click="() => toggleResearch(r)" :class="{selected: state.player.research.includes(r)}">
            {{r.name}}
            <div>Estimate: {{r.estimate ? `${r.estimate} years` : '🎲'}}</div>
          </Card>
        </li>
      </ul>
    </div>

    <div v-else-if="phase === 2">
      <ul>
        <li v-for="p in state.projects.filter((p) => p.unlocked)">
          <Project @click="() => toggleProject(p)" :class="{selected: state.player.hand.includes(p)}" :project="p">
            <template v-slot:costs>
              <div class="meta meta-top">
                <div v-if="p.popularity < 0">😡5PC</div>
              </div>
            </template>
          </Project>
        </li>
      </ul>
    </div>
  </div>

  <div class="actions">
    <button @click="prevPhase" v-if="phase > 0">Back</button>
    <button @click="nextPhase">Done</button>
  </div>
</template>

<script>
import state from '../state';
import Card from './Card.vue';
import Project from './Project.vue';
export default {
  data() {
    return {
      state,
      phase: 0
    };
  },
  components: {
    Card,
    Project
  },
  methods: {
    handSize() {
      return state.player.hand.length + state.player.research.length;
    },
    prevPhase() {
      if (this.phase > 0) {
        this.phase--;
      }
    },
    nextPhase() {
      if (this.phase < 2) {
        this.phase++;
      } else {
        let availableCards = state.projects.filter((p) => p.unlocked).length + state.research.length;
        let maxCards = Math.min(state.consts.MAX_HAND_SIZE, availableCards);
        if (this.handSize() == maxCards || confirm('Your hand is undersized, continue?')) {
          state.phase = 'IMPLEMENTATION';
        }
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
    toggleResearch(research) {
      if (state.player.research.includes(research)) {
        state.player.research = state.player.research.filter((r) => r != research);
      } else {
        state.player.research.push(research);
      }
    },
    toggleProject(project) {
      // TODO hard-coded unpopular cost of 5pc
      if (state.player.hand.includes(project)) {
        state.player.hand = state.player.hand.filter((p) => p != project);
        if (project.popularity < 0) {
          state.player.political_capital += 5;
        }
      } else {
        if (project.popularity < 0 && state.player.political_capital >= 5) {
          state.player.hand.push(project);
          state.player.political_capital -= 5;
        } else {
          state.player.hand.push(project);
        }
      }
    }
  }
}
</script>

<style scoped>
ul {
  display: flex;
  justify-content: space-around;
}
ul .card {
  margin: 0.5em;
}
.hand-slots {
  text-align: center;
}
</style>
