export default {
  mounted(el, {value}) {
    let text = typeof value == 'string' ? value : value.text;
    el.dataset.tip = text;
    el.addEventListener('click', (ev) => {
      ev.stopImmediatePropagation();
      let tip = window.tip;
      tip.text = el.dataset.tip;
      tip.icon = value.icon;
      tip.card = value.card;
      tip.show = true;
    });
  },

  updated(el, {value}) {
    let text = typeof char == 'string' ? value : value.text;
    el.dataset.tip = text;
  }
};
