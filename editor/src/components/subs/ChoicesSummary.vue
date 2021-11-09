<template>
<div class="choices-summary">
  <div :class="flags(choice)" class="choice-summary" v-for="(choice, i) in localData">
    <template v-if="choice.text">
      <div class="choice-type" v-if="choice.type && choice.type !== 'none'">{{ choice.type }}</div>
      <span class="choice-text">[Player]: "{{ choice.text }}"</span>
      <template v-if="choice.effects && choice.effects.length > 0">
        <EffectsSummary :effects="choice.effects" />
      </template>
      <div class="choice-conditions" v-if="choice.conditions && choice.conditions.length > 0">
        <span>Available if:</span> <ConditionsSummary :conditions="choice.conditions" />
      </div>
      <DialogueSummary :dialogue="choice.dialogue" />
    </template>
    <div class="missing-defined" v-else>No Choice Defined</div>
  </div>
</div>
</template>

<script>
import EffectsSummary from './EffectsSummary.vue';
import ConditionsSummary from './ConditionsSummary.vue';

export default {
  props: ['choices'],
  components: {
    EffectsSummary, ConditionsSummary
  },
  beforeCreate: function () {
    // Hack around circular references
    this.$options.components.DialogueSummary = require('./DialogueSummary.vue').default
  },
  data() {
    return {
      localData: this.choices || []
    };
  },
  methods: {
    flags(choice) {
      let invalid = choice.text === undefined || choice.text === '';
      return {invalid};
    },
  }
}
</script>

<style>
.choice-text {
  margin: 2em 0 1em;
  font-weight: bold;
}
.choice-summary {
  position: relative;
  border: 1px solid #888;
  margin-bottom: 0.2em;
  padding: 0.25em;
  margin: 0.25em 0;
  border-radius: 0.2em;
}
.choice-summary.invalid {
  align-items: center;
  background: #ffabab;
}
.choice-summary .conditions-summary {
  display: inline-block;
}
.choice-summary .effects-summary {
  display: inline-block;
  margin-left: 0.5em;
}
.choice-conditions span {
  font-size: 0.8em;
}
.choice-type {
	text-transform: uppercase;
	position: absolute;
	right: 50%;
	font-size: 0.6em;
  background: #ffc9b3;
	padding: 0.2em 0.4em;
	border-radius: 0.2em;
	border: 1px solid #000;
	transform: translate(50%, -50%);
}
.choice-summary .missing-defined {
  display: block;
  margin: 0 auto;
}
</style>
