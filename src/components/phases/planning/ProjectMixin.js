import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
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
      }
    }
  }
};
