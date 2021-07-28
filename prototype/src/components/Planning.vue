<template>
  <h2 v-if="phase === 0">PLANNING.TARGETS</h2>
  <h2 v-else-if="phase === 1">PLANNING.RESEARCH</h2>
  <h2 v-else-if="phase === 2">PLANNING.HAND</h2>

  <div>Year: {{state.player.year}}</div>
  <div>Political Capital: {{state.player.political_capital}}</div>

  <div v-if="phase === 0">
    <ul>
      <li v-for="(d, vari) in state.plan.targets">
        <b>{{vari}}</b>
        <div>Current value: {{state.world[vari].value}}</div>
        <div>
          <input type="number" step="1"
            @change="() => calculatePCWager(vari)"
            v-model="state.plan.targets[vari].value">
          PC wager: <span class="pc-wager">{{state.plan.targets[vari].wager}}</span>
        </div>
      </li>
    </ul>
  </div>

  <div v-else-if="phase > 0">
    <div>Hand slots: {{state.player.hand.length + state.player.research.length}}/{{state.consts.MAX_HAND_SIZE}}</div>

    <div v-if="phase === 1">
      <ul>
        <li v-for="r in state.research">
          <Card @click="() => toggleResearch(r)">
            <div v-if="state.player.research.includes(r)">selected</div>
            {{r.name}}
          </Card>
        </li>
      </ul>
    </div>

    <div v-else-if="phase === 2">
      <ul>
        <li v-for="p in state.projects.filter((p) => p.unlocked)">
          <Card @click="() => toggleProject(p)">
            <div v-if="state.player.hand.includes(p)">selected</div>
            {{p.name}}
            <div>
              <b>Construction:</b>
              ⏳:{{p.construction.years}}
              <span v-for="(v, k) in p.construction.resources">
                <b>{{k}}</b>:{{v}}
              </span>
            </div>
            <div>
              <b>Operation:</b>
              <span v-for="(v, k) in p.operation.resources">
                <b>{{k}}</b>:{{v}}/⏳
              </span>
            </div>
          </Card>
        </li>
      </ul>
    </div>
  </div>

  <button @click="nextPhase">Done</button>
</template>

<script>
import state from '../state';
import Card from './Card.vue';
export default {
  data() {
    return {
      state,
      phase: 0
    };
  },
  components: {
    Card
  },
  methods: {
    nextPhase() {
      this.phase++;
      if (this.phase >= 3) {
        state.phase = 'IMPLEMENTATION';
      }
    },
    calculatePCWager(vari) {
      let val = state.plan.targets[vari].value;
      let wager = (val - state.world[vari].value)**2;
      state.plan.targets[vari].wager = wager;
    },
    toggleResearch(research) {
      if (state.player.research.includes(research)) {
        state.player.research = state.player.research.filter((r) => r != research);
      } else {
        state.player.research.push(research);
      }
    },
    toggleProject(project) {
      if (state.player.hand.includes(project)) {
        state.player.hand = state.player.hand.filter((p) => p != project);
      } else {
        state.player.hand.push(project);
      }
    }
  }
}
</script>

<style>
</style>
