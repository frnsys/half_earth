<template>
<ul class="effects-summary">
  <li v-for="effect in localData" :key="effect.id" class="summary-pill" :class="flags(effect)">
    <div>{{label(effect)}}</div>
    <div>{{value(effect)}}</div>
  </li>
</ul>
</template>

<script>
import state from '../../state';
import consts from '../../consts';

export default {
  props: ['effects'],
  data() {
    return {
      localData: this.effects || []
    };
  },
  methods: {
    itemsOfType(type) {
      return Object.values(state.items)
        .filter((i) => i._type == type)
        .sort((a, b) => a._created < b._created ? 1 : -1);
    },
    flags(effect) {
      let invalid = false;
      let spec = consts.EFFECTS[effect.type];
      if (spec.entity && effect.entity === undefined) {
        invalid = true;
      }
      if (spec.choices && !spec.choices.includes(effect.subtype)) {
        invalid = true;
      }
      if (spec.params) {
        if (Object.keys(spec.params).some((k) => effect.params[k] === undefined || effect.params[k] === '')) {
          invalid = true;
        }
      }
      return {invalid};
    },
    label(effect) {
      let spec = consts.EFFECTS[effect.type];
      let label = `${effect.type}`
      if (spec.choices) {
        label += `.${effect.subtype}`
      }
      return label;
    },
    value(effect) {
      let spec = consts.EFFECTS[effect.type];
      let value = '';
      if (spec.entity) {
        let items = this.itemsOfType(spec.entity);
        let match = items.find(el => el.id == effect.entity);
        value += `${match.name}`;
      }
      if (spec.params) {
        value += `${Object.keys(spec.params).map((k) => {
          return (effect.params[k] !== undefined && effect.params[k] !== '') ? `${effect.params[k] > 0 ? '+' : ''}${effect.params[k]}` : '[MISSING]';
        }).join(',')}`;
      }
      return value;
    }
  }
}
</script>

<style>
.effects-summary .summary-pill > div:first-child {
  background: #DEEF8D;
}
</style>
