const debug = {};
// TODO if (VERSION == 'dev') {
const params = new URLSearchParams(window.location.search);
params.forEach((v, k) => {
  debug[k] = v !== '' ? v : true;
});
// }

export default debug;
