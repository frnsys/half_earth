const PROJECT_STATE = {
  PLANNED: 1,
  CONSTRUCTING: 2,
  OPERATIONAL: 3,
  DESTRUCTING: 4,
};

const PLOT_TYPE = {
  FOREST: '🌳Forest',
  CLEARED: '🏞️Cleared Land',
  DEVELOPED: '🏘️Developed',
  WATER: '🌊Water'
};

const PLOT_ICONS = {
  temperature: '🌡️',
  fertility: '🌽',
  biodiversity: '🐬',
  population: '👥'
};

const VARI_ICONS = {
  biodiversity: '🐬',
  emissions: '☁️',
  contentedness: '😶',
  temperature: '🌡️',
};

const VARI_ABBREV = {
  temperature: 'Temp.',
  biodiversity: 'Biod.',
  contentedness: 'Cont.',
  emissions: 'Emis.',
};

const PLOT_ABBREV = {
  temperature: 'Temp.',
  fertility: 'Fert.',
  biodiversity: 'Biod.',
  population: 'Pop.',
};

export default {
  PROJECT_STATE,
  PLOT_TYPE,
  PLOT_ICONS,
  PLOT_ABBREV,
  VARI_ICONS,
  VARI_ABBREV
};
