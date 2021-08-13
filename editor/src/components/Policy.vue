<template>
<li class="policy">
  <div>
    <label>
      Name
      <Tip>A name describing the policy.</Tip>
    </label>
    <input class="policy--name" type="text" v-model="localPolicy.name" @blur="save" />
  </div>
  <div>
    <label>
      Type
      <Tip>The type of policy. Some are sector-specific; "Projects" are things that are built/executed once and then finished; "Research" unlocks new policies or influences existing ones. Some policy areas might be locked initially (e.g. "Population")</Tip>
    </label>
    <select v-model="localPolicy.type" @change="save">
      <option v-for="t in POLICY_TYPE" :value="t">{{t}}</option>
    </select>
  </div>
  <div>
    <label>
      Description
      <Tip>A 1-2 sentence description of the policy.</Tip>
    </label>
    <textarea v-model="localPolicy.description" @blur="save" />
  </div>
  <div>
    <label>
      Requirements
      <Tip>What resources, land, etc are required for the policy's implementation and maintenance. For "Projects": what is required for its construction, operation, and destruction/decommissioning.</Tip>
    </label>
    <textarea v-model="localPolicy.requirements" placeholder="Resource, land, etc requirements" @blur="save" />
  </div>
  <div>
    <label>
      Effects
      <Tip>What are the impacts of the policy? This includes impacts on variables like emissions, unlocking or triggering events, unlocking new policies, adding/removing responses to events, etc.</Tip>
    </label>
    <textarea v-model="localPolicy.effects" placeholder="Impacts of the policy" @blur="save" />
  </div>
</li>
</template>

<script>
import api from '../api';
import util from '../util';
import Tip from './Tip.vue';

export default {
  props: ['policy'],
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
  data() {
    return {
      localPolicy: Object.assign({}, this.policy)
    };
  },
  watch: {
    policy(newPolicy) {
      this.localPolicy = Object.assign({}, newPolicy);
    }
  },
  methods: {
    save() {
      api.update(this.localPolicy);
    }
  }
}
</script>

<style>
.policy--name {
  font-size: 1.5em;
  border: none;
  border-bottom: 1px solid #000;
  font-weight: bold;
}
</style>
