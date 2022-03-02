// Mixin for components that need to load and present events
import game from '/src/game';
import Dialogue from './Dialogue.vue'
import Scene from './scene/Scene.vue';
import regions from '/assets/content/regions.json';
import EVENTS from '/assets/content/events.json';
import {clone} from 'lib/util';
import debug from '/src/debug';

export default {
  data() {
    return {
      events: [],
      event: null,
      showingEvent: false,
    };
  },
  components: {
    Scene,
    Dialogue,
  },
  computed: {
    hasEvent() {
      if (debug.hideIntro) {
        this.events = this.events.filter(([eventId, _]) => {
          return EVENTS[eventId].arc !== 'Tutorial';
        });
      }
      if (debug.hideEvents) {
        return false;
      } else {
        return this.events.length > 0;
      }
    },
    hasDialogue() {
      return this.event && this.event.dialogue;
    }
  },
  methods: {
    nextEvent() {
      this.event = null;
      if (this.hasEvent) {
        this.showEvent();
      } else {
        this.showingEvent = false;
        if (this.afterEvents) this.afterEvents();
      }
    },
    showEvent() {
      if (this.hasEvent) {
        let [eventId, regionId] = this.events.shift();

        // Clone the event so we don't modify the original
        this.event = clone(EVENTS[eventId]);

        // Set context variables
        if (this.event.dialogue) {
          let ctx = {};
          if (regionId !== undefined) {
            ctx['region'] = regions[regionId].name;
          };
          this.event.dialogue.context = ctx;
          this.event.eventId = eventId;
          this.event.regionId = regionId;
        } else {
          console.log(this.event);
          throw(`Event "${eventId}" missing dialogue!`);
        }

        // Apply event effects
        game.applyEvent(eventId, regionId);
        this.showingEvent = true;
      }
    },
  }
};
