import consts from '../consts';

export default [{
  name: 'Mandated Veganism',
  type: consts.PROJECT_TYPE.PROJECT,
  popularity: -1,
  construction: {
    years: 1,
    resources: {},
    impacts: {
      contentedness: -3
    }
  },
  operation: {
    resources: {},
    impacts: {
      emissions: -2
    }
  },
  destruction: {
    years: 1,
    resources: {
      contentedness: 2,
      emissions: 3
    },
    impacts: {}
  }
}, {
  name: 'Energy Quotas',
  type: consts.PROJECT_TYPE.PROJECT,
  popularity: -1,
  construction: {
    years: 1,
    resources: {},
    impacts: {
      contentedness: -3
    }
  },
  operation: {
    resources: {},
    impacts: {
      emissions: -2
    }
  },
  destruction: {
    years: 1,
    resources: {},
    impacts: {
      contentedness: 2,
      emissions: 3
    }
  }
}, {
  name: 'Solar Radiation Management (Sulphur)',
  type: consts.PROJECT_TYPE.PROJECT,
  popularity: -1,
  construction: {
    years: 1,
    resources: {
      energy: -2
    },
    impacts: {
      contentedness: -1
    }
  },
  operation: {
    resources: {
      energy: -2
    },
    impacts: {
      temperature: -1
    }
  },
  destruction: {
    years: 1,
    resources: {
      contentedness: 2,
    },
    impacts: {
      temperature: 1
    }
  }

}, {
  name: 'Nuclear Power Plant',
  type: consts.PROJECT_TYPE.PROJECT,
  popularity: 1,
  construction: {
    years: 2,
    resources: {
      energy: -2
    },
    impacts: {}
  },
  operation: {
    resources: {
      energy: 3
    },
    impacts: {}
  },
  destruction: {
    years: 1,
    resources: {
      energy: -2
    },
    impacts: {}
  }
}, {
  name: 'Rewilding',
  type: consts.PROJECT_TYPE.PROJECT,
  popularity: 1,
  construction: {
    years: 2,
    resources: {
      energy: -1
    },
    impacts: {}
  },
  operation: {
    resources: {},
    impacts: {
      biodiversity: 2
    }
  },
  destruction: {
    years: 1,
    resources: {
      energy: -2
    },
    impacts: {
      biodiversity: -2
    }
  }
}, {
  name: 'BECCS',
  type: consts.PROJECT_TYPE.PROJECT,
  popularity: 1,
  construction: {
    years: 2,
    resources: {
      energy: -1
    },
    impacts: {
      biodiversity: -1
    }
  },
  operation: {
    resources: {
      energy: 2
    }
  },
  destruction: {
    years: 1,
    resources: {
      energy: -1
    },
    impacts: {
      biodiversity: 1
    }
  }
}, {
  name: 'Nuclear Fusion',
  type: consts.PROJECT_TYPE.RESEARCH,
  popularity: 0,
  construction: {
    years: 1,
    resources: {},
    impacts: {}
  }
}, {
  name: 'New SRM Method',
  type: consts.PROJECT_TYPE.RESEARCH,
  popularity: 0,
  construction: {
    years: 5,
    resources: {},
    impacts: {}
  }
}, {
  name: 'New Battery Technology',
  type: consts.PROJECT_TYPE.RESEARCH,
  popularity: 0,
  construction: {
    years: 5,
    resources: {},
    impacts: {}
  }
}, {
  name: 'Low-Carbon Concrete Production',
  type: consts.PROJECT_TYPE.RESEARCH,
  popularity: 0,
  construction: {
    years: 5,
    resources: {},
    impacts: {}
  }
}, {
  name: 'Perennial Cereals',
  type: consts.PROJECT_TYPE.RESEARCH,
  popularity: 0,
  construction: {
    years: 5,
    resources: {},
    impacts: {}
  }
}];
