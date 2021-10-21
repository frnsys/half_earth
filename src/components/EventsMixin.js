// Mixin for components that need to load and present events
import game from '/src/game';
import Dialogue from './Dialogue.vue'
import Scene from './scene/Scene.vue';
import regions from '/assets/content/regions.json';

export default {
  data() {
    return {
      events: [],
      event: null,
    };
  },
  components: {
    Scene,
    Dialogue,
  },
  computed: {
    hasEvent() {
      return this.events.length > 0;
    }
  },
  methods: {
    // Load frontend event data
    async loadEvent(id) {
      return await fetch(`/assets/content/events/${id}.json`)
        .then((resp) => resp.json());
    },
    nextEvent() {
      this.event = null;
      if (this.hasEvent) {
        this.showEvent();
      } else {
        if (this.afterEvents) this.afterEvents();
      }
    },
    showEvent() {
      if (this.hasEvent) {
        let [eventId, regionId] = this.events.shift();
        this.loadEvent(eventId).then((ev) => {
          this.event = ev;
          console.log(this.event);

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
