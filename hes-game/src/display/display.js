import t from '/src/i18n';
import icons from 'components/icons';

const DISPLAY_NAMES = {
    'Fuel': 'fuel',
    'Electricity': 'electricity',
    'PlantCalories': 'plant-based food',
    'AnimalCalories': 'animal-based food',
    'Land': 'land',
};

const FEATURE_DESCS = {
  'IsSolar': 'solar processes',
  'IsIntermittent': 'intermittent processes',
  'CanMeltdown': 'processes that may meltdown',
  'MakesNuclearWaste': 'processes that produce nuclear waste',
  'IsLaborIntensive': 'especially labor-intensive processes',
  'IsCombustion': 'combustion processes',
  'IsFossil': 'fossil fuel processes',
  'UsesOil': 'oil processes',
  'IsCCS': 'carbon capture processes',
  'UsesLivestock': 'processes that use livestock',
  'UsesPesticides': 'processes that use pesticides',
  'UsesSynFertilizer': 'processes that use synthetic fertilizers',
};

function fillIcons(text) {
  let matches = [...text.matchAll(/\[([a-z_]+)\]/g)];
  for (const match of matches) {
    text = text.replaceAll(match[0], `<img src="${icons[match[1]]}">`);
  }
  return text;
}

function fillVars(text, context) {
  let vars = [...text.matchAll('{([a-z_]+)}')];
  for (const match of vars) {
    text = text.replaceAll(match[0], context[match[1]]);
  }
  return text;
}

function cardTag(name, icon) {
  if (icon) {
    return `<div class="card-tag"><img src="${icons[icon]}">${name}</div>`
  } else {
    return `<div class="card-tag">${name}</div>`
  }
}

function enumKey(v) {
  return v.split(/(?=[A-Z])/).join('_').toLowerCase();
}

function enumDisplay(v, hyphen) {
  let char = hyphen ? '-' : ' ';
  let text = enumKey(v).replace('_', char);
  return t(text);
}

function relationshipName(relationship) {
  if (relationship >= 5) {
    return 'Ally';
  } else if (relationship >= 4) {
    return 'Friendly';
  } else if (relationship <= 1) {
    return 'Nemesis';
  } else {
    return 'Neutral';
  }
}

function displayName(key) {
  return t(DISPLAY_NAMES[key]);
}

function describeFeature(feature) {
  return t(FEATURE_DESCS[feature]);
}

export default {
  cardTag, describeFeature,
  fillIcons, fillVars,
  displayName, enumKey, enumDisplay,
  relationshipName}
