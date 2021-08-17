<!--
  Renders child elements side-by-side with drag-to-scroll.
  Children should be wrapped in <li> tags.
-->

<template>
<ul class="cards"
  :class="{dragging:dragging}"
  @mousedown="startDrag"
  @touchstart="startDrag">
  <slot></slot>
</ul>
</template>

<script>

// For animating snap-to-center
// <https://gist.github.com/andjosh/6764939>
const duration = 150; // ms
function easeInOutQuad(t, b, c, d) {
  t /= d/2;
	if (t < 1) return c/2*t*t + b;
	t--;
	return -c/2 * (t*(t-2) - 1) + b;
};


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
    document.addEventListener('touchmove', this.drag);
    document.addEventListener('touchend', this.endDrag);
  },
  unmounted() {
    document.removeEventListener('mousemove', this.drag);
    document.removeEventListener('mouseup', this.endDrag);
    document.removeEventListener('touchmove', this.drag);
    document.removeEventListener('touchend', this.endDrag);
  },

  methods: {
    drag(ev) {
      if (!this.down) return;
      this.dragging = true;
      const dx = (ev.clientX || ev.touches[0].clientX) - this.pos.x;
      const dy = (ev.clientY || ev.touches[0].clientY) - this.pos.y;

      // Scroll the element
      this.$el.scrollTop = this.pos.top - dy;
      this.$el.scrollLeft = this.pos.left - dx;
    },
    startDrag(ev) {
      // Stop snap-to-center animation if there is one
      if (this.animation) cancelAnimationFrame(this.animation);

      this.down = true;
      this.pos = {
        // Current mouse position
        x: (ev.clientX || ev.touches[0].clientX),
        y: (ev.clientY || ev.touches[0].clientY),

        // Current scroll
        left: this.$el.scrollLeft,
        top: this.$el.scrollTop,
      };
    },
    endDrag(ev) {
      this.down = false;
      if (this.dragging) {
        this.dragging = false;

        // Horizontal snap-to-center
        let rect = this.$el.getBoundingClientRect();
        let scrollLeft = this.$el.scrollLeft;
        let centerOffset = scrollLeft + rect.width/2;

        // Find the child closest to the center
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
          // Animate snap-to-center
          let to = target.child.offsetLeft - rect.width/2 + target.width/2;
          let start = this.$el.scrollLeft,
            change = to - start,
            currentTime = performance.now();

          let update = (timestamp) => {
            this.$el.scrollLeft = easeInOutQuad(timestamp - currentTime, start, change, duration);
            if (timestamp - currentTime < duration) {
              this.animation = requestAnimationFrame(update);
            }
          };
          this.animation = requestAnimationFrame(update);
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
  white-space: normal;
}

/* Margin on either side so
the first and last cards have room
to be centered */
.cards li:first-child {
  margin-left: 12em;
}
.cards li:last-child {
  margin-right: 12em;
}
</style>
