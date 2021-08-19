// <https://gist.github.com/andjosh/6764939>
function easeInOutQuad(t, b, c, d) {
  t /= d/2;
	if (t < 1) return c/2*t*t + b;
	t--;
	return -c/2 * (t*(t-2) - 1) + b;
};

// Duration in ms
// Can pass in arrays for start and end to animate
// multiple values at once.
function animate(start, end, duration, updateFn) {
  // Object to manage the animation
  let anim = {};

  if (!Array.isArray(start)) start = [start];
  if (!Array.isArray(end)) end = [end];

  // Setup and start the animation
  let changes = start.map((s, i) => end[i] - s);
  let startTime = performance.now();
  let update = (timestamp) => {
    let elapsed = timestamp - startTime;
    let vals = start.map((s, i) => easeInOutQuad(elapsed, s, changes[i], duration));
    updateFn(...vals);
    if (elapsed < duration) {
      anim.id = requestAnimationFrame(update);
    } else {
      updateFn(...end);
    }
  };
  anim.id = requestAnimationFrame(update);

  // Call this to cancel/stop the animation
  anim.stop = () => {
    cancelAnimationFrame(anim.id);
  };

  return anim;
}

export default animate;
