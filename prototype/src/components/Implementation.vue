<template>
  <h2>IMPLEMENTATION</h2>

  <div>Year: {{state.player.year}}</div>
  <div>Political Capital: {{state.player.political_capital}}</div>

  <ul class="bar">
    <li v-for="(d, vari) in state.world">
      <b>{{vari}}</b>:{{d.value}}<span v-if="vari in state.plan.targets">/{{state.plan.targets[vari].value}}</span>
      <span style="color:#888;">⏳{{d.change >= 0 ? '+' : '-'}}{{Math.abs(d.change)}}</span>
    </li>
  </ul>

  <!-- TODO EVENTS -->

  <h3>Active Projects</h3>
  <ul>
    <li v-for="p in state.player.projects">
      <Card>
        <b>{{p.project.name}}</b>
        <div>
          <div v-if="p.yearsLeft > 0">construction years left:{{p.yearsLeft}}</div>
          <div v-else-if="p.yearsLeft == 0">completed</div>
        </div>

        <div>
          <b>Destruction:</b>
          ⏳:{{p.project.destruction.years}}
          <span v-for="(v, k) in p.project.destruction.resources">
            <b>{{k}}</b>:{{v}}
          </span>
        </div>
        <div>
          <b>Operation:</b>
          <span v-for="(v, k) in p.project.operation.resources">
            <b>{{k}}</b>:{{v}}/⏳
          </span>
        </div>

        <button @click="() => revokeCard(p)">Revoke</button>
      </Card>
    </li>
  </ul>

  <h3>Hand</h3>
  <ul>
    <li v-for="p in state.player.hand">
      <Card>
        <b>{{p.name}}</b>
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
        <button @click="() => playCard(p)">Play</button>
      </Card>
    </li>
  </ul>

  <h3>Resources</h3>
  <ul class="bar">
    <li v-for="(d, vari) in state.player.resources">
      <b>{{vari}}</b>:{{d.value}}<span style="color:#888;">⏳{{d.change >= 0 ? '+' : '-'}}{{Math.abs(d.change)}}</span>
    </li>
  </ul>

  <button @click="nextTurn">Next Turn</button>

  <div id="help">
    <div>⏳+X : <em>estimate for variable change in next turn</em></div>
  </div>
</template>

<script>
import state from '../state';
import Card from './Card.vue';
export default {
  data() {
    return {
      state,
    };
  },
  components: {
    Card
  },
  methods: {
    nextTurn() {
      state.player.year++;

      // Update resources and indicators
      Object.keys(state.world).forEach((k) => {
        state.world[k].value += state.world[k].change;
      });
      Object.keys(state.player.resources).forEach((k) => {
        state.player.resources[k].value += state.player.resources[k].change;
      });

      // Update project progress
      state.player.projects.forEach((p) => {
        p.yearsLeft = Math.max(0, p.yearsLeft - 1);
      });

      if (state.player.year % 5 == 0) {
        state.phase = 'PLANNING'; // TODO should be report
      }
    },
    playCard(proj) {
      // Deduct construction resources
      Object.keys(proj.construction.resources).forEach((k) => {
        state.player.resources[k].value -= proj.construction.resources[k];
      });

      // Remove from hand
      state.player.hand = state.player.hand.filter((p) => p != proj);

      // Add to active
      state.player.projects.push({
        yearsLeft: proj.construction.years,
        project: proj
      });
    },
    revokeCard(proj) {
      // TODO
    }
  }
}
</script>

<style>
#help {
  color: #888;
  margin-top: 2em;
}
.bar {
  display: flex;
}
.bar li {
  margin-right: 1em;
}
</style>
