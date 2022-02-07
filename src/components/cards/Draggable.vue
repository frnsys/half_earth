<template>
<div
  :id="id"
  class="draggable"
  :class="{dragging:dragging, active:draggable}"
  @mousedown="startDrag"
  @touchstart="startDrag"
  @mousemove="drag"
  @touchmove="drag"
  @mouseup="stopDrag"
  @touchend="stopDrag"
  @mouseleave="stopDrag">
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
      ev.preventDefault(); // Necessary to prevent address bar from showing on drag
      const dx = (ev.clientX !== undefined ? ev.clientX : ev.touches[0].clientX) - this.pos.x;
      const dy = (ev.clientY !== undefined ? ev.clientY : ev.touches[0].clientY) - this.pos.y;

      let minY = this.minY();
      let maxY = this.maxY();
      if (Math.abs(dx) < 20 && Math.abs(dy) > 10) {
        this.dragging = true;
        let rect = this.$el.getBoundingClientRect();
        if (minY && rect.y <= minY) return;
        if (maxY && rect.y >= maxY) return;

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

