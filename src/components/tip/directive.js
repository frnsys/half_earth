export default {
  mounted(el, {value}) {
    let text = typeof value == 'string' ? value : value.text;
    el.dataset.tip = text;
    el.dataset.card = value.card ? JSON.stringify(value.card) : '';
    el.addEventListener('click', (ev) => {
      ev.stopImmediatePropagation();

      // Get the tip component
      let tip = window.tip;
      tip.text = el.dataset.tip;

      // Assumed to remain unchanged
      tip.icon = value.icon;
      tip.subicon = value.subicon;
      tip.supicon = value.supicon;

      // Check if this tip is part of a
      // tip card; if so don't change the card.
      if (!el.closest('.tip--card') && el.dataset.card) {
        tip.card = JSON.parse(el.dataset.card);
      } else {
        tip.card = undefined;
      }

      tip.show = true;
    });
  },

  updated(el, {value}) {
    // Update tip text
    let text = typeof char == 'string' ? value : value.text;
    el.dataset.tip = text;

    // Update card data, if any
    if (value.card) {
      el.dataset.card = JSON.stringify(value.card);
    }
  }
};
