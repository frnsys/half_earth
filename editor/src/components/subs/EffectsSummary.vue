<template>
<ul class="effects-summary">
  <li v-for="effect in localData" :key="effect.id" class="summary-pill" :class="flags(effect)">
    <div>{{label(effect)}}</div>
    <div v-if="entity(effect)" class="has-url"><a :href="entity(effect).url">{{entity(effect).name}}</a></div>
    <div v-if="params(effect)">{{params(effect)}}</div>
    <div v-if="hasEvent(effect)" :style="{background: '#c3c3c3', padding: '0.15em 0'}">
      <Tip :width="120">If this effect is part of an event, the event's variable values will be passed to this event.</Tip>
    </div>
  </li>
</ul>
</template>

<script>
import state from '../../state';
import consts from '../../consts';
import Tip from '../Tip.vue';

function formatParam(k, val) {
  switch (k) {
    case 'Change':
      return `${val > 0 ? '+' : ''}${val}`;
    case 'PercentChange':
      return `${val > 0 ? '+' : ''}${val}%`;
    case 'Delay (months)':
      return `${val} months`;
    default:
      return `${val}`;
  }
}

export default {
  props: ['effects'],
  components: {
    Tip
  },
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
    hasEvent(effect) {
      return effect.type == 'AddEvent' || effect.type == 'TriggerEvent';
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
    entity(effect) {
      let spec = consts.EFFECTS[effect.type];
      if (spec.entity) {
        if (effect.entity) {
          let items = this.itemsOfType(spec.entity);
          let match = items.find(el => el.id == effect.entity);
          return {
            url: `/?type=${match._type}#${match.id}`,
            name: match.name,
          };
        } else {
          return {
            url: '',
            name: '[MISSING]'
          };
        }
      }
      return null;
    },
    params(effect) {
      let spec = consts.EFFECTS[effect.type];
      let value = '';
      if (spec.params) {
        value += `${Object.keys(spec.params).map((k) => {
          let defined = effect.params[k] !== undefined && effect.params[k] !== '';
          if (!defined) return '[MISSING]';
          if (spec.params[k] == Number) {
            return formatParam(k, effect.params[k]);
          }
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
.effects-summary .tip-icon {
  color: #2e2a2a;
  border-color: #2e2a2a;
  background: #eee;
}
</style>
