<template>
<Card :background="style.background" :color="style.color" :class="{'in-progress': status == 'Building'}">
  <template v-slot:header>
    <div>{{group}}</div>
    <div v-if="implemented" class="project-cost">
      <template v-if="hasLevels">
        Level {{level+1}}
      </template>
      <template v-else>
        <img :src="icons.check_blk"> Completed
      </template>
    </div>
    <div v-else class="project-cost" v-tip="costTip">
      <template v-if="kind != 'Policy'"><img :src="icons.time"/> </template>{{remainingCost}}<img :src="icons.political_capital" v-if="kind == 'Policy'">
    </div>
    <img class="barcode" src="/assets/barcode.png" />
  </template>
  <template v-slot:figure>
    <div class="project-required-majority" v-if="required_majority > 0 && !majoritySatisfied">
      <div>
        <img :src="icons.warning" />
        Because of opposition, this requires a {{requiredMajorityFraction}} majority in parliament.
      </div>
    </div>
    <img class="card-image" :src="`/assets/content/images/${image.fname}`" />
    <div v-if="status == 'Building'" class="card-tack-ul project-points">
      <img
        v-for="_ in points"
        class="pip"
        v-tip="{text: `${points} ${kind} points are allocated to this project`, icon: type}"
        :src="icons[type]">
    </div>

    <div class="opposers" v-if="opposersDetailed.length > 0">
      <img v-for="npc in opposersDetailed" v-tip="{text: `${npc.name} is opposed to this. If you implement it, your relationship will worsen by -<img src='${icons.relationship}' />.`, icon: npc.name}" :src="icons[npc.name]">
    </div>
    <div class="supporters" v-if="supportersDetailed.length > 0">
      <img v-for="npc in supportersDetailed" v-tip="{text: `${npc.name} supports this. If you implement it, your relationship will improve by +<img src='${icons.relationship}' />.`, icon: npc.name}" :src="icons[npc.name]">
    </div>
  </template>
  <template v-slot:name>
    {{name}}
  </template>
  <template v-slot:body>
    <div class="passed-stamp" v-if="kind == 'Policy' && status == 'Active'"><img src="/assets/stamp.svg"></div>
    <Effects :effects="activeEffects" />

    <div class="project-upgrade" :class="{upgrading: upgradeQueued}" v-if="status == 'Active' && nextUpgrade !== null">
      <div class="project-upgrade--title">
        <template v-if="upgradeQueued">
          <div>Upgrading in one planning cycle.</div>
        </template>
        <template v-else>
          <div>Next Level</div>
          <div>{{nextUpgrade.cost}}<img class="pip" :src="icons.political_capital"></div>
        </template>
      </div>
      <Effects :effects="nextUpgrade.effects" />
    </div>

    <div class="project-status" v-if="status == 'Building'">{{ kind == 'Research' ? 'Researching' : 'Building'}}</div>

  </template>
  <template v-slot:top-back>
    <p class="card-desc">{{description}}</p>
  </template>
  <template v-slot:bot-back>
    <div class="political-effects" v-if="opposersDetailed.length > 0 || supportersDetailed.length > 0">
      <div class="political-effects-title">Political Effects</div>
      <div class="political-effects-cols">
        <div class="political-effects-opposers" v-if="opposersDetailed.length > 0">
          <div class="political-effects-label">Nay</div>
          <img v-for="npc in opposersDetailed" v-tip="{text: `${npc.name} is opposed to this. If you implement it, your relationship will worsen by -<img src='${icons.relationship}' />.`, icon: npc.name}" :src="icons[npc.name]">
        </div>
        <div class="political-effects-supporters" v-if="supportersDetailed.length > 0">
          <div class="political-effects-label">Yea</div>
          <img v-for="npc in supportersDetailed" v-tip="{text: `${npc.name} supports this. If you implement it, your relationship will improve by +<img src='${icons.relationship}' />.`, icon: npc.name}" :src="icons[npc.name]">
        </div>
      </div>
    </div>
    <div v-else class="card-spacer"></div>
    <div class="card-image-attribution">
      Image: {{image.attribution}}
    </div>
  </template>
</Card>
</template>

<script>
import Card from './Card.vue';
import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts.js';
import {activeEffects} from '/src/display/project';
import Effects from 'components/Effects.vue';
import PROJECTS from '/assets/content/projects.json';
import NPCS from '/assets/content/npcs.json';
import {years_remaining} from 'half-earth-engine';

