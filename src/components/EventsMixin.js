import regions from '../../assets/content/regions.json';
import Dialogue from './Dialogue.vue'
import Interstitial from './interstitials/Interstitial.vue';

export default {
  data() {
    return {
      events: [],
      event: null,
      eventIdx: null,
    };
  },
  components: {
    Dialogue,
    Interstitial,
  },
  mounted() {
    this.showEvent();
  },
  methods: {
    async loadEvent(id) {
      return await fetch(`/assets/content/events/${id}.json`)
        .then((resp) => resp.json());
    },
    nextEvent() {
      if (this.eventIdx < this.events.length - 1) {
        this.eventIdx++;
        this.showEvent();
      } else {
        this.event = null;
        this.eventIdx = null;
      }
    },
    showEvent() {
      if (this.eventIdx !== null) {
        let [eventId, regionId] = this.events[this.eventIdx];
        this.loadEvent(eventId).then((ev) => {
          this.event = ev;

          // Parse/fill in variables
          let vars = [...ev.text.matchAll('{([a-z]+)}')];
          let ctx = {};
          if (regionId !== undefined) {
            ctx['region'] = regions[regionId].name;
        }
          for (const match of vars) {
            ev.text = ev.text.replaceAll(match[0], ctx[match[1]]);
          }
        });
      }
    },
    selectChoice(idx) {
      // TODO skipping this until we figure out dialogue or swipe events
      /* let [eventId, regionId] = this.events[this.eventIdx]; */
      /* game.selectChoice(eventId, regionId, idx); */
    }
  }
};
