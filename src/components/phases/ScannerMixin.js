import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
import PROJECTS from '/assets/content/projects.json';
import Draggable from 'components/cards/Draggable.vue';
import animate from 'lib/anim';

const CARD_HEIGHT = 420;
const WITHDRAW_HEIGHT = 68;

export default {
  components: {
    Draggable,
  },
  data() {
    return {
      scanning: false,
      scanAnim: null,
      withdrawing: false,
      withdrawAnim: null,
    }
  },
  computed: {
    icon() {
      return this.project.kind.toLowerCase();
    },
    nextPointCost() {
      if (this.type == 'Research' && game.isAlly('The Accelerationist')) {
        return consts.discountedPointCost;
      } else {
        return consts.pointCost;
      }
    },
    availablePoints() {
      if (this.type == 'Policy') {
        return state.gameState.political_capital;
      } else {
        return state.points[this.type.toLowerCase()];
      }
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
    canDowngrade() {
      return this.project.kind == 'Policy' && this.project.level > 0;
    },
    implemented() {
      return this.project.status == 'Finished' || this.project.status == 'Active';
    },
    haltable() {
      return this.implemented && (this.project.kind == 'Policy' || this.project.ongoing);
    },
  },
  methods: {
    // Actions
    buyPoint() {
      if (this.project.points >= consts.maxPoints) {
        return false;
      }
      let cost = this.nextPointCost;
      if (cost <= state.gameState.political_capital) {
        game.changePoliticalCapital(-cost);
        state.points[this.type.toLowerCase()]++;
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
        game.startProject(this.project.id);
        this.$emit('change');
        return true;
      }
      return false;
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
    upgradeProject() {
      let nextUpgrade = this.nextUpgrade;
      let available = state.gameState.political_capital;
      if (nextUpgrade && available >= nextUpgrade.cost) {
        game.changePoliticalCapital(-this.project.cost);

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
    pulseProgress() {
      let el = this.$refs.scanProgress.parentElement;
      animate(1, 1.2, 200, (val) => {
        el.style.transform = `scale(${val})`;
      }, () => {
        animate(1.2, 1, 200, (val) => {
          el.style.transform = `scale(${val})`;
        });
      });
    },
    pulseLevel() {
      let el = document.querySelector('.draggable.active .project-cost');
      animate(1, 1.2, 200, (val) => {
        el.style.transform = `scale(${val})`;
      }, () => {
        animate(1.2, 1, 200, (val) => {
          el.style.transform = `scale(${val})`;
        });
      });
    },
    shakeProgress() {
      let el = this.$refs.scanProgress.parentElement;
      el.classList.add('scan-error');
      el.classList.add('shake');
      setTimeout(() => {
        el.classList.remove('shake');
        el.classList.remove('scan-error');
      }, 350);
    },
    shakeScreen() {
      document.body.classList.add('shake');
      setTimeout(() => {
        document.body.classList.remove('shake');
      }, 350);
    },

    // Movement handling
    checkDrag(component) {
      // Check if scanning
      let rect = component.$el.getBoundingClientRect();
      let target = this.$refs.target.getBoundingClientRect();
      if (rect.y < (target.y + target.height)) {
        if (!this.scanning && this.scanAllowed()) {
          this.scanning = true;
          this.$refs.target.parentElement.classList.add('scan-ok');
          this.$refs.target.classList.add('scanning');
          this.scanCard();
        } else if (!this.scanAllowed()) {
          this.rejectScan();
        }
      } else {
        this.stopScanningCard();
      }

      // Check if withdrawing
      if (!this.scanning && this.haltable) {
        let botTarget = window.innerHeight - 150;
        let y = rect.y + rect.height;
        if (y >= botTarget) {
          let p = Math.min(1, (y - botTarget)/WITHDRAW_HEIGHT);
          this.$refs.withdrawTarget.style.bottom = `-${(1-p) * WITHDRAW_HEIGHT}px`
        }

        if (y > window.innerHeight - WITHDRAW_HEIGHT/3) {
          if (!this.withdrawing) {
            this.withdrawing = true;
            this.withdrawCard();
          }
        } else {
          this.stopWithdrawingCard();
        }
      }
    },
    stopDrag() {
      this.stopScanningCard();
      this.stopWithdrawingCard();

      let current = parseInt(this.$refs.withdrawTarget.style.bottom);
      animate(current, -WITHDRAW_HEIGHT, 100, (val) => {
        if (this.$refs.withdrawTarget) {
          this.$refs.withdrawTarget.style.bottom = `${val}px`;
        }
      });
    },

    // Drag limits
    // Has to be methods so we get the correct ref
    yMin() {
      if (this.$refs.target) {
        let target = this.$refs.target.getBoundingClientRect();
        return target.y + target.height - 8;
      } else {
        return 0;
      }
    },
    yMax() {
      return (this.project && this.haltable) ? window.innerHeight - CARD_HEIGHT : window.innerHeight - CARD_HEIGHT - 80;
    },

    // Scanning
    scanAllowed() {
      let p = this.project;
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
    rejectScan() {
      this.$refs.target.parentElement.classList.add('scan-fail');
      this.$refs.target.classList.add('no-scan');
      document.querySelector('.draggable.active').classList.add('scan-reject');
      setTimeout(() => {
        this.$refs.target.parentElement.classList.remove('scan-fail');
      }, 500);
    },
    scanCard() {
      let p = this.project;
      this.scanAnim = animate(0, 100, consts.cardScanTime * 1000, (val) => {
        this.$refs.scanProgress.style.width = `${val}%`;
      }, () => {
        if (this.scanning) {
          // Try to buy point
          let projectActive = p.status == 'Active' || p.status == 'Finished';
          if (projectActive && this.nextUpgrade && this.upgradeProject) {
            this.pulseProgress();
            if (this.nextUpgrade) {
              this.upgradeProject();
              this.pulseLevel();
              this.scanCard();
            }

          } else if (this.type !== 'Policy' && this.buyPoint()) {
            this.assignPoint(p);

            this.pulseProgress();
            this.scanCard();

          } else if (this.type == 'Policy' && this.payPoints(p)) {
            this.pulseProgress();
            this.shakeScreen();

          // If not enough PC
          } else {
            this.rejectScan();
            this.shakeProgress();
            this.stopScanningCard();
          }
        }
      }, true);
    },
    stopScanningCard() {
      this.scanning = false;
      this.$refs.target.classList.remove('scanning');
      this.$refs.target.classList.remove('no-scan');
      this.$refs.target.parentElement.classList.remove('scan-ok');
      document.querySelector('.draggable.active').classList.remove('scan-reject');
      if (this.scanAnim) {
        this.scanAnim.stop();
        this.scanAnim = null;
        this.$refs.scanProgress.style.width = '0';
      }
    },

    // Withdrawing
    withdrawCard() {
      this.$refs.withdrawTarget.classList.add('withdrawing');
      this.withdrawAnim = animate(0, 100, consts.cardWithdrawTime * 1000, (val) => {
        this.$refs.withdrawProgress.style.width = `${val}%`;
      }, () => {
        if (this.withdrawing) {
          if (this.canDowngrade) {
            game.downgradeProject(this.project.id);
          } else {
            game.stopProject(this.project.id);
          }
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
  }
}
