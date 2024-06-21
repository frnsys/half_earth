<template>
  <div class="scanbar-wrapper" ref="target">
    <div class="mini-scanbar">
        <div class="scanbar-base">
          <div class="scan-progress-bar" ref="progress"></div>
        </div>
        <div class="scanbar-led scanbar-led-ok"></div>
        <div class="scanbar-led scanbar-led-bad"></div>
        <div class="card-scan-target"></div>
    </div>
  </div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
import animate from 'lib/anim';
import tutorial from '/src/tutorial';
import ScannerMixin from '../ScannerMixin';
import PROJECTS from 'content/projects.json';

export default {
  props: ['project'],
  mixins: [ScannerMixin],
  computed: {
    shouldShow() {
      return true;
    },
    revealTarget() {
      return 65;
    },
    scanTime() {
      return consts.projectCardScanTime;
    },
    upgradeQueued() {
      return state.queuedUpgrades[this.project.id] == true;
    },
    nextUpgrade() {
      let upgrades = PROJECTS[this.project.id].upgrades;
      if (upgrades.length === 0) {
        return null;
      }
      let idx = this.project.level;
      if (idx >= upgrades.length) {
        return null;
      }
      let upgrade = upgrades[idx];
      return {
        cost: upgrade.cost,
        effects: upgrades[idx].effects,
      }
    },

  },
  methods: {
    scanAllowed() {
      const p = this.project;
      let playerSeats = game.playerSeats();
      if (p.required_majority > 0
        && playerSeats < p.required_majority) {
        return false;
      } else if (this.upgradeQueued) {
        return false;
      } else if (this.nextUpgrade) {
        return true;
      } else if (p.kind == 'Policy' && p.status == 'Active') {
        return false;
      } else {
        return true;
      }
    },
    finishScan() {
      const p = this.project;
      let projectActive = p.status == 'Active' || p.status == 'Finished';

      if (!(p.id in state.planChanges)) {
        state.planChanges[p.id] = {
          points: 0,
          upgrades: 0,
          downgrades: 0,
          withdrawn: false,
          passed: false,
        };
      }
      let changes = state.planChanges[p.id];

      // Upgrading projects
      if (projectActive && this.nextUpgrade && this.upgradeProject) {
        if (this.nextUpgrade) {
          let free = changes.downgrades > 0;
          if (free) {
            changes.downgrades--;
          }
          this.upgradeProject(free);
          this.pulseLevel();
          if (this.nextUpgrade) {
            this.scanCard();
          }

          // Refundable upgrade
          changes.upgrades++;
        }

      // Adding points to Research/Infrastructure
      } else if (this.project.kind !== 'Policy' && this.buyPoint()) {
        if (state.tutorial == tutorial.PROJECTS) {
          state.tutorial++;
        }
        this.assignPoint(p);
        this.pulseCard();
        this.scanCard();

        // Refundable points
        changes.points++;

      // Passing Policies
      // Free if withdrawn in this same session (i.e. undo the withdraw)
      } else if (this.project.kind == 'Policy' && (changes.withdrawn || this.payPoints())) {
        if (state.tutorial == tutorial.PROJECTS) {
          state.tutorial++;
        }

        this.passPolicy();
        this.pulseCard();
        this.shakeScreen();

        // Refundable
        if (changes.withdrawn) {
          changes.withdrawn = false;
        } else {
          changes.passed = true;
        }

        if (this.nextUpgrade) {
          this.scanCard();
        }

      // If not enough PC
      } else {
        this.rejectScan();
        this.shakeProgress();
        this.stopScanningCard();
      }
    },

    // Actions
    buyPoint() {
      if (this.project.points >= consts.maxPoints) {
        return false;
      }
      if (this.project.kind == 'Research' && state.points.research > 0) {
        return true;
      }
      let cost = game.nextPointCost(this.project.kind);
      if (cost <= state.gameState.political_capital) {
        game.changePoliticalCapital(-cost);
        state.points[this.project.kind.toLowerCase()]++;
        if (this.project.kind == 'Research') {
          state.refundableResearchPoints++;
        }
        return true;
      }
      return false;
    },
    payPoints() {
      // Only policies have points paid all at once,
      // rather than assigned.
      let available = state.gameState.political_capital;
      if (this.project.status == 'Inactive' && available >= this.project.cost) {
        game.changePoliticalCapital(-this.project.cost);
        return true;
      }
      return false;
    },
    passPolicy() {
      game.startProject(this.project.id);
      this.$emit('change');
    },
    assignPoint() {
      let p = this.project;
      let type = p.kind.toLowerCase();
      if (state.points[type] > 0 && p.points < consts.maxPoints) {
        game.setProjectPoints(p.id, p.points + 1);
        if (p.status !== 'Building') {
          game.startProject(p.id);

          // Manually update status
          p.status = state.gameState.projects[p.id].status;
        }
        state.points[type]--;
        this.$emit('change');
      }
    },
    upgradeProject(free) {
      let nextUpgrade = this.nextUpgrade;
      let available = state.gameState.political_capital;
      if (nextUpgrade && (free || available >= nextUpgrade.cost)) {
        if (!free) {
          game.changePoliticalCapital(-nextUpgrade.cost);
        }

        // Policies upgraded instantly
        if (this.project.kind == 'Policy') {
          game.upgradeProject(this.project.id);
        } else {
          state.queuedUpgrades[this.project.id] = true;
        }
        this.$emit('change');
        return true;
      }
      return false;
    },

    // Animations
    pulseLevel() {
      let el = document.querySelector('.draggable.active .project-cost');
      if (el) {
        animate(1, 1.2, 200, (val) => {
          el.style.transform = `scale(${val})`;
        }, () => {
          animate(1.2, 1, 200, (val) => {
            el.style.transform = `scale(${val})`;
          });
        });
      }
    },
  }
}
</script>
