import game from '/src/game';
import state from '/src/state';
import Card from './Card.vue';
import Cards from './Cards.vue';
import {describeEffect} from '/src/effects';

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
      return Math.round((i+2)**1.5);
    },
  },
  methods: {
    effectDescs(p) {
      return p.effects
        .map((ev) => describeEffect(ev))
        .filter((desc) => desc !== undefined);
    },
    imageForProject(p) {
      let image = state.projects[p.id].image;
      if (image.fname) {
        return `/assets/content/images/${image.fname}`;
      } else {
        return '/assets/placeholders/project.png';
      }
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
    }
  }
});
