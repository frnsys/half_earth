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

function requireNonEmptyObj(val) {
  return val !== undefined && Object.keys(val).length > 0;
}

function requireOneOfChoice(val, choices) {
  return choices.includes(val);
}

function requirePositive(val) {
  return val !== undefined && val !== '' && val > 0.;
}

function requirePositiveInclZero(val) {
  return val !== undefined && val !== '' && val >= 0.;
}

function requireResources(resources) {
  let valid = Object.keys(consts.RESOURCES).filter((k) => {
    return resources[k] !== undefined && resources[k] !== '' && resources[k] > 0;
  });
  return valid.length > 0;
}

function _itemEffects() {
  return Object.values(state.items).flatMap((item) => {
    return item.effects || [];
  });
}

// Check if something unlocks this item
function hasUnlocker(item) {
  switch (item._type) {
    case 'Event': {
      return _itemEffects().some((effect) => {
        let validType = effect.type == 'AddEvent' || effect.type == 'TriggerEvent';
        return validType && effect.entity === item.id;
      });
    }
    case 'Project': {
      return _itemEffects().some((effect) => {
        let validType = effect.type == 'UnlocksProject';
        return validType && effect.entity === item.id;
      });
    }
    case 'Process': {
      return _itemEffects().some((effect) => {
        let validType = effect.type == 'UnlocksProcess';
        return validType && effect.entity === item.id;
      });
    }
    default: {
      return true
    }
  }
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
    return prob.conditions.length === 0 || (requireAtLeastOne(prob.conditions) && validateConditions(prob.conditions));
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

function validateOutcomes(outcomes) {
  return outcomes.every((outcome) => {
    return requireAtLeastOne(outcome.text) && validateEffects(outcome.effects) && validateProbabilities([outcome.probability]);
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
    validate: ['name', 'effects', 'probabilities', 'description', 'variables', 'locked'],
    questions: ['notes'],
    validateKey: (item, key) => {
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
        case 'locked':
          if (item.locked === undefined) item.locked = false;
          return item.locked == hasUnlocker(item);
        default:
          return true;
      }
    }
  },

  Project: {
    key: 'name',
    validate: ['name', 'description', 'type', 'effects', 'construction', 'years', 'locked', 'outcomes'],
    questions: ['name', 'description', 'notes'],
    validateKey: (item, key) => {
      switch (key) {
        case 'name':
          return requireAtLeastOne(item.name);
        case 'description':
          return requireAtLeastOne(item.description);
        case 'construction':
          return requireNonEmptyObj(item.construction) && requireResources(item.construction);
        case 'type':
          return requireOneOfChoice(item.type, ['Initiative', 'Policy', 'Research']);
        case 'effects':
          return requireAtLeastOne(item.effects) && validateEffects(item.effects);
        case 'years':
          return requirePositive(item.years);
        case 'outcomes':
          return validateOutcomes(item.outcomes);
        case 'locked':
          if (item.locked === undefined) item.locked = false;
          return item.locked == hasUnlocker(item);
        default:
          return true;
      }
    }
  },

  Process: {
    key: 'name',
    validate: ['name', 'description', 'output', 'mix_share', 'locked'],
    questions: ['name', 'description', 'notes'],
    validateKey: (item, key) => {
      switch (key) {
        case 'name':
          return requireAtLeastOne(item.name);
        case 'description':
          return requireAtLeastOne(item.description);
        case 'output':
          return requireOneOfChoice(item.output, Object.keys(consts.OUTPUTS));
        case 'mix_share':
          return requirePositiveInclZero(item.mix_share);
        case 'locked':
          if (item.locked === undefined) item.locked = false;
          return item.locked == hasUnlocker(item);
        default:
          return true;
      }
    }
  },

  Region: {
    key: 'name',
    validate: ['name', 'safety', 'health', 'outlook'],
    questions: ['name', 'notes'],
    validateKey: (item, key) => {
      return validateBasic(item, key, SPECS.Region.validate);
    }
  },

  Earth: {
    key: 'year',
    validate: ['year', 'co2_emissions', 'ch4_emissions', 'n2o_emissions', 'extinction_rate', 'temperature'],
    questions: ['notes'],
    validateKey: (item, key) => {
      return validateBasic(item, key, SPECS.Earth.validate);
    }
  },

  Flag: {
    key: 'name',
    validate: ['name', 'desc'],
    questions: ['desc'],
    validateKey: (item, key) => {
      return validateBasic(item, key, SPECS.Flag.validate);
    }
  },

  Variable: {
    key: 'name',
    validate: ['name', 'values'],
    questions: ['name', 'values'],
    validateKey: (item, key) => {
      return validateBasic(item, key, SPECS.Variable.validate);
    }
  }
}
export default SPECS;
