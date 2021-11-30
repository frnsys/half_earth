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
import animate from 'lib/anim';

// For animating snap-to-center
const duration = 150; // ms

export default {
  data() {
    return {
      down: false,
      dragging: false,
      vel: 0,
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
    this.snapToCenter(false);
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
      const dx = (ev.clientX !== undefined ? ev.clientX : ev.touches[0].clientX) - this.pos.x;

      // Don't allow vertical dragging
      // const dy = (ev.clientY !== undefined ? ev.clientY : ev.touches[0].clientY) - this.pos.y;
      const dy = 0;

      // Scroll the element
      let prev = this.$el.scrollLeft;
      this.$el.scrollTop = this.pos.top - dy;
      this.$el.scrollLeft = this.pos.left - dx;
      let diff = this.$el.scrollLeft - prev; // For momentum

      this.vel = diff;
    },
    startDrag(ev) {
      // Stop snap-to-center animation if there is one
      if (this.animation) this.animation.stop();

      // Stop momentum if any
      if (this.momentum) cancelAnimationFrame(this.momentum);
      this.momentum = null;

      this.down = true;
      this.pos = {
        // Current mouse position
        x: (ev.clientX !== undefined ? ev.clientX : ev.touches[0].clientX),
        y: (ev.clientY !== undefined ? ev.clientY : ev.touches[0].clientY),

        // Current scroll
        left: this.$el.scrollLeft,
        top: this.$el.scrollTop,
      };
    },
    snapToCenter(anim) {
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
        let end = target.child.offsetLeft - rect.width/2 + target.width/2;
        if (anim) {
          // Animate snap-to-center
          let start = this.$el.scrollLeft;
          this.animation = animate(start, end, duration, (val) => {
            this.$el.scrollLeft = val;
          });
        } else {
          this.$el.scrollLeft = end;
        }
      }
    },
    endDrag(ev) {
      this.down = false;
      if (this.dragging) {
        this.dragging = false;

        // Momentum
        this.applyMomentum();
      }
    },
    applyMomentum() {
      this.$el.scrollLeft += this.vel;
      this.vel *= 0.95;
      if (Math.abs(this.vel) > 0.5){
        this.momentum = requestAnimationFrame(this.applyMomentum);
      } else {
        this.snapToCenter(true);
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
  display: flex;
  align-items: center;
}
.cards.dragging {
  cursor: grabbing;
  user-select: none;

  /* Disable pointer events
  while dragging so we can't trigger
  children click events */
  pointer-events: none;
}
.cards .card {
  display: inline-flex;
  min-width: 320px;
  margin: 0 0.5em;
  vertical-align: top;
  white-space: normal;
  user-select: none;
}

/* Margin on either side so
the first and last cards have room
to be centered */
.cards .card:first-child {
  margin-left: 12em;
}
.cards .card:last-child {
  margin-right: 12em;
}
</style>
