<template>
<Hud />
<Interstitial v-if="event" :dialogue="event.dialogue" @done="nextEvent" @select="selectChoice" />
<div class="planning">
  <Research v-if="page == PAGES.RESEARCH" @close="page = null" />
  <Policies v-else-if="page == PAGES.POLICIES" @close="page = null" />
  <Initiatives v-else-if="page == PAGES.INITIATIVES" @close="page = null" />
  <Processes v-else-if="page == PAGES.PROCESSES" @close="page = null" />
  <div v-else class="planning--menu">
    <button v-for="p in Object.keys(PAGES)" @click="select(p)">
      <img v-if="p == 'CONTINUE'" src="/assets/placeholders/earth_win98.png" />
      <img v-else-if="p == 'POLICIES'" src="/assets/placeholders/policy_win98.png" />
      <img v-else-if="p == 'RESEARCH'" src="/assets/placeholders/research_win98.png" />
      <img v-else-if="p == 'INITIATIVES'" src="/assets/placeholders/initiatives_win98.png" />
      <img v-else-if="p == 'PROCESSES'" src="/assets/placeholders/processes_win98.png" />
      <img v-else src="/assets/placeholders/chart.png" />
      {{p}}
    </button>
  </div>
</div>
</template>

<script>
import game from '../../game';
import state from '../../state';
import Research from './Research.vue';
import Policies from './Policies.vue';
import Initiatives from './Initiatives.vue';
import Processes from './Processes.vue';
import Hud from '../Hud.vue';
import EventsMixin from '../EventsMixin';

const PAGES = {
  RESEARCH: 0,
  INITIATIVES: 1,
  POLICIES: 2,
  PROCESSES: 3,
  DASHBOARD: 4,
  CONTINUE: 5
}

export default {
  mixins: [EventsMixin],
  components: {
    Hud,
    Research,
    Policies,
    Initiatives,
    Processes
  },
  data() {
    let events = game.rollPlanningEvents();
    return {
      PAGES,
      page: null,
      events,
      eventIdx: events.length > 0 ? 0 : null
    }
  },
  methods: {
    select(p) {
      if (PAGES[p] == PAGES.CONTINUE) {
        state.phase = 'EVENTS';
      } else {
        this.page = PAGES[p];
      }
    }
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
  display: grid;
  grid-template-columns: 1fr 1fr;
}
.planning--menu button {
  width: 96px;
  height: 96px;
  padding: 0.25em 0.5em;
  border-width: 4px;
  justify-self: center;
  margin-bottom: 1em;
}
.planning--menu img {
  max-width: 100%;
}

.pip {
  width: 32px;
}
.pips {
  padding: 1em 0.5em 0.5em;
  margin: 1em;
  border: 1px solid #454340;
  width: calc(320px + 1em + 2px);
  position: relative;
}
.pips--label {
  position: absolute;
  top: 0;
  left: 50%;
  transform: translate(-50%, -50%);
  background: #FFECC7;
  border: 1px solid #454340;
  padding: 0 0.5em;
  font-size: 0.7em;
  text-transform: uppercase;
  text-align: center;
  width: 120px;
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
}

.planning--page > header {
  padding: 0.5em;
  display: flex;
}
.planning--page .back {
  width: 32px;
  cursor: pointer;
}
</style>
