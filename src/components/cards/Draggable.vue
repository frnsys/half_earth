<template>
<div
  :id="id"
  class="draggable"
  :class="{dragging:dragging, active:draggable}"
  @mousedown="startDrag"
  @touchstart="startDrag">
  <slot />
</div>
</template>

<script>
import animate from 'lib/anim';
import {updateTransform} from 'lib/util';

export default {
  props: ['id', 'draggable', 'minY', 'maxY'],
  data() {
    return {
      down: false,
      dragging: false,
      pos: {
        x: 0,
        y: 0
      }
    }
  },

  // Have to bind a lot of these to the body,
  // because the mouse may leave target areas but we still
  // need to respond to events there.
  mounted() {
    document.body.addEventListener('mouseup', this.stopDrag);
    document.body.addEventListener('touchend', this.stopDrag);
    document.body.addEventListener('mouseleave', this.stopDrag);
    document.body.addEventListener('mousemove', this.drag);
    document.body.addEventListener('touchmove', this.drag);
  },
  unmounted() {
    document.body.removeEventListener('mouseup', this.stopDrag);
    document.body.removeEventListener('touchend', this.stopDrag);
    document.body.removeEventListener('mouseleave', this.stopDrag);
    document.body.removeEventListener('mousemove', this.drag);
    document.body.removeEventListener('touchmove', this.drag);
  },
  methods: {
    startDrag(ev) {
      if (!this.draggable) return;
      this.down = true;
      /* ev.preventDefault(); // Necessary to prevent address bar from showing on drag */

      // Stop snap-back animation if there is one
      if (this.animation) this.animation.stop();

      /* updateTransform(this.$el, {rotate: '-2deg'}); */
      this.$el.style.cursor = 'grab';

      let x = (ev.clientX !== undefined ? ev.clientX : ev.touches[0].clientX);
      let y = (ev.clientY !== undefined ? ev.clientY : ev.touches[0].clientY);
      this.pos = {
        // Current mouse position
        x, y
      };
    },
    drag(ev) {
      if (!this.down) return;
      /* ev.preventDefault(); // Necessary to prevent address bar from showing on drag */
      let dx = (ev.clientX !== undefined ? ev.clientX : ev.touches[0].clientX) - this.pos.x;
      let dy = (ev.clientY !== undefined ? ev.clientY : ev.touches[0].clientY) - this.pos.y;

      let minY = this.minY();
      let maxY = this.maxY();
      if (Math.abs(dx) < 10 && Math.abs(dy) > 10) {
        this.dragging = true;
        let rect = this.$el.getBoundingClientRect();
        if (minY && rect.y <= minY) return;
        if (maxY && rect.y >= maxY) return;

        let top = parseFloat(this.$el.style.top) || 0;
        let baseY = rect.y - top;
        let minDY = minY - baseY;
        let maxDY = maxY - baseY;

        let deltaY = dy - top;
        dy = Math.min(maxDY, Math.max(minDY, dy));
        this.$el.style.top = `${dy}px`;
        /* this.$el.style.left = `${dx}px`; */
        this.$emit('drag', this);
      }
    },
    stopDrag() {
      this.down = false;
      this.dragging = false;

      this.animation = animate(
        [parseInt(this.$el.style.top), parseInt(this.$el.style.left)],
        [0, 0], 100, (top, left) => {
        this.$el.style.top = `${top}px`;
        this.$el.style.left = `${left}px`;
      });
      /* updateTransform(this.$el, {rotate: '0deg'}); */
      this.$emit('dragStop', this);
    }
  }
}
</script>

<style>
.draggable {
  position: relative;
}
.draggable.dragging {
  cursor: grabbing;
  user-select: none;
}

.draggable.dragging > * {
  /* Disable pointer events
  while dragging so we can't trigger
  children click events */
  pointer-events: none;
}
</style>
