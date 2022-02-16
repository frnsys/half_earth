function resizeTextArea(el) {
  let scrollLeft = window.pageXOffset || (document.documentElement || document.body.parentNode || document.body).scrollLeft;
  let scrollTop  = window.pageYOffset || (document.documentElement || document.body.parentNode || document.body).scrollTop;
  el.style.height = 0;
  el.style.height = `${el.scrollHeight + 4}px`;
  window.scrollTo(scrollLeft, scrollTop);
}

export default {
  resizeTextArea
};
