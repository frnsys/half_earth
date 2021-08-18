// <https://gist.github.com/andjosh/6764939>
function easeInOutQuad(t, b, c, d) {
  t /= d/2;
	if (t < 1) return c/2*t*t + b;
	t--;
	return -c/2 * (t*(t-2) - 1) + b;
};

// Duration in ms
function animate(start, end, duration, updateFn) {
  // Object to manage the animation
  let anim = {};

  // Setup and start the animation
  let change = end - start;
  let startTime = performance.now();
  let update = (timestamp) => {
    let val = easeInOutQuad(timestamp - startTime, start, change, duration);
    updateFn(val);
    if (timestamp - startTime < duration) {
      anim.id = requestAnimationFrame(update);
    } else {
      updateFn(end);
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
