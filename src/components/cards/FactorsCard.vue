<template>
<Card class="factors">
  <template v-slot:header>
  </template>
  <template v-slot:body>
    <div class="factors--users">
      <div class="factors--total">
        <div>Total:</div>
        <div>{{total}}<img :src="icons[icon]" /></div>
      </div>
      <div class="factors--user" v-for="user in relevantFactors" :class="{highlight: current && user.name == current.name}">
        <div>
          <div>{{user.name}}</div>
        </div>
        <div>
          <template v-if="user.type == 'Region'">
            <IntensityIcon
              resource="wealth" :intensity="user.intensity" />
            <div class="factors--usage">{{user.amount}}<img :src="icons[icon]"></div>
          </template>
          <template v-else-if="user.type !== 'Project' && user.type !== 'Event'">
            <IntensityIcon
              :resource="icon" :intensity="user.intensity" />
            <div class="factors--usage"><template v-if="user.displayProduced !== null">{{user.displayProduced}}<img :src="icons[user.output]"><span class="arrow">⟵</span></template>{{user.displayAmount}}<img :src="icons[icon]"></div>
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
.factors--users > div {
  margin: 0.5em 0;
  padding: 0 0.25em;
}
.factors--users img {
  height: 14px;
  width: 14px;
  vertical-align: middle;
}
.factors--user > div, .factors--total {
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

.factors--total {
  border: 1px solid #fff;
  position: sticky;
  top: 0;
  background: #222222;
  border-radius: 0.2em;
  padding: 0.1em 0.25em !important;
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
