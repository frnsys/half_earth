<template>
<ul class="conditions-summary">
  <template v-for="(condition, i) in localData" :key="condition.id">
    <li :class="flags(condition)">{{summarizeCondition(condition)}}</li>
    <template v-if="i < localData.length - 1"> AND </template>
  </template>
</ul>
</template>

<script>
import state from '../../state';
import consts from '../../consts';

export default {
  props: ['conditions'],
  data() {
    return {
      localData: this.conditions || []
    };
  },
  methods: {
    itemsOfType(type) {
      return Object.values(state.items)
        .filter((i) => i._type == type)
        .sort((a, b) => a._created < b._created ? 1 : -1);
    },
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
    summarizeCondition(condition) {
      let spec = consts.CONDITIONS[condition.type];
      let str = `${condition.type}`
      if (spec.choices) {
        str += `.${condition.subtype}`
      }
      if (spec.entity) {
        let items = this.itemsOfType(spec.entity);
        let match = items.find(el => el.id == condition.entity);
        str += `==${match.name}`;
      }
      if (spec.compare) {
        str += ` ${condition.comparator} ${(condition.value !== undefined && condition.value !== '') ? condition.value : '[MISSING]'}`;
      }
      return str;
    }
  }
}
</script>

<style>
.conditions-summary {
  font-size: 0.7em;
  margin: 0.5em 0 0 0;
}
.conditions-summary > li {
  display: inline-block;
  padding: 0.25em;
  border: 1px solid #374ab2;
  background: #699DF4;
  border-radius: 0.2em;
}
.conditions-summary > li.invalid {
  background: #D82828;
  border: 1px solid #613232;
  color: #fff;
  font-weight: bold;
}
.conditions-summary > li.invalid::before {
  content: '!';
  padding: 0 0.5em;
  font-weight: bold;
  color: #fff;
}

</style>
