<template>
  <h2>EVENT STREAM</h2>

  <div class="stats">
    <div>Year: {{state.player.year}}</div>
    <div>Political Capital: {{state.player.political_capital}}</div>
    <div id="show-active-modifiers" @click="toggleActiveModifiers">
      <span v-if="showActiveModifiers">Hide</span><span v-else>Show</span> Active Modifiers
    </div>
  </div>

  <ul class="bar">
    <li v-for="(d, vari) in state.world">
      <b>{{VARI_ICONS[vari]}}{{VARI_ABBREV[vari]}}</b>:
      <span v-if="vari in state.plan.targets" :class="{achieved: d.value * state.plan.targets[vari].valence >= state.plan.targets[vari].value * state.plan.targets[vari].valence}">{{d.value}}/{{state.plan.targets[vari].value}}</span>
      <span v-else>{{d.value}}</span>
      <span class="estimate"><span class="icon">⏳</span>{{d.change >= 0 ? '+' : '-'}}{{Math.abs(d.change)}}</span>
    </li>
  </ul>

  <div id="active-modifiers" v-if="showActiveModifiers">
    <em class="note">List active modifiers, projects, etc. Should show project construction progress, if construction is stalled, etc.</em>
  </div>
  <div v-else>
    <div id="event-wrapper">
      <button @click="prevEvent">&lt;</button>
      <div id="event">
        <div v-if="activeEvent >= 0">
          <div id="event-meta">
            <div>{{state.events[activeEvent].type}}</div>
            <div>{{activeEvent+1}}/{{state.events.length}}</div>
          </div>
          <p>{{state.events[activeEvent].body}}</p>
        </div>
        <div v-else>
          <p>Emergency Measures</p>
          <em class="note">Make changes to the plan; these cost PC to use</em>
        </div>
      </div>
      <img src="assets/mock-earth.png" id="mock-earth" />
      <button @click="nextEvent">&gt;</button>
    </div>

    <div id="hand" class="cards" v-if="activeEvent >= 0">
      <Response v-for="r in state.events[activeEvent].responses" :response="r" @click="() => toggleResponse(r)" :class="{selected: state.events[activeEvent].selectedResponse == r, cantPlay: !canPlay(r)}"></Response>
    </div>
    <div id="hand" class="cards" v-else>
      <em class="note">Halt existing projects, change energy mix, etc?</em>
    </div>

    <div class="resources">
      <b>Resources:</b>
      <span class="resource" v-for="(d, vari) in state.player.resources">
        <b>{{vari}}</b>:{{d.value}}<span class="estimate"><span class="icon">⏳</span>{{d.change >= 0 ? '+' : '-'}}{{Math.abs(d.change)}}</span>
      </span>
    </div>

    <div class="actions">
      <button v-if="state.events.every((ev) => ev.selectedResponse !== null)" @click="nextTurn">Next Year</button>
    </div>
  </div>
</template>

<script>
import state from '../state';
import Card from './Card.vue';
import Response from './Response.vue';
export default {
  data() {
    this.updateEstimates();

    return {
      showActiveModifiers: false,
      activeEvent: 0,
      state
    };
  },
  components: {
    Card,
    Response,
  },
  methods: {
    nextTurn() {
      state.player.year++;

      // Reset events
      // TODO this is just for testing...we should actually generate new events
      state.events.forEach((ev) => {
        ev.selectedResponse = null;
      });

      // Update resources and indicators
      Object.keys(state.world).forEach((k) => {
        state.world[k].value += state.world[k].baseChange;
      });
      Object.keys(state.player.resources).forEach((k) => {
        state.player.resources[k].value += state.player.resources[k].baseChange;
      });

      this.updateEstimates();

      // Lose state
      if (state.player.political_capital <= 0) {
        alert('You\'ve lost your planning mandate! You lose');
      }

      if (state.player.year % 5 == 0) {
        state.phase = 'REPORT';
      }
    },
    nextEvent() {
      this.activeEvent++;
      if (this.activeEvent > this.state.events.length - 1) {
        this.activeEvent = -1;
      }
    },
    prevEvent() {
      this.activeEvent--;
      if (this.activeEvent < -1) {
        this.activeEvent = this.state.events.length - 1;
      }
    },
    toggleResponse(response) {
      let ev = this.state.events[this.activeEvent];
      if (ev.selectedResponse == response) {
        ev.selectedResponse = null;
      } else {
        ev.selectedResponse = response;
      }
      this.updateEstimates();
    },
    toggleActiveModifiers() {
      this.showActiveModifiers = !this.showActiveModifiers;
    },
    canPlay(response) {
      return Object.keys(response.costs).every((k) => {
        return state.player.resources[k].value + state.player.resources[k].change - response.costs[k] >= 0;
      });
    },
    updateEstimates() {
      Object.keys(state.world).forEach((k) => {
        state.world[k].change = state.world[k].baseChange;
      });
      Object.keys(state.player.resources).forEach((k) => {
        state.player.resources[k].change = state.player.resources[k].baseChange;
      });

      // Event effects
      state.events.forEach((ev) => {
        Object.keys(ev.impacts).forEach((k) => {
          state.world[k].change += ev.impacts[k];
        });
      });

      // Event response effects
      state.events.filter((ev) => ev.selectedResponse !== null).forEach((ev) => {
        Object.keys(ev.selectedResponse.costs).forEach((k) => {
          state.player.resources[k].change -= ev.selectedResponse.costs[k];
        });
        Object.keys(ev.selectedResponse.impacts).forEach((k) => {
          state.world[k].change += ev.selectedResponse.impacts[k];
        });
      });

      state.player.projects.forEach((p) => {
        let resources = {};
        let impacts = {};
        if (p.status == this.PROJECT_STATE.CONSTRUCTING) {
          resources = p.base.construction.resources;
          impacts = p.base.construction.impacts;
        } else if (p.status == this.PROJECT_STATE.DESTRUCTING) {
          resources = p.base.destruction.resources;
          impacts = p.base.destruction.impacts;
        } else if (p.status == this.PROJECT_STATE.OPERATIONAL) {
          resources = p.base.operation.resources;
          impacts = p.base.operation.impacts;
        };

        Object.keys(resources).forEach((k) => {
          state.player.resources[k].change += resources[k];
        });
        Object.keys(impacts).forEach((k) => {
          state.world[k].change += impacts[k];
        });
        console.log(impacts);
      });
    }
  }
}
</script>

<style>
#event {
  text-align: center;
  border: 1px solid #888;
  padding: 0.25em 0.5em 1em;
  flex: 1;
  margin: 0 1em;
}
#event-meta {
  display: flex;
  justify-content: space-between;
  font-size: 0.75em;
}
#event-wrapper {
  margin: 1em 0;
  display: flex;
  justify-content: space-between;
}
#hand .selected {
  border: 1px solid #000;
}
#hand .cantPlay {
  opacity: 0.25;
  pointer-events: none;
}

#active-modifiers {
  text-align: center;
  margin: 2em 0;
}
#show-active-modifiers {
  cursor: pointer;
}
#show-active-modifiers:hover {
  font-weight: bold;
}

#mock-earth {
	max-width: 80px;
	margin-right: 1em;
}

.note {
  color: #bbb;
}
</style>
