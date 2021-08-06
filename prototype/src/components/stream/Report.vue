<template>
  <h2>REPORT</h2>

  <div class="stats">
    <div>Year: {{state.player.year}}</div>
    <div>Political Capital: {{state.player.political_capital}}</div>
  </div>

  <ul class="results bar">
    <li v-for="(d, vari) in state.world">
      <b>{{vari}}</b>:
      <span v-if="vari in state.plan.targets" :class="{achieved: d.value * state.plan.targets[vari].valence >= state.plan.targets[vari].value * state.plan.targets[vari].valence}">{{d.value}}/{{state.plan.targets[vari].value}}</span>
      <span v-else>{{d.value}}</span>

      <div v-if="vari == 'contentedness'">
        <div v-if="pcChanges[vari] === 0">No Change</div>
        <div class="good" v-else-if="pcChanges[vari] > 0">Bonus</div>
        <div class="bad" v-else>Penalty</div>
      </div>
      <div v-else>
        <div class="good" v-if="pcChanges[vari] > 0">Achieved</div>
        <div class="bad" v-else>Failed</div>
      </div>
      <div>{{pcChanges[vari] >= 0 ? '+' : '-'}}{{Math.abs(pcChanges[vari])}} PC</div>
    </li>
  </ul>

  <div class="result" v-if="state.player.political_capital < 0">Sorry, you lose.</div>
  <div class="actions" v-else>
    <button @click="next">Next</button>
  </div>
</template>

<script>
import state from '../../state';
export default {
  data() {
    let pcChanges = Object.keys(state.plan.targets).reduce((acc, vari) => {
      let val = state.world[vari].value;
      let target = state.plan.targets[vari].value;
      let valence = state.plan.targets[vari].valence;
      let diff = (val - target) * valence;
      let pc = diff**2 * ((diff < 0) ? -1 : 1);
      acc[vari] = pc;
      return acc;
    }, {});

    let contentChange = state.world['contentedness'].value - state.plan.contentedness;
    let contentPC = contentChange**2 * (contentChange < 0 ? -1 : 1);
    pcChanges['contentedness'] = contentPC;

    let change = Object.values(pcChanges).reduce((acc, v) => acc + v, 0);
    state.player.political_capital += change;

    return {
      state,
      pcChanges
    };
  },
  components: {
  },
  mounted() {
    /* let pc */
  },
  methods: {
    next() {
      state.phase = 'PLANNING';
    },
  }
}
</script>

<style>
.result {
  margin: 2em 0;
  text-align: center;
  color: red;
}
.good {
  color: #1bbf5a;
}
.bad {
  color: red;
}
.results {
  margin: 2em 0;
}
</style>
