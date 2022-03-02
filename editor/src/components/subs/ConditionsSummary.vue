<template>
<ul class="conditions-summary">
  <template v-for="(condition, i) in conditions" :key="condition.id">
    <li class="summary-pill" :class="flags(condition)">
      <div>{{label(condition)}}</div>
      <div>{{value(condition)}}</div>
    </li>
    <span class="condition-and" v-if="i < conditions.length - 1"> AND </span>
  </template>
</ul>
</template>

<script>
import state from '../../state';
import consts from '../../consts';

export default {
  props: ['conditions'],
  methods: {
    flags(condition) {
      let invalid = false;
      let spec = consts.CONDITIONS[condition.type];
      if (spec.entity && condition.entity === undefined) {
        invalid = true;
      }
      if (spec.choices && !spec.choices.includes(condition.subtype)) {
        invalid = true;
      }
      if (spec.compare) {
        if (condition.value === undefined || condition.value === '') {
          invalid = true;
        }
      }
      return {invalid};
    },
    label(condition) {
      let spec = consts.CONDITIONS[condition.type];
      let label = `${condition.type}`
      if (spec.choices) {
        label += `.${condition.subtype}`
      }
      return label;
    },
    value(condition) {
      let spec = consts.CONDITIONS[condition.type];
      let value = '';
      if (spec.entity) {
        let match = state.itemsByType[spec.entity][condition.entity];
        if (match) {
          value += `${match.name}`;
        }
      }
      if (spec.compare) {
        value += ` ${condition.comparator} ${(condition.value !== undefined && condition.value !== '') ? condition.value : '[MISSING]'}`;
      }
      if (spec.flag) {
        value += condition.value;
      }
      return value;
    }
  }
}
</script>

<style>
.conditions-summary .summary-pill {
  margin-right: 0;
}
.conditions-summary .summary-pill > div:first-child {
  background: #699DF4;
}
.condition-and {
  font-size: 0.7em;
}
</style>
