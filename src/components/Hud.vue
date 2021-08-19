<template>
  <div class="hud">
    <div>{{state.player.year}}</div>

    <ul class="hud--indicators">
      <li v-for="(d, vari) in estimates">
        <b>{{VARI_ICONS[vari]}}</b>
        <span v-if="vari in state.plan.targets" :class="{achieved: d.value * state.plan.targets[vari].valence >= state.plan.targets[vari].value * state.plan.targets[vari].valence}">{{d.value}}/{{state.plan.targets[vari].value}}</span>
        <span v-else>{{d.value}}</span>
        <span class="estimate">{{d.change >= 0 ? '+' : '-'}}{{Math.abs(d.change)}}</span>
      </li>
    </ul>

    <div>{{state.player.political_capital}}🗳️</div>
  </div>
</template>

<script>
import state from '../state';
export default {
  data() {
    return {
      state,
    };
  },
  computed: {
    estimates() {
      const estimates = {};

      Object.keys(state.world).forEach((k) => {
        estimates[k] = {
          change: state.world[k].baseChange,
          value: state.world[k].value
        };
      });

      // Event effects
      state.events.forEach((ev) => {
        Object.keys(ev.impacts).forEach((k) => {
          estimates[k].change += ev.impacts[k];
        });
      });

      // TODO other factors

      return estimates;
    }
  }
};
</script>

<style>
.hud {
  display: flex;
  background: #202020;
  color: #fff;
  justify-content: space-between;
  padding: 0 0.5em;
  font-size: 0.75em;
}

.hud--indicators li {
  display: inline-block;
  margin: 0 0.25em;
}

.estimate {
	color: #888;
	padding: 0 0.1em;
	margin-left: 0.2em;
}
</style>
