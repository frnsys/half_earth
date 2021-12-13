<template>
<Card class="resource">
  <template v-slot:header>
  </template>
  <template v-slot:body>
    <div class="resource--users">
      <div class="resource--user" v-for="user in relevantRankings" :class="{highlight: current && user.name == current.name}">
        <div>
          <div>{{user.name}}</div>
        </div>
        <div>
          <template v-if="user.type !== 'Project' && user.type !== 'Event'">
            <IntensityIcon
              :resource="icon" :intensity="user.intensity" />
            <div class="resource--usage">{{user.displayAmount}}<img :src="icons[icon]"><template v-if="user.displayProduced !== null"><span class="arrow">⟶</span>{{user.displayProduced}}<img :src="icons[user.output]"></template></div>
          </template>
          <template v-else>
            <div class="resource--usage resource--usage-solo">{{user.amount}}<img :src="icons[icon]"></div>
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

<div class="resource-note">
*Excluding impacts from energy use
</div>
</template>

<script>
import state from '/src/state';
import display from 'lib/display';
import Card from './Card.vue';
import IntensityIcon from './IntensityIcon.vue';

export default {
  props: ['resource'],
  components: {
    Card,
    IntensityIcon,
  },
  data() {
    return {
      ...this.resource,
    };
  },
  computed: {
    relevantRankings() {
      return state.resourceRankings[this.type].filter((user) => user.displayProduced !== 0);
    },
  }
}
</script>

<style>
.resource--user {
  margin: 0.5em 0;
  padding: 0 0.25em;
}
.resource--user > div {
  display: flex;
  justify-content: space-between;
}
.resource--user.highlight {
  background: #f5f9c7;
  color: #111;
  border-radius: 0.2em;
}
.resource--user .card-icon {
  display: flex;
}
.resource--user img {
  height: 14px;
  width: 14px;
  vertical-align: middle;
}
.resource--user .intensity-pips {
  display: flex;
  align-items: center;
  margin-top: 0;
  margin-left: 2px;
  height: 14px;
}
.resource--usage {
  font-size: 14px;
}

.arrow {
  color: #727987;
}

.card-tabs {
  display: flex;
  justify-content: space-between;
  width: 100%;
}
.card-tabs .selected {
  text-decoration: underline;
}
.card-tabs > div:hover {
  cursor: pointer;
  text-decoration: underline;
}

.card.resource {
  overflow-y: scroll;
  scrollbar-width: thin;
}
.card.resource footer img {
  display: none;
}
.card.resource header {
  position: sticky;
  top: 0;
  background: #333;
}
.card.resource .card--body {
  justify-content: space-between;
}

.resource-note {
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
.resource--usage-solo {
  width: 100%;
  text-align: right;
}
</style>
