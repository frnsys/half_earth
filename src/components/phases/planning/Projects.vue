<template>
<div class="plan-change-select planning--page" :class="{scrolling}">
  <div class="planning--page-tabs">
   <div class="project-tab" @click="type = 'Research'" :class="{selected: type == 'Research'}">
      <img :src="icons.research" />
      <div>Research</div>
    </div>
   <div class="project-tab" @click="type = 'Initiative'" :class="{selected: type == 'Initiative'}">
      <img :src="icons.initiative" />
      <div>Infrastructure</div>
    </div>
   <div class="project-tab" @click="type = 'Policy'" :class="{selected: type == 'Policy'}">
      <img :src="icons.policy" />
      <div>Policies</div>
    </div>
    <div @click="$emit('close')">Back</div>
  </div>
  <div class="card-drag-target" ref="target"></div>

  <div class="card-withdraw-target" ref="withdrawTarget">
    Withdraw
    <div class="withdraw-bar" ref="withdrawProgress"></div>
  </div>

  <Cards @dragStart="onDragStart" @dragEnd="onDragEnd" :enabled="scrollable">
    <Draggable @drag="onDragVertical"
      @dragStop="onDragVerticalStop"
      v-for="i in projectOrder"
      :minY="minDragY()"
      :maxY="maxDragY()"
      :draggable="focusedProject == projects[i].id"
      :id="projects[i].id"
      :key="projects[i].id">
      <ProjectCard
        :project="projects[i]"
        @change="$emit('change')" />
    </Draggable>
  </Cards>
  <footer>
    <div class="pips" ref="scanWrapper">
      <div class="scan-progress" ref="scanProgress"></div>
      <template v-if="type == 'Policy'">
        {{availablePoints}}<img class="pip" :src="icons.political_capital">
      </template>
      <template v-if="type == 'Research'">
        <template v-if="availablePoints > 0">
          {{availablePoints}}<img class="pip" :src="icons.research">
        </template>
        <template v-else>
          {{nextPointCost}}<img class="pip" :src="icons.political_capital"> ⮕ <img class="pip" :src="icons.research">
        </template>
      </template>
      <template v-if="type == 'Initiative'">
        <template v-if="availablePoints > 0">
          {{availablePoints}}<img class="pip" :src="icons.initiative">
        </template>
        <template v-else>
          {{nextPointCost}}<img class="pip" :src="icons.political_capital"> ⮕ <img class="pip" :src="icons.initiative">
        </template>
      </template>
    </div>
  </footer>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
import Cards from 'components/cards/Cards.vue';
import Draggable from 'components/cards/Draggable.vue';
import ProjectCard from 'components/cards/ProjectCard.vue';
import PROJECTS from '/assets/content/projects.json';
import animate from 'lib/anim';

const CARDHEIGHT = 420;
const WITHDRAWHEIGHT = 68;

