<template>
<ul class="effects-summary">
  <li v-for="effect in localData" :key="effect.id" class="effect-summary" :class="flags(effect)">
    {{summarizeEffect(effect)}}
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
    summarizeEffect(effect) {
      let spec = consts.EFFECTS[effect.type];
      let str = `${effect.type}`
      if (spec.choices) {
        str += `.${effect.subtype}`
      }
      if (spec.entity) {
        let items = this.itemsOfType(spec.entity);
        let match = items.find(el => el.id == effect.entity);
        str += `:${match.name}`;
      }
      if (spec.params) {
        str += `:${Object.keys(spec.params).map((k) => {
          return (effect.params[k] !== undefined && effect.params[k] !== '') ? effect.params[k] : '[MISSING]';
        }).join(',')}`;
      }
      return str;
    }
  }
}
</script>

<style>
.effects-summary .effect-summary {
  font-size: 0.7em;
  display: inline-block !important;
  padding: 0.25em;
  background: #DEEF8D;
  border-radius: 0.2em;
  border: 1px solid #979869;
  margin: 0.1em 0.5em 0.1em 0 !important;
}
.effects-summary .effect-summary.invalid {
  background: #D82828;
  border: 1px solid #613232;
  color: #fff;
  font-weight: bold;
}
.effects-summary .effect-summary.invalid::before {
  content: '!';
  padding: 0 0.5em;
  font-weight: bold;
  color: #fff;
}
</style>
