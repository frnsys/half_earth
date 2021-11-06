<template>
<Card>
  <template v-slot:header>
    <div>{{name}}</div>
    <div v-if="kind == 'Policy'">
      <template v-if="status !== 'Active'">{{remainingCost}}<img :src="assets.icons.political_capital"></template>
      <template v-else>Implemented</template>
    </div>
    <div v-else>{{status !== 'Finished' ? remainingCost : 'Finished'}}</div>
  </template>
  <template v-slot:figure>
    <img class="card-image" :src="`/assets/content/images/${image.fname}`" />
    <div v-if="status == 'Building'" class="project-points">
      <img
        v-for="_ in points"
        class="pip"
        v-tip="{text: `${points} ${kind} points are allocated to this project`, icon: type}"
        :src="assets.resources[type]">
    </div>
    <div v-if="hasLevels" class="project-level">
      Level {{level+1}}
    </div>

    <div class="opposers">
      <div>Nay</div>
      <div>
        <img v-tip="{text: `The Authoritarian is opposed to this. If you ban it, your relationship will improve by +<img src='${assets.icons.relationship}' />.`, icon: 'authoritarian'}" src="/assets/characters/The Authoritarian.png">
        <img v-tip="{text: `The Economist is opposed to this process. If you ban it, your relationship will improve by +<img src='${assets.icons.relationship}' />.`, icon: 'economist'}" src="/assets/characters/The Economist.png">
        <img v-tip="{text: `The Technocrat is opposed to this process. If you ban it, your relationship will improve by +<img src='${assets.icons.relationship}' />.`, icon: 'technocrat'}" src="/assets/characters/The Technocrat.png">
      </div>
    </div>
    <div class="supporters">
      <div>Yea</div>
      <div>
        <img v-tip="{text: `The Scientist supports this. If you promote it, your relationship will improve by +<img src='${assets.icons.relationship}' />.`, icon: 'scientist'}" src="/assets/characters/The Scientist.png">
        <img v-tip="{text: `The Populist supports this. If you promote it, your relationship will improve by +<img src='${assets.icons.relationship}' />.`, icon: 'populist'}" src="/assets/characters/The Populist.png">
        <img v-tip="{text: `The Ecologist supports this. If you promote it, your relationship will improve by +<img src='${assets.icons.relationship}' />.`, icon: 'ecologist'}" src="/assets/characters/The Ecologist.png">
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
        <button @click="assignPoint">+<img class="pip" :src="`/assets/icons/pips/${type}.png`"></button>
        <button v-if="points > 0" @click="unassignPoint">-<img class="pip" :src="`/assets/icons/pips/${type}.png`"></button>
      </template>
    </div>

    <div class="project-upgrade" v-if="status == 'Active' && nextUpgrade !== null">
      <div class="project-upgrade--title">
        <div>Next Level</div>
        <div>{{nextUpgrade.cost}}<img class="pip" src="/assets/icons/pips/political_capital.png"></div>
        <button @click="upgrade(p)">Upgrade</button>
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
import display from 'lib/display';
import {nearestMultiple} from 'lib/util';
import Effects from 'components/Effects.vue';

function yearsForPoints(points, cost) {
  return Math.max(nearestMultiple(cost/(points**(1/3)), 5), 1);
}

export default {
  props: ['project'],
  components: {
    Card,
    Effects,
  },
  data() {
    let project = state.projects[this.project.id];
    return {
      ...this.project,
      ...project,
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
        let remaining = 1 - this.progress;
        let progressPerYear = 1/yearsForPoints(this.points, this.cost);
        let years = Math.round(remaining/progressPerYear);
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
      return this.status == 'Active' && this.kind == 'Policy' && this.upgrades.length > 0;
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
        effects: state.projects[this.id].upgrades[idx].effects,
      }
    },
    activeEffects() {
      if (this.status == 'Inactive') {
        return this.effects.concat(this.outcomeEffects);
      } else if (this.level === 0) {
        return this.effects;
      } else {
        return this.upgrades[this.level - 1].effects;
      }
    },
    outcomeEffects() {
      let allEffects = [];
      this.outcomes.forEach(({effects}) => {
        allEffects = allEffects.concat(effects)
      });

      // Remove duplicates
      allEffects = allEffects.filter((item, i) => {
        return allEffects.indexOf(item) == i;
      });

      return allEffects.map((e) => {
        e.random = true;
        return e;
      });
    },
  },
  methods: {
    assignPoint() {
      if (state.points[this.type] > 0) {
        game.setProjectPoints(this.id, this.points + 1);
        if (this.status !== 'Building') {
          game.startProject(this.id);
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
        game.upgradeProject(this.id);
      }
    }

  }
}
</script>

<style>
.project-points {
  position: absolute;
  left: 0.5em;
  top: 0.5em;
}
</style>
