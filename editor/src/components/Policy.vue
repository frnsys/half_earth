<template>
<li class="item" :id="policy.id">
  <div class="indicators">
    <div class="indicator indicator--missing" v-if="invalid.length > 0">Missing data</div>
    <div class="indicator indicator--question" v-if="questions.length > 0">Question(s)</div>
  </div>
  <div>
    <label>
      Name
      <Tip>A name describing the policy.</Tip>
    </label>
    <input class="title" type="text" placeholder="Name of the policy/project/research" v-model="localData.name" @blur="save" :class="flags('name')" />
  </div>
  <div>
    <label>
      Type
      <Tip>The type of policy. Some are sector-specific; "Projects" are things that are built/executed once and then finished; "Research" unlocks new policies or influences existing ones. Some policy areas might be locked initially (e.g. "Population")</Tip>
    </label>
    <select v-model="localData.type" @change="save" :class="flags('type')">
      <option v-for="t in POLICY_TYPE" :value="t">{{t}}</option>
    </select>
  </div>
  <div>
    <label>
      Description
      <Tip>A 1-2 sentence description of the policy.</Tip>
    </label>
    <textarea v-model="localData.description" placeholder="A brief description" @blur="save" :class="flags('description')"/>
  </div>
  <div>
    <label>
      Requirements
      <Tip>What resources, land, etc are required for the policy's implementation and maintenance. For "Projects": what is required for its construction, operation, and destruction/decommissioning.</Tip>
    </label>
    <textarea v-model="localData.requirements" placeholder="Resource, land, etc requirements" @blur="save" :class="flags('requirements')"/>
  </div>
  <div>
    <label>
      Effects
      <Tip>What are the impacts of the policy? This includes impacts on variables like emissions, unlocking or triggering events, unlocking new policies, adding/removing responses to events, etc.</Tip>
    </label>
    <textarea v-model="localData.effects" placeholder="Impacts of the policy" @blur="save"  :class="flags('effects')"/>
  </div>
  <div>
    <label>
      Flavor Text/Dialogue (optional)
      <Tip>Possible dialogue or other flavor to text to accompany the policy (e.g.an advisor giving their take on the policy when you first select/unlock it).</Tip>
    </label>
    <textarea v-model="localData.flavor" placeholder="Flavor text and dialogue" @blur="save" />
  </div>
  <div class="notes">
    <label @click="() => expandNotes = !expandNotes">
      <span>
        <span class="notes-icon question" v-if="flags('notes').question">?</span>
        <span class="notes-icon" v-if="localData.notes && localData.notes.length > 0">!</span> Notes, References, &amp; Discussion</span>
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
  props: ['policy'],
  data() {
    return {
      expandNotes: false,
      localData: Object.assign({}, this.policy)
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
      return ['name', 'type', 'description', 'requirements', 'effects'].filter((k) => {
        let val = this.localData[k];
        return !(val && val.length > 0);
      });
    },
    questions() {
      return ['name', 'description', 'requirements', 'effects', 'notes'].filter((k) => {
        let val = this.localData[k];
        return val && val.includes('?');
      });
    }
  },
  watch: {
    policy(newPolicy) {
      this.localData = Object.assign({}, newPolicy);
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
