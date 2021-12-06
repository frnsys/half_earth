import game from '/src/game';
import state from '/src/state';
import costs from 'lib/costs';
import Cards from './Cards.vue';
import ProjectCard from 'components/cards/ProjectCard.vue';

export default {
  components: {
    Cards,
    ProjectCard,
  },
  mounted() {
    let active = [];
    let inactive = [];

    let projects = state.gameState.projects.filter((p) => p.kind == this.type && !p.locked);
    projects.forEach((p, i) => {
      if (p.status == 'Building' || p.ongoing && p.status == 'Active') {
        active.push(i);
      } else {
        inactive.push(i);
      }
    });

    active.sort((a, b) => projects[a].name.toLowerCase().localeCompare(projects[b].name.toLowerCase()))
    inactive.sort((a, b) => projects[a].name.toLowerCase().localeCompare(projects[b].name.toLowerCase()))
    this.projectOrder = active.concat(inactive);
  },
  data() {
    return {
      state,
      projectOrder: [],
    };
  },
  computed: {
    projects() {
      return state.gameState.projects.filter((p) => p.kind == this.type && !p.locked);
    },
    pointsInUse() {
      let active = this.projects.filter((p) => p.status == 'Building' || p.ongoing && p.status == 'Active');
      let points = active.map((p) => p.points).reduce((acc, v) => acc + v, 0);
      return points + this.availablePoints;
    },
    availablePoints() {
      if (this.type == 'Policy') {
        return state.gameState.political_capital;
      } else {
        return state.points[this.type.toLowerCase()];
      }
    },
    nextPointCost() {
      return costs.nextPointCost(this.pointsInUse);
    },
  },
  methods: {
    buyPoint() {
      let cost = this.nextPointCost;
      if (cost <= state.gameState.political_capital) {
        game.changePoliticalCapital(-cost);
        state.points[this.type.toLowerCase()]++;
      }
    }
  }
};
