import state from './state';
import consts from './consts';

function itemsOfType(type) {
  return Object.values(state.items)
    .filter((i) => i._type == type)
    .sort((a, b) => a._created < b._created ? 1 : -1);
}

function requireAtLeastOne(val) {
  return val !== undefined && val.length > 0;
}

function requireOneOfChoice(val, choices) {
  return choices.includes(val);
}

function requireResources(resources) {
  let valid = Object.keys(consts.RESOURCES).filter((k) => {
    return resources[k] !== undefined && resources[k] !== '' && resources[k] > 0;
  });
  return valid.length > 0;
}

function validateBasic(item, key, required) {
  if (required.includes(key)) {
    let val = item[key];
    return val !== undefined &&
      ((typeof val === 'string' && val.length > 0)
        || (typeof val === 'number'));
  } else {
    return true;
  }
}

function validateProbabilities(probs) {
  return probs.every((prob) => {
    return requireAtLeastOne(prob.conditions) && validateConditions(prob.conditions);
  });
}

function validateConditions(conds) {
  return conds.every((cond) => {
    if (!Object.keys(consts.CONDITIONS).includes(cond.type)) {
      return false;
    }

    let valid = true;
    let spec = consts.CONDITIONS[cond.type];
    if (spec.choices && !spec.choices.includes(cond.subtype)) {
      valid = false;
    }

    if (spec.entity) {
      let items = itemsOfType(spec.entity);
      let validItem = items.some((item) => item.id == cond.entity);
      if (!validItem) valid = false;
    }

    if (spec.compare && (cond.value === undefined || cond.value === '')) {
      valid = false;
    }

    return valid;
  });
}

function validateEffects(effects) {
  return effects.every((effect) => {
      let valid = true;
      let spec = consts.EFFECTS[effect.type];
      if (spec.entity && effect.entity === undefined) {
        valid = false;
      }
      if (spec.choices && !spec.choices.includes(effect.subtype)) {
        valid = false;
      }
      if (spec.params) {
        if (Object.keys(spec.params).some((k) => effect.params[k] === undefined || effect.params[k] === '')) {
          valid = false;
        }
      }
      return valid;
  });
}

function validateVariables(variables) {
  let definedVariables =  Object.values(state.items)
    .filter((i) => i._type == 'Variable')
    .reduce((acc, v) => {
      acc[v.name] = (v.values || '').split('\n').filter((x) => x !== '');
      return acc;
    }, {});
  return (variables || []).every((name) => {
    let defined = Object.keys(definedVariables).includes(name);
    let values = definedVariables[name];
    return defined && values.length > 0;
  });
}

const SPECS = {
  Event: {
    key: 'name',
    required: ['name', 'effects', 'probabilities', 'description', 'variables'],
    questions: ['name', 'description', 'notes'],
    validateKey: (item, key) => {
      // TODO variables
      switch (key) {
        case 'name':
          return requireAtLeastOne(item.name);
        case 'description':
          return requireAtLeastOne(item.description);
        case 'effects':
          return requireAtLeastOne(item.effects) && validateEffects(item.effects);
        case 'probabilities':
          return requireAtLeastOne(item.probabilities) && validateProbabilities(item.probabilities);
        case 'variables':
          return validateVariables(item.variables);
        default:
          return true;
      }
    }
  },

  Project: {
    key: 'name',
    required: ['name', 'description', 'type', 'effects', 'construction'],
    questions: ['name', 'description', 'notes'],
    validateKey: (item, key) => {
      switch (key) {
        case 'name':
          return requireAtLeastOne(item.name);
        case 'description':
          return requireAtLeastOne(item.description);
        case 'construction':
          return requireAtLeastOne(item.construction) && requireResources(item.construction);
        case 'type':
          return requireOneOfChoice(item.type, ['Initiative', 'Policy', 'Research']);
        case 'effects':
          return requireAtLeastOne(item.effects) && validateEffects(item.effects);
        default:
          return true;
      }
    }
  },

  Process: {
    key: 'name',
    required: ['name', 'description', 'output'],
    questions: ['name', 'description', 'notes'],
    validateKey: (item, key) => {
      switch (key) {
        case 'name':
          return requireAtLeastOne(item.name);
        case 'description':
          return requireAtLeastOne(item.description);
        case 'output':
          return requireOneOfChoice(item.output, Object.keys(consts.OUTPUTS));
        default:
          return true;
      }
    }
  },

  Region: {
    key: 'name',
    required: ['name', 'satiety', 'safety', 'health', 'outlook'],
    questions: ['name', 'notes'],
    validateKey: (item, key) => {
      return validateBasic(item, key, SPECS.Region.required);
    }
  },

  Earth: {
    key: 'year',
    required: ['year', 'emissions', 'atmospheric_ghg', 'extinction_rate', 'temperature', 'ozone_damage'],
    questions: ['notes'],
    validateKey: (item, key) => {
      return validateBasic(item, key, SPECS.Earth.required);
    }
  },

  Flag: {
    key: 'name',
    required: ['name', 'desc'],
    questions: ['desc'],
    validateKey: (item, key) => {
      return validateBasic(item, key, SPECS.Flag.required);
    }
  },

  Variable: {
    key: 'name',
    required: ['name', 'values'],
    questions: ['name', 'values'],
    validateKey: (item, key) => {
      return validateBasic(item, key, SPECS.Variable.required);
    }
  }
}
export default SPECS;
