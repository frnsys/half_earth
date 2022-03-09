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
import {updateTransform, isTouchDevice} from 'lib/util';

export default {
  props: ['id', 'draggable', 'minY', 'maxY'],
  data() {
    return {
      down: false,
      enabled: false,
      dragging: false,
      pos: {
        x: 0,
        y: 0
      }
    }
  },
  mounted() {
    if (this.draggable) {
      this.enable();
    }
  },
  unmounted() {
    this.disable();
  },
  watch: {
    draggable(draggable, prev) {
      if (draggable !== prev) {
        if (!draggable) {
          this.disable();
        } else if (!this.enabled) {
          this.enable();
        }
      }
    }
  },
  methods: {
    enable() {
      if (this.enabled) return;
      this.enabled = true;
      if (isTouchDevice) {
        document.body.addEventListener('touchend', this.stopDrag);
        document.body.addEventListener('touchmove', this.drag, {passive: true});
      } else {
        document.body.addEventListener('mouseup', this.stopDrag);
        document.body.addEventListener('mouseleave', this.stopDrag);
        document.body.addEventListener('mousemove', this.drag, {passive: true});
      }
    },
    disable() {
      if (!this.enabled) return;
      this.enabled = false;
      if (isTouchDevice) {
        document.body.removeEventListener('touchend', this.stopDrag);
        document.body.removeEventListener('touchmove', this.drag, {passive: true});
      } else {
        document.body.removeEventListener('mouseup', this.stopDrag);
        document.body.removeEventListener('mouseleave', this.stopDrag);
        document.body.removeEventListener('mousemove', this.drag, {passive: true});
      }
    },
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
      if (!this.draggable) this.stopDrag();
      if (!this.down) return;
      /* ev.preventDefault(); // Necessary to prevent address bar from showing on drag */
      let dx = (ev.clientX !== undefined ? ev.clientX : ev.touches[0].clientX) - this.pos.x;
      let dy = (ev.clientY !== undefined ? ev.clientY : ev.touches[0].clientY) - this.pos.y;

      let minY = this.minY();
      let maxY = this.maxY();
      if (Math.abs(dy) > 10) {
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
      } else if (Math.abs(dx) >= 2) {
        this.$emit('tryScroll', this);
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
      this.$emit('dragStop', this);
      /* updateTransform(this.$el, {rotate: '0deg'}); */
    }
  }
}
</script>

<style>
.draggable {
  position: relative;
  perspective: 3000px;
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

