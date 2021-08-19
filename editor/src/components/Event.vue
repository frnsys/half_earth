<template>
<li class="item" :id="event.id">
  <div class="indicators">
    <div class="indicator indicator--missing" v-if="invalid.length > 0">Missing data</div>
    <div class="indicator indicator--question" v-if="questions.length > 0">Question(s)</div>
  </div>
  <div>
    <label>
      Description
      <Tip>A 1-2 sentence description of the event. You can include variables (just capitalize them).</Tip>
    </label>
    <textarea class="title" v-model="localData.body" @blur="save" placeholder="Event description" :class="flags('body')"/>
  </div>
  <fieldset>
    <div>
      <label>
        Area
        <Tip>The area of the event--is this something global or does it happen in a specific location on the globe?</Tip>
      </label>
      <select v-model="localData.area" @change="save" :class="flags('area')">
        <option v-for="t in EVENT_AREA" :value="t">{{t}}</option>
      </select>
    </div>
    <div>
      <label>
        Story Arc (optional)
        <Tip>If the event is part of or triggers an arc, note the arc name here</Tip>
      </label>
      <input type="text" list="arcs" v-model="localData.arc" @blur="save" />
    </div>
  </fieldset>
  <div>
    <label>
      Variations (optional)
      <Tip>Variations on how the event can occur and the conditions they require. Some events, for example, may be less or more severe depending on past player actions.</Tip>
    </label>
    <textarea v-model="localData.variations" placeholder="Variations on the event" @blur="save" :class="flags('variations')"/>
  </div>
  <fieldset>
    <div>
      <label>
        Conditions
        <Tip>Under what conditions the event is likely/becomes more likely to occur. If this is a "Local" event, also detail the criteria for candidate locations (e.g. it has to be on a coast)</Tip>
      </label>
      <textarea v-model="localData.conditions" placeholder="Conditions influencing event probability" @blur="save" :class="flags('conditions')"/>
    </div>
    <div>
      <label>
        Effects
        <Tip>What are the impacts of the event, just by occurring/before the player responds?</Tip>
      </label>
      <textarea v-model="localData.effects" placeholder="Impacts of the event" @blur="save" :class="flags('effects')"/>
    </div>
  </fieldset>
  <div>
    <label>
      Responses (optional)
      <Tip>What can the player do to respond? What responses are available can be influenced by other decisions, like policies, resources, and projects.</Tip>
    </label>
    <textarea v-model="localData.responses" placeholder="Player responses" @blur="save" :class="flags('responses')"/>
  </div>
  <div>
    <label>
      Flavor Text/Dialogue (optional)
      <Tip>Possible dialogue or other flavor to text to accompany the event (e.g. an advisor introducing the event, giving suggestions on what to do, etc).</Tip>
    </label>
    <textarea v-model="localData.flavor" placeholder="Flavor text and dialogue" @blur="save" />
  </div>
  <div class="notes">
    <label @click="() => expandNotes = !expandNotes">
      <span><span class="notes-icon" v-if="localData.notes && localData.notes.length > 0">!</span> Notes, References, &amp; Discussion</span>
      <div class="notes--toggle">Toggle</div>
    </label>
    <textarea v-if="expandNotes" v-model="localData.notes" placeholder="Write any notes or discussion for others" @blur="save" :class="flags('notes')"/>
  </div>
</li>
</template>

<script>
import api from '../api';
import util from '../util';
import Tip from './Tip.vue';

export default {
  props: ['event'],
  data() {
    return {
      expandNotes: false,
      localData: Object.assign({}, this.event)
    };
  },
  components: {
    Tip
  },
  mounted() {
    this.$el.querySelectorAll('textarea').forEach((el) => {
      util.resizeTextArea(el);
      el.addEventListener('input', () => {
        util.resizeTextArea(el);
      });
    });
  },
  computed: {
    invalid() {
      return ['body', 'area', 'conditions', 'effects'].filter((k) => {
        let val = this.localData[k];
        return !(val && val.length > 0);
      });
    },
    questions() {
      return ['body', 'conditions', 'effects', 'variations', 'responses', 'notes'].filter((k) => {
        let val = this.localData[k];
        return val && val.includes('?');
      });
    }
  },
  watch: {
    event(newEvent) {
      this.localData = Object.assign({}, newEvent);
      this.$el.querySelectorAll('textarea').forEach((el) => {
        util.resizeTextArea(el);
      });
    }
  },
  methods: {
    save() {
      api.update(this.localData);
    },
    flags(key) {
      return {
        invalid: this.invalid.includes(key),
        question: this.questions.includes(key)
      }
    }
  }
}
</script>
