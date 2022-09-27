<template>
<Card :background="style.background" :color="style.color" :class="{'in-progress': project.status == 'Building', 'is-new': isNew}">
  <template v-slot:header>
    <div>{{t(project.group)}}</div>
    <img v-if="isNew" class="new-card-icon" src="/assets/new.svg" />
    <div v-if="implemented" class="project-cost">
      <template v-if="hasLevels">
        {{t('Level')}} {{level+1}}
      </template>
      <template v-else>
        <img :src="icons.check_blk"> {{t('Completed')}}
      </template>
    </div>
    <div v-else class="project-cost" v-tip="costTip">
      <template v-if="project.kind != 'Policy' || project.status == 'Building'"><img :src="icons.time"/> </template>{{remainingCost}}<img :src="icons.political_capital" v-if="project.kind == 'Policy' && project.status != 'Building'">
    </div>
    <img class="barcode" src="/assets/barcode.png" />
  </template>
  <template v-slot:figure>
    <div class="project-required-majority" v-if="project.required_majority > 0 && !majoritySatisfied">
      <div>
        <img :src="icons.warning" />
        {{t('Because of opposition, this requires a majority in parliament.')}}
      </div>
    </div>
    <img class="card-image" :src="info.image.fname ? `/assets/content/images/${info.image.fname}` : '/assets/missing_content.png'" />
    <div v-if="project.kind != 'Policy' && project.status == 'Building'" class="card-tack-ul project-points">
      <img
        v-for="i in consts.maxPoints"
        class="pip"
        v-tip="{text: t(`{points} {kind} points are allocated to this project`, {points: project.points, kind: t(project.kind)}), icon: type}"
        :class="{'empty-point': i > project.points}"
        :src="icons[type]">
    </div>

    <div class="opposers" v-if="opposersDetailed.length > 0">
      <img v-for="npc in opposersDetailed" v-tip="{text: t(`{name} is opposed to this. If you implement it, your relationship will worsen by -<img src='{icon}' />.`, {name: t(npc.name), icon: icons.relationship}), icon: npc.name}" :src="icons[npc.name]">
    </div>
    <div class="supporters" v-if="supportersDetailed.length > 0">
      <img v-for="npc in supportersDetailed" v-tip="{text: t(`{name} supports this. If you implement it, your relationship will improve by +<img src='{icon}' />.`, {name: t(npc.name), icon: icons.relationship}), icon: npc.name}" :src="icons[npc.name]">
    </div>
  </template>
  <template v-slot:name>
    {{t(project.name)}}
  </template>
  <template v-slot:body>
    <div class="passed-stamp" v-if="project.kind == 'Policy' && (project.status == 'Active' || project.status == 'Building')"><img src="/assets/stamp.svg"></div>
    <Effects class="solo-effects" :effects="activeEffects" />

    <div class="project-upgrade" :class="{upgrading: upgradeQueued}" v-if="project.status == 'Active' && nextUpgrade !== null">
      <div class="project-upgrade--title">
        <template v-if="upgradeQueued">
          <div>{{t('Upgrading in one planning cycle.')}}</div>
        </template>
        <template v-else>
          <div>{{t('Next Level')}}</div>
          <div>{{nextUpgrade.cost}}<img class="pip" :src="icons.political_capital"></div>
        </template>
      </div>
      <Effects :effects="nextUpgrade.effects" />
    </div>

    <div class="project-upgrade" v-if="project.status == 'Active' && canDowngrade">
      <div class="project-upgrade--title">
        <div>{{t('Prev Level')}}</div>
      </div>
      <Effects :effects="prevUpgrade.effects" />
    </div>

    <div class="project-status" v-if="project.status == 'Building'">{{ project.kind == 'Research' ? t('Researching') : t('Building')}}</div>

  </template>
  <template v-slot:top-back>
    <p class="card-desc">{{t(info.description)}}</p>
  </template>
  <template v-slot:bot-back>
    <div class="political-effects" v-if="opposersDetailed.length > 0 || supportersDetailed.length > 0">
      <div class="political-effects-title">{{t('Political Effects')}}</div>
      <div class="political-effects-cols">
        <div class="political-effects-opposers" v-if="opposersDetailed.length > 0">
          <div class="political-effects-label">{{t('Nay')}}</div>
          <div class="political-effects-portraits">
            <img v-for="npc in opposersDetailed" v-tip="{text: t(`{name} is opposed to this. If you implement it, your relationship will worsen by -<img src='{icon}' />.`, {name: t(npc.name), icon: icons.relationship}), icon: npc.name}" :src="icons[npc.name]">
          </div>
        </div>
        <div class="political-effects-supporters" v-if="supportersDetailed.length > 0">
          <div class="political-effects-label">{{t('Yea')}}</div>
          <div class="political-effects-portraits">
            <img v-for="npc in supportersDetailed" v-tip="{text: t(`{name} supports this. If you implement it, your relationship will improve by +<img src='{icon}' />.`, {name: t(npc.name), icon: icons.relationship}), icon: npc.name}" :src="icons[npc.name]">
          </div>
        </div>
      </div>
    </div>
    <div v-else class="card-spacer"></div>
    <div class="card-image-attribution">
      {{t('Image:')}} {{info.image.attribution}}
    </div>
  </template>
</Card>
</template>

