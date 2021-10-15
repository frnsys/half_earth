/*
 * Event conditions:
 * - state value >=< comparison
 * - state all,none,some
 * - arcs
*/

function sampleEvents(state, events) {
  let sampled = events.filter((ev) => {
    return Math.random() < ev.probFn(state);
  }).map((ev) => {
    // Copy the event template
    let event = JSON.parse(JSON.stringify(ev));
    event.location = 128; // TODO determine where the event occurs
    event.selectedResponse = null;
    return event;
  });
}
