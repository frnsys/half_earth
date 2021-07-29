<template>
  <h2>IMPLEMENTATION</h2>

  <div class="stats">
    <div>Year: {{state.player.year}}</div>
    <div>Political Capital: {{state.player.political_capital}}</div>
  </div>

  <ul class="bar">
    <li v-for="(d, vari) in state.world">
      <b>{{vari}}</b>:
      <span v-if="vari in state.plan.targets" :class="{achieved: d.value * state.plan.targets[vari].valence >= state.plan.targets[vari].value * state.plan.targets[vari].valence}">{{d.value}}/{{state.plan.targets[vari].value}}</span>
      <span v-else>{{d.value}}</span>
      <span class="estimate"><span class="icon">⏳</span>{{d.change >= 0 ? '+' : '-'}}{{Math.abs(d.change)}}</span>
    </li>
  </ul>

  <!-- TODO EVENTS -->

  <h3>Active Projects</h3>
  <div v-if="state.player.projects.length === 0">No active projects</div>
  <div v-else class="active-projects cards">
    <ActiveProject :project="p" v-for="p in state.player.projects">
      <template v-slot:actions>
        <button @click="() => revokeCard(p)">Revoke</button>
      </template>
    </ActiveProject>
  </div>

  <h3>Hand</h3>
  <div class="hand cards">
    <Project :project="p" v-for="p in state.player.hand">
      <template v-slot:actions>
        <button @click="() => playCard(p)">Play</button>
      </template>
    </Project>
    <Card class="card--research" v-for="r in state.player.research">
      <div>Research</div>
      {{r.name}}
    </Card>
  </div>

  <b>Resources:</b>
    <span class="resource" v-for="(d, vari) in state.player.resources">
      <b>{{vari}}</b>:{{d.value}}<span class="estimate"><span class="icon">⏳</span>{{d.change >= 0 ? '+' : '-'}}{{Math.abs(d.change)}}</span>
    </span>

  <div class="actions">
    <button @click="nextTurn">Next Year</button>
  </div>

  <div id="help">
    <div><span class="estimate"><span class="icon">⏳</span>+X</span> : <em>estimate for variable change in next turn</em></div>
  </div>
</template>

<script>
import state from '../state';
import {PROJECT_STATE} from '../consts';
import Card from './Card.vue';
import Project from './Project.vue';
import ActiveProject from './ActiveProject.vue';


export default {
  data() {
    this.updateEstimates();
    return {
      state,
    };
  },
  components: {
    Card,
    Project,
    ActiveProject
  },
  methods: {
    nextTurn() {
      state.player.year++;

      // Update resources and indicators
      Object.keys(state.world).forEach((k) => {
        state.world[k].value += state.world[k].baseChange;
      });
      Object.keys(state.player.resources).forEach((k) => {
        state.player.resources[k].value += state.player.resources[k].baseChange;
      });

      // Update project progress
      state.player.projects.forEach((p) => {
        let requiredResources = {};
        if (p.status == PROJECT_STATE.CONSTRUCTING) {
          requiredResources = p.base.construction.resources;
        } else if (p.status == PROJECT_STATE.DESTRUCTING) {
          requiredResources = p.base.destruction.resources;
        } else if (p.status == PROJECT_STATE.OPERATIONAL) {
          requiredResources = p.base.operation.resources;
        }
        let enoughResources = Object.keys(requiredResources).every((k) => {
          return state.player.resources[k].value >= requiredResources[k];
        });

        if (enoughResources) {
          if (p.yearsLeft > 0) p.yearsLeft -= 1;

          // Deduct resources
          Object.keys(requiredResources).forEach((k) => {
            state.player.resources[k].value -= requiredResources[k];
          });

          if (p.yearsLeft === 0) {
            if (p.status == PROJECT_STATE.CONSTRUCTING) {
              p.status = PROJECT_STATE.OPERATIONAL;
            } else if (p.status == PROJECT_STATE.DESTRUCTING) {
              // Remove card
              state.player.projects = state.player.projects.filter((p_) => p_ !== p);
            }
          }
        }
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
    playCard(proj) {
      // Remove from hand
      state.player.hand = state.player.hand.filter((p) => p != proj);

      // Add to active
      state.player.projects.push({
        status: PROJECT_STATE.CONSTRUCTING,
        yearsLeft: proj.construction.years,
        base: proj
      });

      this.updateEstimates();
    },
    revokeCard(proj) {
      proj.status = PROJECT_STATE.DESTRUCTING;
      proj.yearsLeft = proj.base.destruction.years;
      this.updateEstimates();
    },
    updateEstimates() {
      Object.keys(state.world).forEach((k) => {
        state.world[k].change = state.world[k].baseChange;
      });
      Object.keys(state.player.resources).forEach((k) => {
        state.player.resources[k].change = state.player.resources[k].baseChange;
      });

      state.player.projects.forEach((p) => {
        let resources = {};
        if (p.status == PROJECT_STATE.CONSTRUCTING) {
          resources = p.base.construction.resources;
        } else if (p.status == PROJECT_STATE.DESTRUCTING) {
          resources = p.base.destruction.resources;
        } else if (p.status == PROJECT_STATE.OPERATIONAL) {
          resources = p.base.operation.resources;
        };

        Object.keys(resources).forEach((k) => {
          if (k in state.world) {
            state.world[k].change += resources[k];
          } else {
            // Resources depicted as costs
            state.player.resources[k].change -= resources[k];
          }
        });
      });
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
  padding: 0.5em 0;
  justify-content: space-around;
}
.bar li {
  margin: 0 1em 0 0;
}

.hand .card,
.active-projects .card {
  border: 1px solid #000;
}
.active-projects .in-progress {
  opacity: 0.5;
  border: 1px dashed #000;
}
.resource {
  margin-left: 1em;
}

.hand .card--research {
  opacity: 0.5;
  border: 1px dashed #000;
  pointer-events: none;
}

.estimate {
	color: #888;
	background: #f0f0f0;
	border: 1px solid #aaa;
	padding: 0 0.1em;
	margin-left: 0.2em;
}

.achieved {
  color: #1bbf5a;
  border-bottom: 2px solid #1bbf5a;
}
</style>