/*
Description: Convert a decimal number into a fraction
Author: Michaël Niessen (© 2018)
Website: http://AssemblySys.com

If you find this script useful, you can show your
appreciation by getting Michaël a cup of coffee ;)
PayPal: https://www.paypal.me/MichaelNiessen

As long as this notice (including author name and details) is included and
UNALTERED, this code can be used and distributed freely.
*/
function decimalToFraction(value, donly = true) {
   var tolerance = 1.0E-6; // from how many decimals the number is rounded
   var h1 = 1;
   var h2 = 0;
   var k1 = 0;
   var k2 = 1;
   var negative = false;
   var i;

   if (parseInt(value) == value) { // if value is an integer, stop the script
      return value;
   } else if (value < 0) {
      negative = true;
      value = -value;
   }

   if (donly) {
      i = parseInt(value);
      value -= i;
   }

   var b = value;

   do {
      var a = Math.floor(b);
      var aux = h1;
      h1 = a * h1 + h2;
      h2 = aux;
      aux = k1;
      k1 = a * k1 + k2;
      k2 = aux;
      b = 1 / (b - a);
   } while (Math.abs(value - h1 / k1) > value * tolerance);

   return (negative ? "-" : '') + ((donly & (i != 0)) ? i + ' ' : '') + (h1 == 0 ? '' : h1 + "/" + k1);
}


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
    style() {
      let style = consts.groupStyle[this.group];
      if (!style) {
        return {
          background: '#e0e0e0',
          color: '#000',
        };
      } else {
        if (!style.color) {
          style.color = '#000';
        }
        return style;
      }
    },
    remainingCost() {
      if (this.implemented) {
        return null;
      } else if (this.status == 'Building') {
        let years = years_remaining(this.project.progress, this.project.points, this.project.cost);
        return `${years} yrs left`;
      } else {
        let cost = this.points > 0 ? this.estimate : this.cost;
        if (this.kind == 'Policy') {
          return cost;
        } else {
          return `${cost} yrs`;
        }
      }
    },
    hasLevels() {
      return this.upgrades.length > 0;
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
    requiredMajorityFraction() {
      return decimalToFraction(this.required_majority);
    },
    majoritySatisfied() {
      let playerSeats = game.playerSeats();
      return playerSeats >= this.required_majority;
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
    },
    implemented() {
      return this.status == 'Finished' || this.status == 'Active';
    },
  },
  methods: {
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
        this.$emit('change');
      }
    },
    halt() {
      game.stopProject(this.id);
      this.$emit('change');
    }
  }
}
</script>

<style>
.project-cost {
  color: #000;
  background: #fff;
  border-radius: 1em;
  border: 1px solid #000;
  text-align: center;
  font-family: 'W95FA', monospace;
  font-size: 0.9em;
  padding: 0.1em 0.25em 0;
  line-height: 1.2;
  display: flex;
  text-transform: none;
}
.project-cost img {
  height: 12px;
  margin-top: 0 !important;
  width: auto !important;
}
.project-cost img:first-child {
  margin-right: 0.2em;
}
.project-points {
  max-width: 110px;
  text-align: left;
}

.project-status {
  color: #fff;
  border-radius: 1em;
  text-align: center;
  font-family: 'W95FA', monospace;
  font-size: 0.9em;
  padding: 0.4em 0.5em 0.3em;
  text-transform: uppercase;
  background: #FF66FF;
  position: absolute;
  left: 50%;
  transform: translate(-50%, 50%);
  bottom: 0;
  border: 1px solid #b929b9;
}

.project-upgrade {
  background: rgba(0,0,0,0.15);
  padding: 0.25em 0.5em;
  border-radius: 0.2em;
  font-size: 0.9em;
  border: 1px solid rgba(0,0,0,0.2);
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

.project-group {
  padding: 0 0.1em;
  border-radius: 0.2em;
  color: #000;
  background: #888;
}

.project-required-majority {
  position: absolute;
  left: 0;
  bottom: 0;
  right: 0;
  top: 0;
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  text-align: center;
  background: rgba(0,0,0,0.8);
  color: #fff;
  font-size: 0.85em;
  padding: 1em;
}
.project-required-majority img {
  width: 24px;
  display: block;
  margin: 0 auto 0.5em;
}

.project-majority-tip {
  font-size: 0.6em;
  text-align: center;
}

.political-effects-label,
.political-effects-title {
  font-family: 'Inter', sans-serif;
  font-size: 0.7em;
  text-align: center;
  text-transform: uppercase;
  font-weight: bold;
  margin-bottom: 0.5em;
}
.political-effects img {
  width: 28px;
}
.political-effects-cols {
  display: flex;
  justify-content: space-evenly;
}
.political-effects-opposers,
.political-effects-supporters {
  background: rgba(0,0,0,0.8);
  width: 64px;
  margin: 0.25em;
  padding: 0.5em;
  text-align: center;
  border-radius: 0.5em;
}
.political-effects-supporters .political-effects-label {
  color: #2FE863;
}
.political-effects-opposers .political-effects-label {
  color: #FF0404;
}

.card-spacer, .political-effects {
  flex: 1;
}

.passed-stamp {
  position: absolute;
  z-index: 2;
  top: -10px;
}
.passed-stamp img {
  width: 240px !important;
}

.in-progress {
  animation-duration: 1.5s;
  animation-name: progresspulse;
  animation-iteration-count: infinite;
  animation-direction: alternate;
}

@keyframes progresspulse {
  from {
    box-shadow: 0 0 2px #FF66FF, inset 1px 0px 8px #FF66FF;
  }

  to {
    box-shadow: 0 0 24px #FF66FF, inset 1px 0px 8px #FF66FF;
  }
}
</style>