<script>
import t from '/src/i18n';
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
  computed: {
    info() {
      return PROJECTS[this.project.id];
    },
    isNew() {
      return !state.viewed.includes(this.project.ref_id);
    },
    type() {
      return this.project.kind.toLowerCase();
    },
    style() {
      let style = consts.groupStyle[this.project.group];
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
      } else if (this.project.status == 'Building') {
        if (this.project.kind == 'Policy') {
          return t('1 planning cycle left');
        } else {
          let years = years_remaining(this.project.progress, this.project.points, this.project.cost);
          return t(`{years} yrs left`, {years});
        }
      } else {
        let cost = this.project.points > 0 ? this.project.estimate : this.project.cost;
        if (this.project.kind == 'Policy') {
          let changes = state.planChanges[this.project.id];
          if (changes && changes.withdrawn) {
            return 0;
          } else {
            return cost;
          }
        } else {
          return t('{cost} yrs', {cost});
        }
      }
    },
    hasLevels() {
      return this.info.upgrades.length > 0;
    },
    nextUpgrade() {
      if (this.info.upgrades.length === 0) {
        return null;
      }
      let idx = this.project.level;
      if (idx >= this.info.upgrades.length) {
        return null;
      }
      let upgrade = this.info.upgrades[idx];

      let cost = upgrade.cost;
      let changes = state.planChanges[this.project.id];
      if (changes && changes.downgrades > 0) {
        cost = 0;
      }
      return {
        cost,
        effects: this.info.upgrades[idx].effects,
      }
    },
    prevUpgrade() {
      if (this.canDowngrade) {
        let idx = this.project.level - 2;
        let upgrade = idx < 0 ? {effects: this.info.effects, cost: 0} : this.info.upgrades[idx];
        return {
          cost: upgrade.cost,
          effects: upgrade.effects,
        }
      }
      return null;
    },
    canDowngrade() {
      return this.project.kind == 'Policy' && this.project.level > 0;
    },
    upgradeQueued() {
        return state.queuedUpgrades[this.project.id] == true;
    },
    activeEffects() {
      return activeEffects(this.project);
    },
    supportersDetailed() {
      return this.project.supporters
        .filter((id) => !state.gameState.npcs[id].locked)
        .map((id) => NPCS[id]);
    },
    opposersDetailed() {
      return this.project.opposers
        .filter((id) => !state.gameState.npcs[id].locked)
        .map((id) => NPCS[id]);
    },
    requiredMajorityFraction() {
      return decimalToFraction(this.project.required_majority);
    },
    majoritySatisfied() {
      if (state.gameState.flags.includes('ParliamentSuspended')) {
        return true;
      } else {
        let playerSeats = game.playerSeats();
        return playerSeats >= this.project.required_majority;
      }
    },
    costTip() {
      if (this.project.kind == 'Policy') {
        return {
          icon: 'political_capital',
          text: t(`This policy costs {remainingCost} political capital to implement.`, {
            remainingCost: this.remainingCost})
        }
      } else {
        return {
          icon: this.type,
          text: t(`This will take about {remainingCost} to finish. Allocate more {kind} points to accelerate its progress.`, {
            remainingCost: this.remainingCost, kind: t(this.project.kind)})
        }
      }
    },
    implemented() {
      return this.project.status == 'Finished' || this.project.status == 'Active';
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
          game.upgradeProject(this.project.id);
        } else {
          state.queuedUpgrades[this.project.id] = true;
        }
        this.$emit('change');
      }
    },
    halt() {
      game.stopProject(this.project.id);
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
  font-size: 1em;
  padding: 0.1em 0.25em 0;
  line-height: 1.2;
  display: flex;
  text-transform: none;
  white-space: nowrap;
}
.project-cost img {
  height: 13px;
  margin-top: 0 !important;
  width: auto !important;
}
.project-cost img:first-child {
  margin-right: 0.2em;
}
.project-points {
  max-width: 58px;
  text-align: left;
}
.project-points .pip:first-child,
.project-points .pip:nth-child(7) {
  margin-left: 0px;
}

.project-status {
  color: #000;
  border-radius: 1em;
  text-align: center;
  font-family: 'W95FA', monospace;
  font-size: 0.9em;
  padding: 0.4em 0.5em 0.3em;
  text-transform: uppercase;
  background: var(--colour-pink);
  position: absolute;
  left: 50%;
  transform: translate(-50%, 50%);
  bottom: 0;
  border: 1px solid #438d0c;
  z-index: 1;

  backface-visibility: hidden;
  -webkit-backface-visibility: hidden;
}

.project-upgrade {
  background: rgba(0,0,0,0.15);
  padding: 0.1em 0.3em;
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
.project-upgrade .effect--text img {
  height: 12px;
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
  background: rgba(0,0,0,0.6);
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
.political-effects-portraits{
  display: flex;
}
.political-effects img {
  width: 28px;
  image-rendering: auto;
  margin: 0 2px;
}

.political-effects-cols {
  display: flex;
  justify-content: space-evenly;
}
.political-effects-opposers,
.political-effects-supporters {
  background: rgba(0,0,0,0.4);
  /* width: 64px; */
  margin: 0.25em;
  padding: 0.5em;
  text-align: center;
  border-radius: 0.5em;

  border-right: 1px solid rgba(255,255,255,0.5);
  border-bottom: 1px solid rgba(255,255,255,0.5);
  border-top: 1px solid rgba(0,0,0,0.4);
  border-left: 1px solid rgba(0,0,0,0.4);

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
  top: -160%;
  left: 10px;
  pointer-events: none;
}
.passed-stamp img {
  width: 260px !important;
}

.in-progress {
  animation-duration: 1.5s;
  animation-name: progresspulse;
  animation-iteration-count: infinite;
  animation-direction: alternate;
}

.is-new {
  box-shadow: 0 0 12px red;
}

.empty-point {
  opacity: 0.5;
  filter: grayscale(0.6) brightness(1.3);
}

@keyframes progresspulse {
  from {
    box-shadow: 0 0 2px var(--colour-pink), inset 1px 0px 8px var(--colour-pink);
  }

  to {
    box-shadow: 0 0 24px var(--colour-pink), inset 1px 0px 8px var(--colour-pink);
  }
}
</style>
