<template>
<div class="draggable"
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
  data() {
    return {
      dragging: false,
      pos: {
        x: 0,
        y: 0
      }
    }
  },
  methods: {
    startDrag(ev) {
      this.dragging = true;
      ev.preventDefault(); // Necessary to prevent address bar from showing on drag

      // Stop snap-back animation if there is one
      if (this.animation) this.animation.stop();

      updateTransform(this.$el, {rotate: '-2deg'});
      this.$el.style.cursor = 'grab';

      this.pos = {
        // Current mouse position
        x: (ev.clientX !== undefined ? ev.clientX : ev.touches[0].clientX),
        y: (ev.clientY !== undefined ? ev.clientY : ev.touches[0].clientY),
      };

      this.$emit('dragStart', this);
    },
    drag(ev) {
      if (!this.dragging) return;
      ev.preventDefault(); // Necessary to prevent address bar from showing on drag
      const dx = (ev.clientX !== undefined ? ev.clientX : ev.touches[0].clientX) - this.pos.x;
      const dy = (ev.clientY !== undefined ? ev.clientY : ev.touches[0].clientY) - this.pos.y;
      this.$el.style.top = `${dy}px`;
      this.$el.style.left = `${dx}px`;
      this.$emit('drag', this);
    },
    stopDrag() {
      this.dragging = false;

      this.animation = animate(
        [parseInt(this.$el.style.top), parseInt(this.$el.style.left)],
        [0, 0], 100, (top, left) => {
        this.$el.style.top = `${top}px`;
        this.$el.style.left = `${left}px`;
      });
      updateTransform(this.$el, {rotate: '0deg'});
      this.$emit('dragStop', this);
    }
  }
}
</script>

<style>
.draggable {
  position: relative;
}
</style>
