<template>
  <div class="card-withdraw-target" ref="target">
    {{ refundable ? 'Undo' : (canDowngrade ? 'Downgrade' : 'Withdraw') }}
    <div class="withdraw-bar" ref="progress"></div>
  </div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
import ScannerMixin from '../ScannerMixin';
import PROJECTS from '/assets/content/projects.json';

export default {
  props: ['project'],
  mixins: [ScannerMixin],
  computed: {
    shouldShow() {
      return this.scanAllowed();
    },
    revealTarget() {
      return -60;
    },
    scanTime() {
      return consts.projectCardWithdrawTime;
    },
    canDowngrade() {
      return this.project.kind == 'Policy' && this.project.level > 0;
    },
    implemented() {
      return this.project.status == 'Finished' || this.project.status == 'Active';
    },
    haltable() {
      return this.implemented && (this.project.kind == 'Policy' || this.project.ongoing);
    },
    refundable() {
      let changes = state.planChanges[this.project.id];
      return changes !== undefined && (
        changes.upgrades > 0
        || changes.points > 0
        || changes.passed || changes.withdrawn);
    },
  },
  methods: {
    scanAllowed() {
      return this.haltable || this.refundable;
    },
    finishScan() {
      this.shrinkPulseCard();
      let keepWithdrawing = false;
      if (!(this.project.id in state.planChanges)) {
        state.planChanges[this.project.id] = {
          points: 0,
          upgrades: 0,
          downgrades: 0,
          withdrawn: false,
          passed: false,
        };
      }
      let changes = state.planChanges[this.project.id];
      if (this.refundable) {
        if (changes.upgrades > 0) {
          let level = this.project.level;
          let upgrades = PROJECTS[this.project.id].upgrades;
          let cost = upgrades[level-1].cost;
          game.changePoliticalCapital(cost);
          if (this.project.kind == 'Policy') {
            game.downgradeProject(this.project.id);
          } else {
            state.queuedUpgrades[this.project.id] = false;
          }
          changes.upgrades--;

          // Can maybe keep withdrawing
          keepWithdrawing = changes.upgrades > 0 || changes.passed;
        } else if (changes.passed) {
          game.changePoliticalCapital(this.project.cost);
          game.stopProject(this.project.id);
          changes.passed = false;
        } else {
          let points = changes.points;
          let refund = game.nextPointCost(this.project.kind) * points;

          // Don't allow stored research-only points to be converted into PC,
          // instead convert them back into research points
          if (this.project.kind == 'Research') {
            let excessPoints = Math.max(points - state.refundableResearchPoints, 0);
            refund = game.nextPointCost(this.project.kind) * (points - excessPoints);
            state.refundableResearchPoints = Math.max(0, state.refundableResearchPoints - points);
            state.points.research += excessPoints;
          }
          this.unassignPoints(points);
          game.changePoliticalCapital(refund);
          changes.points = 0;
        }
      } else if (this.canDowngrade) {
        game.downgradeProject(this.project.id);
        keepWithdrawing = this.project.level > 0;
        changes.downgrades++;
      } else {
        game.stopProject(this.project.id);
        changes.withdrawn = true;
      }
      this.$emit('change');

      if (keepWithdrawing) {
        this.scanCard();
      } else {
        this.stopScanningCard();
      }
    },
    unassignPoints(points) {
      let p = this.project;
      let newPoints = p.points - points;
      game.setProjectPoints(p.id, newPoints);
      if (p.status == 'Building' && newPoints == 0) {
        game.stopProject(p.id);

        // Manually update status
        p.status = state.gameState.projects[p.id].status;
      }
      this.$emit('change');
    },
  }
}
</script>
