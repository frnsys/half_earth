import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
import PROJECTS from '/assets/content/projects.json';
import Draggable from 'components/cards/Draggable.vue';
import animate from 'lib/anim';

const CARD_HEIGHT = 420;
const WITHDRAW_HEIGHT = 68;
const SCANBAR_HEIGHT = 80;


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
    withdrawTime(){
      if(!this.process){
        return consts.cardWithdrawTime
      } else {
        return consts.processCardWithdrawTime
      }
    },
    scanTime(){
      if(!this.process){
        return consts.cardScanTime
      } else {
        return consts.processCardScanTime
      }
    },
    icon() {
      return this.project.kind.toLowerCase();
    },
    nextPointCost() {
      let discount = 0;
      if (this.type == 'Research') {
        if (game.isAlly('The Accelerationist')) {
          discount++;
        }
        if (state.gameState.flags.includes('HyperResearch')) {
          discount++;
        }
      }
      return Math.max(0, consts.pointCost - discount);
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
      if (this.process) return false;
      return this.project.kind == 'Policy' && this.project.level > 0;
    },
    implemented() {
      if (this.process) return false;
      return this.project.status == 'Finished' || this.project.status == 'Active';
    },
    haltable() {
      return this.implemented && (this.project.kind == 'Policy' || this.project.ongoing);
    },
    refundable() {
      if (this.process) return false;
      let changes = state.planChanges[this.project.id];
      return changes !== undefined && (
        changes.upgrades > 0
        || changes.points > 0
        || changes.passed || changes.withdrawn);
    },
    processAddable(){
      if(!this.process) return false
      if (this.points === 0) return false
      return true
    },
    processSubtractable(){
      if(!this.process) return false
      if (this.changedMixShare(this.process) === 0) return false
      return true
    }
  },
  methods: {
    // Actions
    buyPoint() {
      if (this.project.points >= consts.maxPoints) {
        return false;
      }
      if (this.project.kind == 'Research' && state.points.research > 0) {
        return true;
      }
      let cost = this.nextPointCost;
      if (cost <= state.gameState.political_capital) {
        game.changePoliticalCapital(-cost);
        state.points[this.type.toLowerCase()]++;
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
    pulseCard() {
      let el = document.querySelector('.draggable.active');
      animate(1, 1.1, 200, (val) => {
        el.style.transform = `scale(${val})`;
      }, () => {
        animate(1.1, 1, 200, (val) => {
          el.style.transform = `scale(${val})`;
        });
      });
    },
    shakeScreen() {
      document.body.classList.add('shake');
      setTimeout(() => {
        document.body.classList.remove('shake');
      }, 500);
    },

    // Movement handling
    checkDrag(component) {
      // Check if scanning
      let rect = component.$el.getBoundingClientRect();
      let target = this.$refs.target.getBoundingClientRect();

      


      // Pop down scanner
      let topTarget = -20;
      let y = rect.y;
      if (y >= topTarget) {

        var p = Math.min(1, SCANBAR_HEIGHT/(topTarget - y));
        var px =  Math.abs(p * SCANBAR_HEIGHT) + topTarget;
        this.$refs.target.style.top = `${px}px`
      }

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
      if (!this.scanning && (this.haltable || this.refundable || this.processSubtractable)) {
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

      let botTargetCurrent = parseInt(this.$refs.withdrawTarget.style.bottom);
      animate(botTargetCurrent, -WITHDRAW_HEIGHT, 100, (val) => {
        if (this.$refs.withdrawTarget) {
          this.$refs.withdrawTarget.style.bottom = `${val}px`;
        }
      });

      let topTargetCurrent = parseInt(this.$refs.target.style.top);
      animate(topTargetCurrent, 0, 100, (val) => {
        if (this.$refs.target) {
          this.$refs.target.style.top = `${val}px`;
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
      if((this.project && (this.haltable || this.refundable)) || (this.process && this.processSubtractable)){
        return window.innerHeight - CARD_HEIGHT
      } else {
        return window.innerHeight - CARD_HEIGHT - 80
      }
    },

    // Scanning
    scanAllowed() {
      if(!this.process){
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
      } else {
        if(this.processAddable){
          return true
        } else {
          return false
        }
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
      
      const proc = this.process;

      let p = this.project;
      this.scanAnim = animate(0, 100, this.scanTime * 1000, (val) => {
        this.$refs.scanProgress.style.width = `${val}%`;
      }, () => {

        
        
        if (this.scanning) {
          if(!this.process){
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
              // this.pulseProgress();
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
            } else if (this.type !== 'Policy' && this.buyPoint()) {
              this.assignPoint(p);
              // this.pulseProgress();
              this.pulseCard();
              this.scanCard();

              // Refundable points
              changes.points++;

            // Passing Policies
            // Free if withdrawn in this same session (i.e. undo the withdraw)
            } else if (this.type == 'Policy' && (changes.withdrawn || this.payPoints())) {
              this.passPolicy();
              // this.pulseProgress();
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
          } else {
            // Handle processes here
            if(this.processAddable){
              this.addPoint(proc);
              this.pulseCard();
              this.scanCard();
            } else {
              this.rejectScan();
              this.shakeProgress();
              this.stopScanningCard();
            }
          }
        }
      }, true);
    },
    stopScanningCard() {
      this.scanning = false;
      this.$refs.target.classList.remove('scanning');
      this.$refs.target.classList.remove('no-scan');
      this.$refs.target.parentElement.classList.remove('scan-ok');
      let active = document.querySelector('.draggable.active');
      if (active) active.classList.remove('scan-reject');
      if (this.scanAnim) {
        this.scanAnim.stop();
        this.scanAnim = null;
        this.$refs.scanProgress.style.width = '0';
      }

    },

    // Withdrawing
    withdrawCard() {
      this.$refs.withdrawTarget.classList.add('withdrawing');
      this.withdrawAnim = animate(0, 100, this.withdrawTime * 1000, (val) => {
        this.$refs.withdrawProgress.style.width = `${val}%`;
      }, () => {
        let keepWithdrawing = false;
        if (this.withdrawing) {
          if (!this.process){
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
              let refund = this.nextPointCost * points;

              // Don't allow stored research-only points to be converted into PC,
              // instead convert them back into research points
              if (this.project.kind == 'Research') {
                let excessPoints = Math.max(points - state.refundableResearchPoints, 0);
                refund = this.nextPointCost * (points - excessPoints);
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
        
         }else{
          this.removePoint(this.process);
         }
        }
        if (keepWithdrawing) {
          this.withdrawCard();
        } else {
          keepWithdrawing = this.processSubtractable;
          
          if (keepWithdrawing) {
            this.withdrawCard();
          } else {
            this.stopWithdrawingCard();
          }

        }
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
  },
}
