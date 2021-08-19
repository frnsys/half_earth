const params = new URLSearchParams(window.location.search);

const debug = {};
params.forEach((v, k) => {
  debug[k] = v !== '' ? v : true;
});

export default debug;
