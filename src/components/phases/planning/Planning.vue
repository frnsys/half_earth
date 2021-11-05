<template>
<Hud />
<Dialogue v-if="event && event.dialogue" :dialogue="event.dialogue" :effects="event.effects" @done="nextEvent" @select="selectChoice" />
<div class="planning">
  <Research v-if="page == PAGES.RESEARCH" @close="page = null" />
  <Policies v-else-if="page == PAGES.POLICIES" @close="page = null" />
  <Initiatives v-else-if="page == PAGES.INITIATIVES" @close="page = null" />
  <Processes v-else-if="page == PAGES.PROCESSES" @close="page = null" />
  <Coalition v-else-if="page == PAGES.COALITION" @close="page = null" />
  <Dashboard v-else-if="page == PAGES.DASHBOARD" @close="page = null" />
  <div v-else class="planning--menu">
    <button v-for="p in Object.keys(PAGES)" @click="select(p)">
      <img :src="ICONS[p]" />
      {{p}}
    </button>
  </div>
  <div class="production--demand planning--demand">
    <div v-for="v, k in demand">
      {{demand[k]}}{{consts.icons[k]}}
    </div>
    <div>{{emissions}}{{consts.icons['emissions']}}</div>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import display from 'lib/display';
import Research from './Research.vue';
import Policies from './Policies.vue';
import Processes from './Processes.vue';
import Initiatives from './Initiatives.vue';
import Coalition from './Coalition.vue';
import Dashboard from './Dashboard.vue';
import EventsMixin from 'components/EventsMixin';
import Hud from 'components/Hud.vue';
import EVENTS from '/assets/content/events.json';

const PAGES = {
  RESEARCH: 0,
  INITIATIVES: 1,
  POLICIES: 2,
  PROCESSES: 3,
  COALITION: 4,
  DASHBOARD: 5,
  CONTINUE: 6
};

const ICONS = {
  'CONTINUE': '/assets/placeholders/earth_win98.png',
  'POLICIES': '/assets/placeholders/policy_win98.png',
  'RESEARCH': '/assets/placeholders/research_win98.png',
  'INITIATIVES': '/assets/placeholders/initiatives_win98.png',
  'PROCESSES': '/assets/placeholders/processes_win98.png',
  'COALITION': '/assets/placeholders/coalition_win98.png',
  'DASHBOARD': '/assets/placeholders/chart.png',
};

export default {
  mixins: [EventsMixin],
  components: {
    Hud,
    Research,
    Policies,
    Initiatives,
    Processes,
    Coalition,
    Dashboard
  },
  created() {
    this.PAGES = PAGES;
    this.ICONS = ICONS;
  },
  mounted() {
    this.showEvent();
  },
  activated() {
    this.showEvent();
  },
  data() {
    let events = game.roll.planningEvents();

    // Group events by pages
    let eventsByPage = Object.keys(PAGES).reduce((acc, k) => {
      acc[k] = [];
      return acc;
    }, {});
    eventsByPage[null] = [];
    events.forEach(([ev_id, region_id]) => {
      let ev = EVENTS[ev_id];
      let page = null;
      let parts = ev.name.split(':');
      if (parts.length > 1) {
        page = parts.shift();
      }
      eventsByPage[page].push([ev_id, region_id]);
    });

    return {
      state,
      events: eventsByPage[null],
      eventsByPage,
      page: null,
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
    select(p) {
      if (PAGES[p] == PAGES.CONTINUE) {
        state.phase = 'EVENTS';
      } else {
        this.page = PAGES[p];
        this.events = this.eventsByPage[p];
        this.showEvent();
      }
    },
  }
}
</script>

<style>
.planning {
  background: #ffecc7;
  min-height: 100vh;
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
  margin: 0 14% 1em;
  padding: 0.25em 0.5em;
  border-width: 4px;
  justify-self: center;
}
.planning--menu img {
  max-width: 100%;
}

.pip {
  width: 22px;
  vertical-align: middle;
}
.pips {
  padding: 0.5em;
  margin: 1em;
  border: 1px solid #454340;
  position: relative;
  text-align: center;
}
.pips--buy {
  cursor: pointer;
  user-select: none;
}
.pips--buy:hover {
  background: #eae7e7;
}
.pip-in-use {
  opacity: 0.5;
}

.planning--page {
  display: flex;
  flex-direction: column;
  flex: 1;
}
.planning--page .cards {
  flex: 1;
}
.planning--page .card header img {
  width: 12px;
  vertical-align: middle;
  margin-top: -2px;
}

.planning--page > header {
  padding: 0.5em;
  display: flex;
  justify-content: space-between;
}
.planning--page .back {
  width: 32px;
  cursor: pointer;
}

.planning .effects img {
  width: 16px;
  height: 16px;
  vertical-align: middle;
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
  position: absolute;
  bottom: 2em;
  left: 0;
  right: 0;
}
</style>
