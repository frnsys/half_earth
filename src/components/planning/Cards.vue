<template>
<ul class="cards"
  :class="{dragging:dragging}"
  @mousedown="startDrag">
  <slot></slot>
</ul>
</template>

<script>
// <https://gist.github.com/andjosh/6764939>
function easeInOutQuad(t, b, c, d) {
  t /= d/2;
	if (t < 1) return c/2*t*t + b;
	t--;
	return -c/2 * (t*(t-2) - 1) + b;
};

const duration = 150; // ms
const increment = 20;
function scrollTo(el, to) {
  let start = el.scrollLeft,
    change = to - start,
    currentTime = 0;

  let animate = setInterval(() => {
    let val = easeInOutQuad(currentTime, start, change, duration);
    el.scrollLeft = val;
    currentTime += increment;
    if (currentTime >= duration) {
      clearInterval(animate)
    }
  }, increment);
  return animate;
}


export default {
  data() {
    return {
      down: false,
      dragging: false,
      pos: {
        x: 0,
        y: 0,
        top: 0,
        left: 0,
      }
    }
  },

  // Have to bind these on the document because
  // dragging likely brings the mouse out of the element's bounds
  mounted() {
    document.addEventListener('mousemove', this.drag);
    document.addEventListener('mouseup', this.endDrag);
  },
  unmounted() {
    document.removeEventListener('mousemove', this.drag);
    document.removeEventListener('mouseup', this.endDrag);
  },

  methods: {
    drag(ev) {
      if (!this.down) return;
      this.dragging = true;
      const dx = ev.clientX - this.pos.x;
      const dy = ev.clientY - this.pos.y;

      // Scroll the element
      this.$el.scrollTop = this.pos.top - dy;
      this.$el.scrollLeft = this.pos.left - dx;
    },
    startDrag(ev) {
      if (this.animation) clearInterval(this.animation);
      this.down = true;
      this.pos = {
        // Current mouse position
        x: ev.clientX,
        y: ev.clientY,

        // Current scroll
        left: this.$el.scrollLeft,
        top: this.$el.scrollTop,
      };
    },
    endDrag(ev) {
      this.down = false;
      if (this.dragging) {
        this.dragging = false;

        // Horizontal snapping
        let rect = this.$el.getBoundingClientRect();
        let centerOffset = this.$el.scrollLeft + rect.width/2;

        let target = [...this.$el.children].reduce((acc, child) => {
          let childRect = child.getBoundingClientRect();
          let childCenterOffset = child.offsetLeft + childRect.width/2;
          let offset = Math.abs(childCenterOffset - centerOffset);
          if (!acc || offset < acc.offset) {
            return {child, offset, width: childRect.width};
          } else {
            return acc;
          }
        }, null);
        if (target) {
          this.animation = scrollTo(this.$el, target.child.offsetLeft - rect.width/2 + target.width/2);
        }
      }
    }
  }
}
</script>

<style>
.cards {
  cursor: grab;
  white-space: nowrap;
  overflow: hidden;
  width: 100%;
  position: relative;
}
.cards.dragging {
  cursor: grabbing;
  user-select: none;

  /* Disable pointer events
  while dragging so we can't trigger
  children click events */
  pointer-events: none;
}
.cards li {
  display: inline-block;
  margin: 0 0.5em;
  vertical-align: top;
}
</style>
