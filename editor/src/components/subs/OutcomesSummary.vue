<template>
<ul class="outcomes-summary">
  <li v-for="(outcome, i) in localData" :key="outcome.id">
    <div class="outcome-text" :class="{invalid: outcome.text === undefined || outcome.text === ''}">{{ outcome.text || '[MISSING TEXT]' }}</div>
    <EffectsSummary v-if="outcome.effects" :effects="outcome.effects" />
    <div class="probability-type" v-if="i < localData.length - 1">{{ outcome.probability.type }}</div>
    <div class="probability-type" v-else>Default</div>
    <template v-if="outcome.probability.conditions.length > 0 ">
      <span> if </span>
      <ConditionsSummary :conditions="outcome.probability.conditions" />
    </template>
  </li>
</ul>
</template>

<script>
import EffectsSummary from './EffectsSummary.vue';
import ConditionsSummary from './ConditionsSummary.vue';

export default {
  props: ['outcomes'],
  components: {
    EffectsSummary,
    ConditionsSummary
  },
  data() {
    return {
      localData: this.outcomes || []
    };
  },
}
</script>

<style>
.outcomes-summary {
  display: flex;
  justify-content: space-between;
  margin-bottom: 1em;
}
.outcomes-summary > li {
	background: #eee;
	padding: 0.25em 0.5em;
	border: 1px solid #888;
	width: 32%;
}
.outcome-text.invalid {
  color: #F54242;
}
</style>
