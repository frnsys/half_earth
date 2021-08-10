export default [{
  type: 'Natural Disaster',
  body: 'This is an event. Two sentences!',
  location: 238,
  selectedResponse: null,
  impacts: {
    contentedness: -1,
  },
  responses: [{
    name: 'Response A',
    costs: {
      energy: 1
    },
    impacts: {
      contentedness: -1,
    }
  }, {
    name: 'Response B',
    costs: {
      energy: 3
    },
    impacts: {
      contentedness: 1,
    }
  }, {
    name: 'Response C',
    costs: {
      energy: 12
    },
    impacts: {
      contentedness: 5,
    }
  }]
}, {
  type: 'Cultural Shift',
  body: 'This is another event. Another two sentences!',
  location: 351,
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