export default {
  components: {
    Cards,
    Draggable,
    ProjectCard,
  },
  data() {
    return {
      state,
      scrolling: false,
      scrollable: true,
      focusedProject: 0,
      type: 'Research',
      projectOrder: [],

      scanning: false,
      scanAnim: null,

      withdrawing: false,
      withdrawAnim: null,
    };
  },
  watch: {
    type(type) {
      this.scrolling = false;
      this.scrollable = true;
      // Kind of hacky, but
      // figure out what the focused card is
      this.$nextTick(() => {
        for (let el of document.querySelectorAll('.draggable')) {
          let rect = el.getBoundingClientRect();
          if (rect.x > 0) {
            this.focusedProject = el.id;
            break;
          }
        }
      });
    }
  },
  computed: {
    selectedProject() {
      if (this.focusedProject !== null) {
        return state.gameState.projects[this.focusedProject];
      }
    },
    projectOrder() {
      let projects = state.gameState.projects
        .filter((p) => p.kind == this.type && !p.locked);

      let idxs = projects.map((p, i) => i);
      idxs.sort((a, b) => projects[a].name.toLowerCase().localeCompare(projects[b].name.toLowerCase()))
      return idxs;
    },
    projects() {
      return state.gameState.projects.filter((p) => p.kind == this.type && !p.locked);
    },
    availablePoints() {
      if (this.type == 'Policy') {
        return state.gameState.political_capital;
      } else {
        return state.points[this.type.toLowerCase()];
      }
    },
    nextPointCost() {
      return consts.pointCost;
    },
  },
  methods: {
    buyPoint() {
      let cost = this.nextPointCost;
      if (cost <= state.gameState.political_capital) {
        game.changePoliticalCapital(-cost);
        state.points[this.type.toLowerCase()]++;
        return true;
      }
      return false;
    },
    payPoints(project) {
      // Only policies have points paid all at once,
      // rather than assigned.
      let available = state.gameState.political_capital;
      if (project.status == 'Inactive' && available >= project.cost) {
        game.changePoliticalCapital(-project.cost);
        game.startProject(project.id);
        this.$emit('change');
        return true;
      }
      return false;
    },
    assignPoint(project) {
      let type = project.kind.toLowerCase();
      if (state.points[type] > 0 && project.points < consts.maxPoints) {
        game.setProjectPoints(project.id, project.points + 1);
        if (project.status !== 'Building') {
          game.startProject(project.id);

          // Manually update status
          project.status = state.gameState.projects[project.id].status;
        }
        state.points[type]--;
        this.$emit('change'); // TODO does this need to be adjusted?
      }
    },
    onDragStart() {
      this.focusedProject = null;
      this.scrolling = true;
    },
    pulseScan() {
      // Pulse the scan progress when it's full
      animate(1, 1.2, 200, (val) => {
        this.$refs.scanWrapper.style.transform = `scale(${val})`;
      }, () => {
        animate(1.2, 1, 200, (val) => {
          this.$refs.scanWrapper.style.transform = `scale(${val})`;
        });
      });
    },
    rejectScan() {
      this.$refs.target.classList.add('no-scan');
    },
    scanCard() {
      this.scanAnim = animate(0, 100, consts.cardScanTime * 1000, (val) => {
        this.$refs.scanProgress.style.width = `${val}%`;
      }, () => {
        if (this.scanning) {
          // Try to buy point
          let projectActive = this.selectedProject.status == 'Active' || this.selectedProject.status == 'Finished';
          if (projectActive && this.nextUpgrade(this.selectedProject) && this.upgradeProject(this.selectedProject)) {
            this.pulseScan();
            if (this.nextUpgrade(this.selectedProject)) {
              this.scanCard();
            }

          } else if (this.type !== 'Policy' && this.buyPoint()) {
            this.assignPoint(this.selectedProject);

            this.pulseScan();
            this.scanCard();

          } else if (this.type == 'Policy' && this.payPoints(this.selectedProject)) {
            this.pulseScan();
            document.body.classList.add('screenshake');
            setTimeout(() => {
              document.body.classList.remove('screenshake');
            }, 350);

          // If not enough PC
          } else {
            this.rejectScan();

            // Pulse the scan progress when it's full
            this.$refs.scanWrapper.classList.add('not-enough-pc');
            animate(1, 1.2, 200, (val) => {
              this.$refs.scanWrapper.style.transform = `scale(${val})`;
            }, () => {
              animate(1.2, 1, 200, (val) => {
                this.$refs.scanWrapper.style.transform = `scale(${val})`;
              }, () => {
                this.$refs.scanWrapper.classList.remove('not-enough-pc');
              });
            });

            this.stopScanningCard();
          }
        }
      }, true);
    },
    withdrawCard() {
      this.$refs.withdrawTarget.classList.add('withdrawing');
      this.withdrawAnim = animate(0, 100, consts.cardWithdrawTime * 1000, (val) => {
        this.$refs.withdrawProgress.style.width = `${val}%`;
      }, () => {
        if (this.withdrawing) {
          game.stopProject(this.selectedProject.id);
          this.$emit('change');
        }
        this.stopWithdrawingCard();
      }, true);
    },
    stopWithdrawingCard() {
      this.withdrawing = false;
      this.$refs.withdrawTarget.classList.remove('withdrawing');
      if (this.withdrawAnim) {
        this.withdrawAnim.stop();
        this.withdrawAnim = null;
        this.$refs.withdrawProgress.style.width = '0';
      }
    },
    scanAllowed() {
      let p = this.selectedProject;
      let playerSeats = game.playerSeats();
      if (p.required_majority > 0 && playerSeats < p.required_majority) {
        return false;
      } else if (this.upgradeQueued(this.selectedProject)) {
        return false;
      } else if (this.nextUpgrade(this.selectedProject)) {
        return true;
      } else if (p.kind == 'Policy' && p.status == 'Active') {
        return false;
      } else {
        return true;
      }
    },
    upgradeProject(project) {
      let nextUpgrade = this.nextUpgrade(project);
      let available = state.gameState.political_capital;
      if (nextUpgrade && available >= nextUpgrade.cost) {
        game.changePoliticalCapital(-project.cost);

        // Policies upgraded instantly
        if (project.kind == 'Policy') {
          game.upgradeProject(project.id);
        } else {
          state.queuedUpgrades[project.id] = true;
        }
        this.$emit('change');
        return true;
      }
      return false;
    },
    nextUpgrade(project) {
      let upgrades = PROJECTS[project.id].upgrades;
      if (upgrades.length === 0) {
        return null;
      }
      let idx = project.level;
      if (idx >= upgrades.length) {
        return null;
      }
      let upgrade = upgrades[idx];
      return {
        cost: upgrade.cost,
        effects: upgrades[idx].effects,
      }
    },
    upgradeQueued(project) {
        return state.queuedUpgrades[project.id] == true;
    },
    stopScanningCard() {
      this.scanning = false;
      this.$refs.target.classList.remove('scanning');
      this.$refs.target.classList.remove('no-scan');
      if (this.scanAnim) {
        this.scanAnim.stop();
        this.scanAnim = null;
        this.$refs.scanProgress.style.width = '0';
      }
    },
    onDragEnd(card) {
      if (card) {
        this.focusedProject = parseInt(card.child.id);
      }
      this.scrolling = false;
    },
    onDragVertical(c) {
      this.scrollable = false;

      let rect = c.$el.getBoundingClientRect();
      let target = this.$refs.target.getBoundingClientRect();
      if (rect.y < (target.y + target.height)) {
        if (!this.scanning && this.scanAllowed()) {
          this.scanning = true;
          this.$refs.target.classList.add('scanning');
          this.scanCard();
        } else if (!this.scanAllowed()) {
          this.rejectScan();
        }
      } else {
        this.stopScanningCard();
      }

      if (this.haltable(this.selectedProject)) {
        let botTarget = window.innerHeight - 150;
        let y = rect.y + CARDHEIGHT;
        if (y >= botTarget) {
          let p = Math.min(1, (y - botTarget)/WITHDRAWHEIGHT);
          this.$refs.withdrawTarget.style.bottom = `-${(1-p) * WITHDRAWHEIGHT}px`
        }

        if (y > window.innerHeight - WITHDRAWHEIGHT/3) {
          if (!this.withdrawing) {
            this.withdrawing = true;
            this.withdrawCard();
          }
        } else {
          this.stopWithdrawingCard();
        }
      }
    },
    onDragVerticalStop() {
      this.scrollable = true;
      this.stopScanningCard();
      this.stopWithdrawingCard();

      let current = parseInt(this.$refs.withdrawTarget.style.bottom);
      animate(current, -WITHDRAWHEIGHT, 100, (val) => {
        this.$refs.withdrawTarget.style.bottom = `${val}px`;
      });
    },
    // Has to be a method so
    // we get the correct ref
    minDragY() {
      if (this.$refs.target) {
        let target = this.$refs.target.getBoundingClientRect();
        return target.y + target.height - 10;
      } else {
        return 0;
      }
    },
    maxDragY() {
      return (this.selectedProject && this.haltable(this.selectedProject)) ? window.innerHeight - CARDHEIGHT : window.innerHeight - CARDHEIGHT - 68;
    },
    implemented(project) {
      return project.status == 'Finished' || project.status == 'Active';
    },
    haltable(project) {
      return this.implemented(project) && (project.kind == 'Policy' || project.ongoing);
    },
  }
}
</script>

