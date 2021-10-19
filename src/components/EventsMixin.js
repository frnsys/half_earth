import game from '/src/game';
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
      if (this.eventIdx !== null && this.events.length > 0) {
        let [eventId, regionId] = this.events[this.eventIdx];
        console.log(`Loading event id: ${eventId}`);
        this.loadEvent(eventId).then((ev) => {
          console.log(ev);
          this.event = ev;

          // Set context variables
          if (this.event.dialogue) {
            let ctx = {};
            if (regionId !== undefined) {
              ctx['region'] = regions[regionId].name;
            };
            this.event.dialogue.context = ctx;
          } else {
            throw(`Event "${eventId}" missing dialogue!`);
          }

          // Apply event effects
          game.applyEvent(eventId, regionId);
        });
      } else {
        console.log('NO EVENTS');
      }
    },
    selectChoice(idx) {
      // TODO skipping this until we figure out dialogue or swipe events
      /* let [eventId, regionId] = this.events[this.eventIdx]; */
      /* game.selectChoice(eventId, regionId, idx); */
    }
  }
};
