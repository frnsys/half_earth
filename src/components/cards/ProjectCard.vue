<template>
<Card>
  <template v-slot:header>
    <div>{{name}}</div>
    <div v-if="status == 'Finished' || status == 'Active'">
      <img :src="icons.check">
    </div>
  </template>
  <template v-slot:figure>
    <img class="card-image" :src="`/assets/content/images/${image.fname}`" />
    <div v-if="status !== 'Finished' && status !== 'Active'" class="card-tack-ur project-cost" v-tip="costTip">
      {{remainingCost}}<img :src="icons.political_capital" v-if="kind == 'Policy'">
    </div>
    <div v-if="status == 'Building'" class="card-tack-ul project-points">
      <img
        v-for="_ in points"
        class="pip"
        v-tip="{text: `${points} ${kind} points are allocated to this project`, icon: type}"
        :src="icons[type]">
    </div>
    <div v-if="hasLevels" class="project-level">
      Level {{level+1}}
    </div>

    <div class="opposers" v-if="opposersDetailed.length > 0">
      <div>Nay</div>
      <div>
        <img v-for="npc in opposersDetailed" v-tip="{text: `${npc.name} is opposed to this. If you implement it, your relationship will worsen by -<img src='${icons.relationship}' />.`, icon: npc.name}" :src="icons[npc.name]">
      </div>
    </div>
    <div class="supporters" v-if="supportersDetailed.length > 0">
      <div>Yea</div>
      <div>
        <img v-for="npc in supportersDetailed" v-tip="{text: `${npc.name} supports this. If you implement it, your relationship will improve by +<img src='${icons.relationship}' />.`, icon: npc.name}" :src="icons[npc.name]">
      </div>
    </div>
  </template>
  <template v-slot:body>
    <Effects :effects="activeEffects" />

    <div class="card-actions" v-if="status == 'Inactive' || status == 'Building'">
      <template v-if="kind == 'Policy'">
        <button @click="payPoints">Implement</button>
      </template>
      <template v-else>
        <button @click="assignPoint">+<img class="pip" :src="icons[type]"></button>
        <button v-if="points > 0" @click="unassignPoint">-<img class="pip" :src="icons[type]"></button>
      </template>
    </div>

    <div class="project-upgrade" :class="{upgrading: upgradeQueued}" v-if="status == 'Active' && nextUpgrade !== null">
      <div class="project-upgrade--title">
        <template v-if="upgradeQueued">
          <div>Upgrading in one planning cycle.</div>
        </template>
        <template v-else>
          <div>Next Level</div>
          <div>{{nextUpgrade.cost}}<img class="pip" :src="icons.political_capital"></div>
          <button @click="upgrade(p)">Upgrade</button>
        </template>
      </div>
      <Effects :effects="nextUpgrade.effects" />
    </div>

  </template>
  <template v-slot:back>
    <p>{{description}}</p>
    <div class="card-image-attribution">
      Image: {{image.attribution}}
    </div>
  </template>
  <template v-slot:footer>
    <div>GOSPLANT</div>
  </template>
</Card>
</template>

<script>
import Card from './Card.vue';
import game from '/src/game';
import state from '/src/state';
import {activeEffects} from '/src/display/project';
import Effects from 'components/Effects.vue';
import PROJECTS from '/assets/content/projects.json';
import NPCS from '/assets/content/npcs.json';

const MAX_POINTS = 15;

