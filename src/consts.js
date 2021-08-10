const PROJECT_STATE = {
  PLANNED: 1,
  CONSTRUCTING: 2,
  OPERATIONAL: 3,
  DESTRUCTING: 4,
};

const PROJECT_TYPE = {
  PROJECT: 1,
  RESEARCH: 2,
  POLICY: 3
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

export default {
  PROJECT_STATE,
  PROJECT_TYPE,
  VARI_ICONS,
  VARI_ABBREV
};

