<template>
<ul class="probabilities-summary">
  <li v-for="probability in probabilities" :key="probability.id">
    <div class="probability-type">{{ probability.type }}</div>
    <template v-if="probability.conditions.length > 0 ">
      <span> if </span>
      <ConditionsSummary :conditions="probability.conditions" />
    </template>
  </li>
</ul>
</template>

<script>
import ConditionsSummary from './ConditionsSummary.vue';

export default {
  props: ['probabilities'],
  components: {
    ConditionsSummary
  },
}
</script>

<style>
.probabilities-summary .conditions-summary {
  margin: 0 0 0 1em;
}
.probabilities-summary > li {
  display: block !important;
	margin: 0 !important;
	padding: 0 !important;
	background: 0 !important;
	border: 0 !important;
}
.probability-type {
  font-size: 0.7em;
  display: inline-block;
  padding: 0.25em;
  border: 1px solid #856363;
  background: #FABA93;
  border-radius: 0.2em;
  width: 70px;
}
</style>