<style>
.plan-change-select.scrolling .card {
  pointer-events: none;
}
.pips {
  position: relative;
}
.pips .scan-progress {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 0;
  z-index: -1;
  background: #FF66FF;
  box-shadow: 0px 0px 7px #FF66FF;
}

.card-drag-target.scanning {
  animation-duration: 1s;
  animation-name: scanning;
  animation-iteration-count: infinite;
  animation-direction: alternate;
}

.not-enough-pc {
  background: red !important;
  box-shadow: 0px 0px 7px red !important;
}
.no-scan {
  box-shadow: 0px 0px 7px red !important;
}

@keyframes scanning {
  from {
    box-shadow: 0 0 2px #6bff66, inset 1px 0px 8px #6bff66;
  }

  to {
    box-shadow: 0 0 24px #6bff66, inset 1px 0px 8px #6bff66;
  }
}

.card-withdraw-target {
  position: absolute;
  bottom: -68px;
  height: 68px;
  background: #eee;
  border-radius: 0.5em 0.5em 0 0;
  left: 0.5em;
  right: 0.5em;
  border-left: 3px solid #fff;
  box-shadow: 0 1px 4px rgb(0 0 0 / 60%), 0 0 8px #5f09f2;
  border-top: 2px solid #fff;
  border-right: 2px solid #666;
  z-index: 2;
  display: flex;
  align-items: center;
  justify-content: space-around;
  font-size: 1.3em;
  overflow: hidden;
}
.withdraw-bar {
  position: absolute;
  left: 0;
  bottom: 0;
  top: 0;
  width: 0;
  background: #f8bfb5;
  border-radius: 0 0.2em 0.2em 0;
  z-index: -1;
}
.card-withdraw-target.withdrawing {
  animation-duration: 0.35s;
  animation-name: withdrawing;
  animation-iteration-count: infinite;
  animation-direction: alternate;
}

@keyframes withdrawing {
  from {
    box-shadow: 0 0 2px #EB3941;
  }

  to {
    box-shadow: 0 0 24px #EB3941;
  }
}

.screenshake {
  animation-duration: 0.35s;
  animation-name: screenshake;
}

@keyframes screenshake {
  from {
    transform: translate(0, 0);
  }

  25% {
    transform: translate(-2px, -1px);
  }

  35% {
    transform: translate(1px, 2px);
  }

  55% {
    transform: translate(3px, 1px);
  }

  75% {
    transform: translate(-1px, 2px);
  }

  to {
    transform: translate(0, 0);
  }
}
</style>
