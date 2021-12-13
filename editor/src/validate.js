import state from './state';
import consts from './consts';

function requireAtLeastOne(val) {
  return val !== undefined && val !== null && val.length > 0;
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
    return resources[k] !== undefined && resources[k] !== '' && resources[k] !== 0;
  });
  return valid.length > 0;
}

function requireByproducts(byproducts) {
  let valid = Object.keys(consts.BYPRODUCTS).filter((k) => {
    return byproducts[k] !== undefined && byproducts[k] !== '' && byproducts[k] !== 0;
  });
  return valid.length > 0;
}

// Get all effects associated with an item
function _itemEffects() {
  return Object.values(state.items)
    .filter((i) => !i.deleted)
    .flatMap((item) => {
      let effects = item.effects || [];
      if (item.outcomes) {
        item.outcomes.forEach((o) => {
          effects = effects.concat(o.effects || []);
        });
      }
      return effects;
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
    case 'NPC': {
      return _itemEffects().some((effect) => {
        let validType = effect.type == 'UnlocksNPC';
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
      let items = Object.values(state.itemsByType[spec.entity]);
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
    if (effect === undefined) return false;
    let valid = true;
    let spec = consts.EFFECTS[effect.type];
    if (spec === undefined) return false;
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
  return outcomes.every((outcome, i) => {
    // First outcome doesn't need text
    return i == 0 || (requireAtLeastOne(outcome.text) && validateEffects(outcome.effects) && validateProbabilities([outcome.probability]));
  });
}

function validateVariables(variables) {
  let definedVariables =  Object.values(state.items)
    .filter((i) => i._type == 'Variable')
    .filter((i) => !i.deleted)
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

function validateDialogue(dialogue) {
  return dialogue !== undefined && dialogue.lines && dialogue.lines.length > 0 && dialogue.lines.every((l) => {
    return requireOneOfChoice(l.speaker, consts.SPEAKERS) && requireAtLeastOne(l.text);
  });
}

const SPECS = {
  Event: {
    key: 'name',
    validate: ['name', 'probabilities', 'variables', 'locked', 'type', 'subphase', 'dialogue'],
    questions: ['notes'],
    validateKey: (item, key) => {
      switch (key) {
        case 'name':
          return requireAtLeastOne(item.name);
        case 'type':
          return requireOneOfChoice(item.type, consts.EVENT_TYPES);
        case 'dialogue':
          if (item.type == 'Icon') {
            return true
          } else {
            return validateDialogue(item.dialogue);
          }
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
    validate: ['name', 'description', 'type', 'effects', 'cost', 'locked', 'outcomes'],
    questions: ['name', 'description', 'notes'],
    validateKey: (item, key) => {
      switch (key) {
        case 'name':
          return requireAtLeastOne(item.name);
        case 'description':
          return requireAtLeastOne(item.description);
        case 'type':
          return requireOneOfChoice(item.type, ['Initiative', 'Policy', 'Research']);
        case 'effects':
          return (item.outcomes && item.outcomes.length == 1) ? requireAtLeastOne(item.effects) && validateEffects(item.effects) : true;
        case 'cost':
          return requirePositive(item.cost);
        case 'outcomes':
          return validateOutcomes(item.outcomes || []);
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
    validate: ['name', 'output', 'mix_share', 'locked', 'feedstock', 'feedstock_amount', 'resources', 'byproducts'],
    questions: ['name', 'notes'],
    validateKey: (item, key) => {
      switch (key) {
        case 'name':
          return requireAtLeastOne(item.name);
        case 'output':
          return requireOneOfChoice(item.output, Object.keys(consts.OUTPUTS));
        case 'feedstock':
          return requireOneOfChoice(item.feedstock, Object.keys(consts.FEEDSTOCKS));
        case 'feedstock_amount':
          return requirePositiveInclZero(item.feedstock_amount);
        case 'mix_share':
          return requirePositiveInclZero(item.mix_share);
        case 'byproducts':
          return requireNonEmptyObj(item.byproducts) && requireByproducts(item.byproducts);
        case 'resources':
          return requireNonEmptyObj(item.resources) && requireResources(item.resources);
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
    validate: ['name', 'health', 'outlook', 'income_level', 'population'],
    questions: ['name', 'notes'],
    validateKey: (item, key) => {
      switch (key) {
        case 'income_level':
          return requireOneOfChoice(item.income_level, consts.INCOME_LEVELS);
        default:
          return validateBasic(item, key, SPECS.Region.validate);
      }
    }
  },

  World: {
    key: 'year',
    validate: ['year', 'co2_emissions', 'ch4_emissions', 'n2o_emissions', 'extinction_rate', 'temperature', 'sea_level_rise'],
    questions: ['notes'],
    validateKey: (item, key) => {
      return validateBasic(item, key, SPECS.World.validate);
    }
  },

  Variable: {
    key: 'name',
    validate: ['name', 'values'],
    questions: ['name', 'values'],
    validateKey: (item, key) => {
      return validateBasic(item, key, SPECS.Variable.validate);
    }
  },

  Const: {
    key: 'name',
    validate: ['name', 'value'],
    questions: ['name', 'value'],
    validateKey: (item, key) => {
      return validateBasic(item, key, SPECS.Const.validate);
    }
  },

  Industry: {
    key: 'name',
    validate: ['name', 'resources'],
    questions: [],
    validateKey: (item, key) => {
      switch (key) {
        case 'name':
          return requireAtLeastOne(item.name);
        case 'resources':
          return requireNonEmptyObj(item.resources) && requireResources(item.resources);
        default:
          return true;
      }
    }
  },

  NPC: {
    key: 'name',
    validate: ['name', 'description', 'locked'],
    questions: ['name', 'description', 'notes'],
    validateKey: (item, key) => {
      switch (key) {
        case 'name':
          return requireAtLeastOne(item.name);
        case 'description':
          return requireAtLeastOne(item.description);
        case 'locked':
          if (item.locked === undefined) item.locked = false;
          return item.locked == hasUnlocker(item);
        default:
          return true;
      }
    }
  },

}
export default SPECS;
