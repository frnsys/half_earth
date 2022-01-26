import consts from '/src/consts.json';

const intensities = {
  'land': {
    'energy': [0, 0.001, 0.01, 0.1],
    'calories': [0, 0.001, 0.002, 0.01],
  },
  'energy': {
    'energy': [0, 0.001, 0.01, 0.1], // TODO EROI
    'calories': [0, 0.00015, 0.0005, 0.001],
  },
  'water': {
    'energy': [0, 1, 2, 5],
    'calories': [0, 1, 2, 3],
  },
  'emissions': {
    'energy': [-2000, 0, 200, 800],
    'calories': [-1, 0, 0.5, 1],
  },
  'biodiversity': {
    'energy': [0, 1, 2, 3],
    'calories': [0, 1, 2, 3],
  },
};

function intensity(val, key, type) {
  let stops;
  if (key in intensities) {
    stops = intensities[key][type];
  } else {
    stops = consts.demand_levels.map((o) => Math.floor(o[key]));
  }
  for (let i = 0; i < stops.length - 1; i++) {
    if (val >= stops[i] && val < stops[i+1]) {
      return i+1;
    }
  }
  return stops.length;
}

function scale(val, key) {
  switch (key) {
    case 'outlook': return Math.round(val/consts.base_outlook * 4);
    case 'extinction': return Math.round(val/60 * 4);
    case 'habitability': return Math.round(val/consts.base_habitability * 4);
    case 'world_outlook': return Math.round(val/(consts.base_outlook+consts.base_world_outlook) * 4);
  }
}

function color(intensity, invert) {
  intensity = invert ? 5 - intensity : intensity;
  if (intensity <= 1) {
    return '#43CC70';
  } else if (intensity === 2) {
    return '#FBC011';
  } else if (intensity === 3) {
    return '#f28435';
  } else {
    return '#EF3838';
  }
}

function describe(intensity) {
  if (intensity === 0) {
    return 'Very Low';
  } else if (intensity === 1) {
    return 'Low';
  } else if (intensity === 2) {
    return 'Moderate';
  } else if (intensity === 3) {
    return 'High';
  } else {
    return 'Very High';
  }
}

export default {
  intensity, color, scale, describe
}
