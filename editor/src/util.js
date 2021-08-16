function resizeTextArea(el) {
  el.style.height = 0;
  el.style.height = `${el.scrollHeight + 4}px`;
}

// <https://gist.github.com/mathewbyrne/1280286>
function slugify(text) {
  return text.toString().toLowerCase()
    .replace(/\s+/g, '-')           // Replace spaces with -
    .replace(/[^\w\-]+/g, '')       // Remove all non-word chars
    .replace(/\-\-+/g, '-')         // Replace multiple - with single -
    .replace(/^-+/, '')             // Trim - from start of text
    .replace(/-+$/, '');            // Trim - from end of text
}

export default {
  slugify,
  resizeTextArea
};
