<template>
<Card class="factors">
  <template v-slot:header>
  </template>
  <template v-slot:body>
    <div class="factors--users">
      <div class="factors--user" v-for="user in relevantFactors" :class="{highlight: current && user.name == current.name}">
        <div>
          <div>{{user.name}}</div>
        </div>
        <div>
          <template v-if="user.type !== 'Project' && user.type !== 'Event'">
            <IntensityIcon
              :resource="icon" :intensity="user.intensity" />
            <div class="factors--usage">{{user.displayAmount}}<img :src="icons[icon]"><template v-if="user.displayProduced !== null"><span class="arrow">⟶</span>{{user.displayProduced}}<img :src="icons[user.output]"></template></div>
          </template>
          <template v-else>
            <div class="factors--usage factors--usage-solo">{{user.amount}}<img :src="icons[icon]"></div>
          </template>
        </div>
      </div>
    </div>
  </template>
  <template v-slot:back>
  </template>
  <template v-slot:footer>
  </template>
</Card>

<div class="factors-note">
Only direct impacts are shown.
</div>
</template>

<script>
import state from '/src/state';
import Card from './Card.vue';
import IntensityIcon from './IntensityIcon.vue';

export default {
  props: ['factors'],
  components: {
    Card,
    IntensityIcon,
  },
  data() {
    return {
      ...this.factors,
    };
  },
  computed: {
    relevantFactors() {
      return state.factors[this.type].filter((user) => user.displayProduced !== 0);
    },
  }
}
</script>

<style>
.factors--user {
  margin: 0.5em 0;
  padding: 0 0.25em;
}
.factors--user > div {
  display: flex;
  justify-content: space-between;
}
.factors--user.highlight {
  background: #f5f9c7;
  color: #111;
  border-radius: 0.2em;
}
.factors--user .card-icon {
  display: flex;
}
.factors--user img {
  height: 14px;
  width: 14px;
  vertical-align: middle;
}
.factors--user .intensity-pips {
  display: flex;
  align-items: center;
  margin-top: 0;
  margin-left: 2px;
  height: 14px;
}
.factors--usage {
  font-size: 14px;
}

.arrow {
  color: #727987;
}

.card.factors {
  overflow-y: scroll;
  scrollbar-width: thin;
}
.card.factors footer img {
  display: none;
}
.card.factors header {
  position: sticky;
  top: 0;
  background: #333;
}
.card.factors .card--body {
  justify-content: space-between;
}

.factors-note {
  position: absolute;
  bottom: 2em;
  left: 50%;
  transform: translate(-50%, 0);
  background: #222;
  color: #fff;
  padding: 0.5em 1em;
  text-align: center;
  border-radius: 0.2em;
  font-size: 0.8em;
}

/* hacky, but so exhausted at this point */
.factors--usage-solo {
  width: 100%;
  text-align: right;
}
</style>
