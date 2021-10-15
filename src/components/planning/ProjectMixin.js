import game from '../../game';
import state from '../../state';
import Cards from './Cards.vue';
import Card from './Card.vue';

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
  },
  methods: {
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
        if (p.status == 'Building') {
          game.stopProject(p.id);
        }
        state.points[type].available++;
      }
    },
    payPoints(p) {
      if (p.status == 'Inactive' && state.points[type].available >= p.cost) {
        state.points[type].available -= p.cost
        game.startProject(p.id);
      }
    }
  }
});
