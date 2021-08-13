function resizeTextArea(el) {
  el.style.height = 0;
  el.style.height = `${el.scrollHeight + 4}px`;
}

export default {
  resizeTextArea
};