export default {
  props: ['project'],
  components: {
    Card,
    Effects,
  },
  data() {
    return {
      ...this.project,
      ...PROJECTS[this.project.id],
    };
  },
  watch: {
    project(project) {
      // Kind of hacky, but update data when the project changes
      Object.keys(project).forEach((k) => {
        this[k] = project[k];
      });
    }
  },
  computed: {
    type() {
      return this.kind.toLowerCase();
    },
    remainingCost() {
      if (this.status == 'Active' || this.status == 'Finished') {
        return null;
      } else if (this.status == 'Building') {
        let years = game.yearsRemaining(this.project);
        return `${years} years left`;
      } else {
        let cost = this.points > 0 ? this.estimate : this.cost;
        if (this.kind == 'Policy') {
          return cost;
        } else {
          return `${cost} years`;
        }
      }
    },
    hasLevels() {
      return this.status == 'Active'
        && this.kind == 'Policy'
        && this.upgrades.length > 0;
    },
    nextUpgrade() {
      if (this.upgrades.length === 0) {
        return null;
      }
      let idx = this.level;
      if (idx >= this.upgrades.length) {
        return null;
      }
      let upgrade = this.upgrades[idx];
      return {
        cost: upgrade.cost,
        effects: this.upgrades[idx].effects,
      }
    },
    upgradeQueued() {
        return state.queuedUpgrades[this.id] == true;
    },
    activeEffects() {
      return activeEffects(this);
    },
    supportersDetailed() {
      return this.supporters
        .filter((id) => !state.gameState.npcs[id].locked)
        .map((id) => NPCS[id]);
    },
    opposersDetailed() {
      return this.opposers
        .filter((id) => !state.gameState.npcs[id].locked)
        .map((id) => NPCS[id]);
    },
    costTip() {
      if (this.kind == 'Policy') {
        return {
          icon: 'political_capital',
          text: `This policy costs ${this.remainingCost} political capital to implement.`
        }
      } else {
        return {
          icon: this.type,
          text: `This will take about ${this.remainingCost} to finish. Allocate more ${this.kind} points to accelerate its progress.`
        }
      }
    }
  },
  methods: {
    assignPoint() {
      if (state.points[this.type] > 0 && this.points < MAX_POINTS) {
        game.setProjectPoints(this.id, this.points + 1);
        if (this.status !== 'Building') {
          game.startProject(this.id);

          // Manually update status
          this.status = state.gameState.projects[this.id].status;
        }
        state.points[this.type]--;
      }
    },
    unassignPoint() {
      if (this.points > 0) {
        game.setProjectPoints(this.id, this.points - 1);
        if (this.status == 'Building' && this.points <= 1) {
          game.stopProject(this.id);

          // Manually update status
          this.status = state.gameState.projects[this.id].status;
        }
        state.points[this.type]++;
      }
    },
    payPoints() {
      // Only policies have points paid all at once,
      // rather than assigned.
      let available = state.gameState.political_capital;
      if (this.status == 'Inactive' && available >= this.cost) {
        game.changePoliticalCapital(-this.cost);
        game.startProject(this.id);
      }
    },
    upgrade() {
      let nextUpgrade = this.nextUpgrade;
      let available = state.gameState.political_capital;
      if (nextUpgrade && available >= nextUpgrade.cost) {
        game.changePoliticalCapital(-this.cost);

        // Policies upgraded instantly
        if (this.kind == 'Policy') {
          game.upgradeProject(this.id);
        } else {
          state.queuedUpgrades[this.id] = true;
        }
      }
    }
  }
}
</script>

<style>
.project-cost {
  color: #fff;
  background: rgba(25,25,25,0.9);
  padding: 0 0.2em;
  border-radius: 0.2em;
  text-transform: uppercase;
  font-size: 0.9em;
}
.project-cost img {
  height: 12px;
}
.project-points {
  max-width: 110px;
}

.project-upgrade {
  background: #333;
  padding: 0.25em 0.5em;
  border-radius: 0.2em;
  font-size: 0.9em;
  border: 2px solid #444;
}
.project-upgrade.upgrading {
  border: 2px solid #43CC70;
}
.project-upgrade--title {
  display: flex;
  font-size: 0.9em;
  margin-bottom: 0.2em;
  justify-content: space-between;
  border-bottom: 1px solid #ddd;
  padding-bottom: 0.1em;
}
.project-upgrade--title button {
  font-size: 0.9em;
  padding: 0 1em;
}
.project-upgrade--title img {
  width: 12px;
}
</style>
