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

import throttle from "lodash.throttle";

export default {
  props: ['id', 'draggable', 'minY', 'maxY'],
  data() {
    return {
      dragging: false,
    }
  },
  mounted() {
    this.top = 0;
    this.enabled = false;
    this.down = false;
    this.elY = 0;
    this.pos = {
      x: 0,
      y: 0,
    };
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
  created(){
    this.dragHandler = throttle((ev) => {
      this.drag(ev);
    }, 16);
  },
  methods: {
    enable() {
      if (this.enabled) return;
      this.enabled = true;
      if (isTouchDevice) {
        document.body.addEventListener('touchend', this.stopDrag);
        document.body.addEventListener('touchmove', this.dragHandler, {passive: true});
      } else {
        document.body.addEventListener('mouseup', this.stopDrag);
        document.body.addEventListener('mouseleave', this.stopDrag);
        document.body.addEventListener('mousemove', this.dragHandler, {passive: true});
      }

      // Get and cache current y position of this element
      this.observer = new IntersectionObserver((entries) => {
        let rect = entries[0].boundingClientRect;
        this.elY = rect.y;
        this.elHeight = rect.height;
        this.observer.disconnect();
      });
      this.observer.observe(this.$el);
    },
    disable() {
      if (!this.enabled) return;
      this.enabled = false;
      if (isTouchDevice) {
        document.body.removeEventListener('touchend', this.stopDrag);
        document.body.removeEventListener('touchmove', this.dragHandler, {passive: true});
      } else {
        document.body.removeEventListener('mouseup', this.stopDrag);
        document.body.removeEventListener('mouseleave', this.stopDrag);
        document.body.removeEventListener('mousemove', this.dragHandler, {passive: true});
      }
    },
    startDrag(ev) {
      if (!this.draggable) return;
      this.down = true;

      // Stop snap-back animation if there is one
      if (this.animation) this.animation.stop();

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
      let dx = (ev.clientX !== undefined ? ev.clientX : ev.touches[0].clientX) - this.pos.x;
      let dy = (ev.clientY !== undefined ? ev.clientY : ev.touches[0].clientY) - this.pos.y;

      let minY = this.minY();
      let maxY = this.maxY();
      if (Math.abs(dy) > 10) {
        this.dragging = true;
        let y = this.elY + this.top;
        if (minY && y <= minY) return;
        if (maxY && y >= maxY) return;

        let baseY = y - this.top;
        let minDY = minY - baseY;
        let maxDY = maxY - baseY;

        let deltaY = dy - this.top;
        dy = Math.min(maxDY, Math.max(minDY, dy));
        this.$el.style.transform = `translate(0, ${dy}px)`;
        this.top = dy;

        this.$emit('drag', {y, height: this.elHeight});
      } else if (Math.abs(dx) >= 2) {
        this.$emit('tryScroll', this);
      }
    },
    stopDrag() {
      this.down = false;
      this.dragging = false;

      this.animation = animate(
        this.top,
        0, 100, (top) => {
        this.top = top;
        this.$el.style.transform = `translate(0, ${top}px)`;
      });
      this.$emit('dragStop', this);
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

