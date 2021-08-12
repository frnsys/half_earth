export default [{
  type: 'Natural Disaster',
  body: 'A major hurricane will make landfall soon in the Southeastern US.',
  location: 238,
  selectedResponse: null,
  impacts: {
    contentedness: -1,
  },
  responses: [{
    name: 'Evacuate the region',
    costs: {
      energy: 1
    },
    impacts: {
      contentedness: -1,
    }
  }, {
    name: 'Accelerate seawall construction',
    costs: {
      energy: 3
    },
    impacts: {
      contentedness: 1,
    }
  }, {
    name: 'Ignore',
    costs: {},
    impacts: {
      contentedness: -3,
    }
  }]
}, {
  type: 'Cultural Shift',
  body: 'Everyone is really into hemp clothing now.',
  location: 351,
  selectedResponse: null,
  impacts: {
    carbon: -1,
  },
  responses: [{
    name: 'Great',
    costs: {
      energy: 10
    },
    impacts: {}
  }, {
    name: 'Grow more hemp',
    costs: {},
    impacts: {}
  }]
}, {
  type: 'Planetary Crisis',
  body: 'Emergency measures placeholder',
  location: 191,
  selectedResponse: null,
  impacts: {
    carbon: -1,
  },
  responses: [{
    name: 'Response A',
    costs: {
      energy: 10
    },
    impacts: {}
  }, {
    name: 'Response B',
    costs: {},
    impacts: {}
  }, {
    name: 'Response C',
    costs: {},
    impacts: {}
  }]
}];
