<template>
<Hud />
<Dialogue v-if="hasDialogue" v-bind="event" @done="nextEvent" />
<div class="planning">
  <header>
    <div :class="{active: page == PAGES.PLAN, highlight: planHighlighted}" @click="selectPage(PAGES.PLAN)">Plan</div>
    <div :class="{active: page == PAGES.PARLIAMENT, disabled: parliamentDisabled, highlight: parliamentHighlighted}" @click="selectPage(PAGES.PARLIAMENT)">Govt</div>
    <div :class="{active: page == PAGES.DASHBOARD, disabled: dashboardDisabled, highlight: dashboardHighlighted}" @click="selectPage(PAGES.DASHBOARD)"><img class="changes-icon" v-if="hasChanges" :src="icons.hourglass" />Stats</div>
    <div :class="{active: page == PAGES.REGIONS, disabled: regionsDisabled, highlight: regionsHighlighted}" @click="selectPage(PAGES.REGIONS)">World</div>
  </header>

  <Plan v-if="page == PAGES.PLAN" @page="pageEvents" @change="planChangeEvents" />
  <Parliament v-else-if="page == PAGES.PARLIAMENT" />
  <Dashboard v-else-if="page == PAGES.DASHBOARD" />
  <Regions v-else-if="page == PAGES.REGIONS" />
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import Hud from 'components/Hud.vue';
import Parliament from './tabs/Parliament.vue';
import Dashboard from './tabs/Dashboard.vue';
import Plan from './tabs/Plan.vue';
import Regions from './tabs/Regions.vue';
import EventsMixin from 'components/EventsMixin';
import tutorial from '/src/tutorial';

const PAGES = {
  PLAN: 'Plan',
  PARLIAMENT: 'Parliament',
  DASHBOARD: 'Dashboard',
  REGIONS: 'Regions',
};

export default {
  mixins: [EventsMixin],
  components: {
    Hud,
    Parliament,
    Dashboard,
    Regions,
    Plan,
  },
  created() {
    this.PAGES = PAGES;
  },
  mounted() {
    game.updateFactors();
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
    hasChanges() {
      let totalChanges = 0;
      Object.values(state.processMixChanges).forEach((output) => {
        totalChanges += Object.values(output).reduce((acc, change) => {
          return acc + Math.abs(change);
        }, 0);
      });
      return totalChanges !== 0;
    },
    parliamentDisabled() {
      return state.tutorial < tutorial.PARLIAMENT;
    },
    parliamentHighlighted() {
      return state.tutorial == tutorial.PARLIAMENT;
    },
    dashboardDisabled() {
      return state.tutorial < tutorial.DASHBOARD;
    },
    dashboardHighlighted() {
      return state.tutorial == tutorial.DASHBOARD;
    },
    regionsDisabled() {
      return state.tutorial < tutorial.REGIONS;
    },
    regionsHighlighted() {
      return state.tutorial == tutorial.REGIONS;
    },
    planHighlighted() {
      return state.tutorial == tutorial.PLAN;
    },
  },
  methods: {
    selectPage(p) {
      if (p == PAGES.PARLIAMENT && state.tutorial == tutorial.PARLIAMENT) {
        state.tutorial++;
      } else if (p == PAGES.DASHBOARD && state.tutorial == tutorial.DASHBOARD) {
        state.tutorial++;
      } else if (p == PAGES.REGIONS && state.tutorial == tutorial.REGIONS) {
        state.tutorial++;
      } else if (p == PAGES.PLAN && state.tutorial == tutorial.PLAN) {
        state.tutorial++;
      }
      this.page = p;
      this.events = game.roll.planning(this.page);
      this.showEvent();
    },
    pageEvents(p) {
      this.events = game.roll.planning(p);
      this.showEvent();
    },
    planChangeEvents() {
      this.events = game.roll.planning('PlanChange');
      this.showEvent();
    }
  }
}
</script>

<style>
.planning {
  background: #ffecc7;
  height: calc(100% - 16px);
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
  image-rendering: auto;
}
.pips {
  padding: 0.5em;
  margin: 0 auto 0.5em;
  position: relative;
  text-align: center;
  font-size: 1.2em;
  color: #fff;
  user-select: none;
  border-radius: 0.2em;
  background: rgba(0,0,0,0.1);
}
.pip-in-use {
  opacity: 0.5;
}

.planning--page {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow-y: scroll;
  padding: 4em 0.5em 1em 0.5em;

  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none;  /* Internet Explorer 10+ */
}
.planning--page::-webkit-scrollbar { /* WebKit */
  width: 0;
  height: 0;
}
.planning--page .cards {
  flex: 1;
  margin-top: 24px;
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
  position: absolute;
  margin: 0 auto;
  left: 0.5em;
  right: 0.5em;
  z-index: 2;
  border-radius: 0 0 0.3em 0.3em;
  background: #fff;
  box-shadow: 0 1px 2px rgba(0,0,0,0.5);
  max-width: 360px;
}
.planning > header div.disabled {
  pointer-events: none;
  opacity: 0.5;
}
.planning > header div.highlight {
  animation-duration: 0.75s;
  animation-name: highlight;
  animation-iteration-count: infinite;
  animation-direction: alternate;
}
.planning > header div {
  flex: 1;
  text-align: center;
  padding: 0.5em 0.25em;
  border-right: 1px solid #aaa;
}
.planning > header div:first-child {
  border-radius: 0 0 0 0.3em;
}
.planning > header div:last-child {
  border-radius: 0 0 0.3em 0;
}
.planning > header div:hover {
  background: #d4e4a7;
}
.planning > header div.active {
  background: var(--colour-pink);
  color: #000;
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

.changes-icon {
  width: 8px;
  margin-right: 1px;
}
</style>
