<template>
  <h2>IMPLEMENTATION</h2>

  <div class="stats">
    <div>Year: {{state.player.year}}</div>
    <div>Political Capital: {{state.player.political_capital}}</div>
  </div>

  <ul class="bar">
    <li v-for="(d, vari) in state.world">
      <b>{{VARI_ICONS[vari]}}{{VARI_ABBREV[vari]}}</b>:
      <span v-if="vari in state.plan.targets" :class="{achieved: d.value * state.plan.targets[vari].valence >= state.plan.targets[vari].value * state.plan.targets[vari].valence}">{{d.value}}/{{state.plan.targets[vari].value}}</span>
      <span v-else>{{d.value}}</span>
      <span class="estimate"><span class="icon">⏳</span>{{d.change >= 0 ? '+' : '-'}}{{Math.abs(d.change)}}</span>
    </li>
  </ul>

  <!-- TODO EVENTS -->
  <h3>Region</h3>
  <div class="region">
    <Plot v-for="plot in state.region" @click="() => togglePlot(plot)" :class="{'selected-plot': selectedPlot === plot}" :plot="plot">
      <template v-slot:actions v-if="plot.project">
        <button v-if="plot.project.status == PROJECT_STATE.OPERATIONAL" @click="() => revokeCard(plot.project)">Revoke</button>
        <button v-else-if="plot.project.status == PROJECT_STATE.PLANNED" @click="() => cancelCard(plot.project)">Undo</button>
        <button v-else-if="plot.project.status == PROJECT_STATE.CONSTRUCTING" @click="() => cancelCard(plot.project)">Cancel</button>
      </template>
    </Plot>
  </div>

  <h3>Active Policies</h3>
  <div v-if="state.player.projects.length === 0">No active policies</div>
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
        <button @click="() => playCard(p)" :disabled="!(p.global || (selectedPlot !== null && selectedPlot.project === null))">Play</button>
      </template>
    </Project>
    <Card class="card--research" v-for="r in state.player.research">
      <div>Research</div>
      {{r.name}}
    </Card>
  </div>

  <div class="resources">
    <b>Resources:</b>
      <span class="resource" v-for="(d, vari) in state.player.resources">
        <b>{{vari}}</b>:{{d.value}}<span class="estimate"><span class="icon">⏳</span>{{d.change >= 0 ? '+' : '-'}}{{Math.abs(d.change)}}</span>
      </span>
    </div>

  <div class="actions">
    <button @click="nextTurn">Next Year</button>
  </div>

  <div id="help">
    <div><span class="estimate"><span class="icon">⏳</span>+X</span> : <em>estimate for variable change in next turn</em></div>
    <p><em>Misc Notes</em></p>
    <ul>
      <li>- Adjacency effects</li>
      <li>- Convert tile types (e.g. convert water to land by landfilling)</li>
      <li>- Things like BECCS should have outputs based on the plot fertility</li>
      <li>- Global mandates should have worldwide effects, like energy quotas lowers the energy/output of everything</li>
    </ul>
  </div>
</template>

<script>
import state from '../state';
import Card from './Card.vue';
import Plot from './Plot.vue';
import Project from './Project.vue';
import ActiveProject from './ActiveProject.vue';

export default {
  data() {
    this.updateEstimates();
    return {
      selectedPlot: null,
      state,
    };
  },
  components: {
    Card,
    Plot,
    Project,
    ActiveProject
  },
  methods: {
    togglePlot(i) {
      this.selectedPlot = this.selectedPlot === i ? null : i;
    },
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
      state.player.projects.concat(state.region.filter((plot) => plot.project).map((plot) => plot.project)).forEach((p) => {
        if (p.status == this.PROJECT_STATE.PLANNED) {
          p.status = this.PROJECT_STATE.CONSTRUCTING;
        }

        let requiredResources = {};
        let impacts = {};
        if (p.status == this.PROJECT_STATE.CONSTRUCTING) {
          requiredResources = p.base.construction.resources;
          impacts = p.base.construction.impacts;
        } else if (p.status == this.PROJECT_STATE.DESTRUCTING) {
          requiredResources = p.base.destruction.resources;
          impacts = p.base.destruction.impacts;
        } else if (p.status == this.PROJECT_STATE.OPERATIONAL) {
          requiredResources = p.base.operation.resources;
          impacts = p.base.operation.impacts;
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

          // Apply impacts
          Object.keys(impacts).forEach((k) => {
            state.world[k].value += impacts[k];
          });

          if (p.yearsLeft === 0) {
            if (p.status == this.PROJECT_STATE.CONSTRUCTING) {
              p.status = this.PROJECT_STATE.OPERATIONAL;
            } else if (p.status == this.PROJECT_STATE.DESTRUCTING) {
              // Remove card
              state.player.projects = state.player.projects.filter((p_) => p_ !== p);
            }
          }
        }
      });

      // Plot updates
      state.region.forEach((plot) => {
        if (plot.toxic && plot.fertility > 0 && Math.random() < 0.25) {
          plot.fertility--;
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

      if (proj.global) {
        // Add to active policies
        state.player.projects.push({
          status: this.PROJECT_STATE.PLANNED,
          yearsLeft: proj.construction.years,
          base: proj
        });
      } else if (this.selectedPlot) {
        // Add to selected plot
        this.selectedPlot.project = {
          status: this.PROJECT_STATE.PLANNED,
          yearsLeft: proj.construction.years,
          plot: this.selectedPlot,
          base: proj
        };
        this.selectedPlot = null;
      }

      this.updateEstimates();
    },
    revokeCard(proj) {
      proj.status = this.PROJECT_STATE.DESTRUCTING;
      proj.yearsLeft = proj.base.destruction.years;
      if (proj.plot) {
        proj.plot.project = null;
      }
      this.updateEstimates();
    },
    cancelCard(proj) {
      if (proj.status == this.PROJECT_STATE.PLANNED) {
        // Put back into hand
        state.player.hand.push(proj.base);

        // Remove card
        if (proj.plot) {
          proj.plot.project = null;
        }
      } else if (proj.status == this.PROJECT_STATE.CONSTRUCTING) {
        // Remove card
        if (proj.plot) {
          proj.plot.project = null;
          if (proj.base.toxic) {
            proj.plot.toxic = true;
          }
        }
      }
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
          // Resources depicted as costs
          state.player.resources[k].change -= resources[k];
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
#help {
  color: #888;
  margin-top: 2em;
}
#help li {
  margin: 0;
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
  margin: 0 0.5em 1em 0.5em;
}
.active-projects .in-progress {
  opacity: 0.5;
  border: 1px dashed #000;
}

.resources {
  margin: 2em 0 0 0;
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

.region {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
}
.region .card {
  width: 24%;
  margin-bottom: 0.5em;
}
.region .selected-plot {
  background: #f0f0f0;
  border: 1px solid #000;
  box-shadow: 3px 3px 0 rgba(0,0,0,0.5);
}
</style>
