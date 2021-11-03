import game from '/src/game';
import state from '/src/state';
import Card from './Card.vue';
import Cards from './Cards.vue';
import {describeEffect} from '/src/effects';
import {nearestMultiple} from '/src/lib/util';

function years_for_points(points, cost) {
  return Math.max(nearestMultiple(cost/(points**(1/3)), 5), 1);
}


export default (type) => ({
  components: {
    Card,
    Cards,
  },
  data() {
    return {
      state
    };
  },
  computed: {
    projects() {
      return state.gameState.projects.filter((p) => p.kind == type && !p.locked);
    },
    pointsInUse() {
      let active = this.projects.filter((p) => p.status == 'Building' || p.ongoing && p.status == 'Active');
      let points = active.map((p) => p.points).reduce((acc, v) => acc + v, 0);
      return points + this.availablePoints;
    },
    availablePoints() {
      if (type == 'Policy') {
        return state.gameState.political_capital;
      } else {
        return state.points[type].available;
      }
    },
    nextPointCost() {
      let i = this.pointsInUse;
      return Math.round((i+1)**1.5);
    },
  },
  methods: {
    effectDescs(effects) {
      let descs = effects
        .map((ev) => {
          let desc = describeEffect(ev);
          if (desc) {
            return `${ev.random ? '🎲 ' : ''}${desc}`;
          }
        })
        .filter((desc) => desc !== undefined);
      return descs.filter((item, i) => {
        return descs.indexOf(item) == i;
      });
    },
    remainingCost(p) {
      if (p.status == 'Active' || p.status == 'Finished') {
        return null;
      } else if (p.status == 'Building') {
        let remaining = 1 - p.progress;
        let progressPerYear = 1/years_for_points(p.points, p.cost);
        let years = Math.round(remaining/progressPerYear);
        return `${years} years left`;
      } else {
        let cost = p.points > 0 ? p.estimate : p.cost;
        if (p.type == 'Policy') {
          return cost;
        } else {
          return `${cost} years`;
        }
      }
    },
    imageForProject(p) {
      let image = state.projects[p.id].image;
      if (image.fname) {
        return `/assets/content/images/${image.fname}`;
      } else {
        return '/assets/placeholders/project.png';
      }
    },
    nextUpgrade(p) {
      if (p.upgrades.length === 0) {
        return null;
      }
      let idx = p.level;
      if (idx >= p.upgrades.length) {
        return null;
      }
      let upgrade = p.upgrades[idx];
      return {
        cost: upgrade.cost,
        effects: state.projects[p.id].upgrades[idx].effects,
      }
    },
    activeEffects(p) {
      let project = state.projects[p.id];
      if (p.status == 'Inactive') {
        return project.effects.concat(this.outcomeEffects(p));
      } else if (p.level === 0) {
        return project.effects;
      } else {
        return project.upgrades[p.level - 1].effects;
      }
    },
    outcomeEffects(p) {
      let project = state.projects[p.id];
      let allEffects = [];
      project.outcomes.forEach(({effects}) => {
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
    status(p) {
      return p.status.toLowerCase();
    },
    assignPoint(p) {
      if (state.points[type].available > 0) {
        game.setProjectPoints(p.id, p.points + 1);
        if (p.status !== 'Building') {
          game.startProject(p.id);
        }
        state.points[type].available--;
      }
    },
    unassignPoint(p) {
      if (p.points > 0) {
        game.setProjectPoints(p.id, p.points - 1);
        if (p.status == 'Building' && p.points <= 1) {
          game.stopProject(p.id);
        }
        state.points[type].available++;
      }
    },
    buyPoint() {
      let cost = this.nextPointCost;
      if (cost <= state.gameState.political_capital) {
        game.changePoliticalCapital(-cost);
        state.points[type].available++;
      }
    },
    payPoints(p) {
      // Only policies have points paid all at once,
      // rather than assigned.
      let available = state.gameState.political_capital;
      if (p.status == 'Inactive' && available >= p.cost) {
        game.changePoliticalCapital(-p.cost);
        game.startProject(p.id);
      }
    },
    upgrade(p) {
      let nextUpgrade = this.nextUpgrade(p);
      let available = state.gameState.political_capital;
      if (nextUpgrade && available >= nextUpgrade.cost) {
        game.changePoliticalCapital(-p.cost);
        game.upgradeProject(p.id);
      }
    }
  }
});
