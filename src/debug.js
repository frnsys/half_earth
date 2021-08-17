const params = new URLSearchParams(window.location.search);

export default {
  fps: params.get('fps') !== null,
  globe: params.get('globe') !== null,
};
