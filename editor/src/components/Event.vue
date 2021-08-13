<template>
<li class="event">
  <div>
    <label>
      Description
      <Tip>A 1-2 sentence description of the event. You can include variables (just capitalize them).</Tip>
    </label>
    <textarea v-model="localEvent.body" @blur="save" />
  </div>
  <div>
    <label>
      Variations (optional)
      <Tip>Variations on how the event can occur and the conditions they require. Some events, for example, may be less or more severe depending on past player actions.</Tip>
    </label>
    <textarea v-model="localEvent.description" @blur="save" />
  </div>
  <div>
    <label>
      Area
      <Tip>The area of the event--is this something global or does it happen in a specific location on the globe?</Tip>
    </label>
    <select v-model="localEvent.area" @change="save">
      <option v-for="t in EVENT_AREA" :value="t">{{t}}</option>
    </select>
  </div>
  <div v-if="localEvent.area == 'Local'">
    <label>
      Locations
      <Tip>What is the criteria for this event to occur in a given location? E.g. it has to be on a coast</Tip>
    </label>
    <textarea v-model="localEvent.locationConditions" placeholder="Location conditions" @blur="save" />
  </div>
  <div>
    <label>
      Conditions
      <Tip>Under what conditions the event is likely/becomes more likely to occur</Tip>
    </label>
    <textarea v-model="localEvent.conditions" placeholder="Conditions influencing event probability" @blur="save" />
  </div>
  <div>
    <label>
      Effects
      <Tip>What are the impacts of the event, just by occurring/before the player responds?</Tip>
    </label>
    <textarea v-model="localEvent.effects" placeholder="Impacts of the event" @blur="save" />
  </div>
  <div>
    <label>
      Responses (optional)
      <Tip>What can the player do to respond? What responses are available can be influenced by other decisions, like policies, resources, and projects.</Tip>
    </label>
    <textarea v-model="localEvent.responses" placeholder="Player responses" @blur="save" />
  </div>
  <div>
    <label>
      Story Arc (optional)
      <Tip>If the event is part of or triggers an arc, note the arc name here</Tip>
    </label>
    <input type="text" v-model="localEvent.arc" @blur="save" />
  </div>

</li>
</template>

<script>
import api from '../api';
import Tip from './Tip.vue';

export default {
  props: ['event'],
  components: {
    Tip
  },
  data() {
    return {
      localEvent: Object.assign({}, this.event)
    };
  },
  watch: {
    event(newEvent) {
      this.localEvent = Object.assign({}, newEvent);
    }
  },
  methods: {
    save() {
      api.update(this.localEvent);
    }
  }
}
</script>

