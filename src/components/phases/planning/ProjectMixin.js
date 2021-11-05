import game from '/src/game';
import state from '/src/state';
import Cards from './Cards.vue';
import ProjectCard from 'components/cards/ProjectCard.vue';

export default (type) => ({
  components: {
    Cards,
    ProjectCard,
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
        return state.points[type.toLowerCase()];
      }
    },
    nextPointCost() {
      let i = this.pointsInUse;
      return Math.round((i+1)**1.5);
    },
  },
  methods: {
    buyPoint() {
      let cost = this.nextPointCost;
      if (cost <= state.gameState.political_capital) {
        game.changePoliticalCapital(-cost);
        state.points[type.toLowerCase()]++;
      }
    }
  }
});
