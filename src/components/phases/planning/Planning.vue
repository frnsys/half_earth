<template>
<Hud />
<Dialogue v-if="hasDialogue" v-bind="event" @done="nextEvent" />
<div class="planning">
  <header>
    <div :class="{active: page == PAGES.PLAN}" @click="selectPage(PAGES.PLAN)">Plan</div>
    <div :class="{active: page == PAGES.COALITION}" @click="selectPage(PAGES.COALITION)">Coalition</div>
    <div :class="{active: page == PAGES.DASHBOARD}" @click="selectPage(PAGES.DASHBOARD)">Dashboard</div>
    <div :class="{active: page == PAGES.REGIONS}" @click="selectPage(PAGES.REGIONS)">Regions</div>
  </header>

  <Plan v-if="page == PAGES.PLAN" @page="pageEvents" />
  <Coalition v-else-if="page == PAGES.COALITION" />
  <Dashboard v-else-if="page == PAGES.DASHBOARD" />
  <Regions v-else-if="page == PAGES.REGIONS" />
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import display from 'lib/display';
import Hud from 'components/Hud.vue';
import Coalition from './Coalition.vue';
import Dashboard from './Dashboard.vue';
import Plan from './Plan.vue';
import Regions from './Regions.vue';
import EventsMixin from 'components/EventsMixin';
import EVENTS from '/assets/content/events.json';

const PAGES = {
  PLAN: 'Plan',
  COALITION: 'Coalition',
  DASHBOARD: 'Dashboard',
  REGIONS: 'Regions',
};

export default {
  mixins: [EventsMixin],
  components: {
    Hud,
    Coalition,
    Dashboard,
    Regions,
    Plan,
  },
  created() {
    this.PAGES = PAGES;
  },
  mounted() {
    game.updateResourceRankings();
    this.showEvent();
  },
  activated() {
    this.showEvent();
  },
  data() {
    let events = game.roll.planning('Start');
    return {
      state,
      events,
      page: PAGES.PLAN,
    }
  },
  computed: {
    demand() {
      return display.outputs(state.gameState.output_demand);
    },
    emissions() {
      return display.gtco2eq(state.gameState.byproducts);
    }
  },
  methods: {
    selectPage(p) {
      this.page = p;
      this.events = game.roll.planning(this.page);
      this.showEvent();
    },
    pageEvents(p) {
      this.events = game.roll.planning(p);
      this.showEvent();
    }
  }
}
</script>

<style>
.planning {
  background: #ffecc7;
  height: calc(100vh - 16px);
  display: flex;
  flex-direction: column;
}
.planning--menu {
  padding: 1em;
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}
.planning--menu button {
  width: 96px;
  height: 96px;
  margin: 0 10% 1em;
  padding: 0.25em 0.5em;
  border-width: 4px;
  justify-self: center;
}
.planning--menu img {
  max-width: 36px;
}

.pip {
  width: 22px;
  vertical-align: middle;
}
.pips {
  padding: 0.5em;
  margin: 0.25em;
  position: relative;
  text-align: center;
  font-size: 1.2em;
  color: #fff;
}
.pips--buy {
  cursor: pointer;
  user-select: none;
  border-radius: 0.2em;
  background: rgba(0,0,0,0.1);
}
.pips--buy:hover {
  background: rgba(255,255,255,0.3);
}
.pip-in-use {
  opacity: 0.5;
}

.planning--page {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow-y: scroll;

  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none;  /* Internet Explorer 10+ */
}
.planning--page::-webkit-scrollbar { /* WebKit */
  width: 0;
  height: 0;
}
.planning--page .cards {
  flex: 1;
  margin-top: -1em;
}

.project--upgrade--title {
  display: flex;
  font-size: 0.75em;
  justify-content: space-between;
  border-bottom: 1px dashed;
  padding: 0 0 0.25em 0;
  margin-bottom: 0.5em;
}
.project--upgrade--title button {
  padding: 0 0.5em;
}
.project--upgrade .effects {
    font-size: 0.8em;
    padding: 0.1em 0.3em;
    border: none;
    background: rgba(0,0,0,0.1);
}
.project--upgrade img,
.project--upgrade .effects img {
  width: 16px;
  height: 16px;
  vertical-align: middle;
}

.planning--demand {
  display: flex;
  justify-content: space-evenly;
  padding: 1em;
  font-size: 1.1em;
}
.planning--demand img {
  width: 22px;
  vertical-align: middle;
}

.planning > header {
  display: flex;
  border-bottom: 1px solid #000;
}
.planning > header div {
  flex: 1;
  text-align: center;
  padding: 0.25em;
  border-right: 1px solid #000;
}
.planning > header div:hover {
  background: #e3b6a0;
}
.planning > header div.active {
  background: #e47d4a;
  color: #fff;
}
.planning > header div:last-child {
  border-right: none;
}

.planning--page > footer {
  display: flex;
  justify-content: space-between;
}

.minicard-grid {
  flex-wrap: wrap;
  display: flex;
  justify-content: space-evenly;
}
.minicard-grid-item {
  width: 80px;
  margin: 1em 0.5em;
}
.minicard-grid-item-label {
  text-align: center;
  font-size: 0.8em;
}
</style>
