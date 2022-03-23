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
import throttle from "lodash.throttle";
import {updateTransform} from 'lib/util';

export default {
  props: ['id', 'draggable', 'minY', 'maxY'],
  data() {
    return {
      dragging: false,
    }
  },
  mounted() {
    // Keep track of the top offset from the element's starting y position;
    // this is updated as the component is dragged
    this.top = 0;

    // Whether or not dragging is enabled
    this.enabled = false;

    // Whether or not dragging is started,
    // i.e. the component has been clicked or touched
    this.down = false;

    // Cache the starting y position of the element
    this.elY = 0;

    // Current position of the cursor
    this.pos = {
      x: 0,
      y: 0,
    };

    if (this.draggable) {
      this.enable();
    }
  },
  beforeUnmount() {
    this.disable();
  },
  watch: {
    draggable(draggable, prev) {
      // Check if draggable prop has changed
      if (draggable !== prev) {
        // If not draggable, disable dragging events
        if (!draggable) {
          this.disable();
          this.stopDrag();

        // If draggable and not already enabled,
        // enable dragging events
        } else if (!this.enabled) {
          this.enable();
        }
      }
    }
  },
  created(){
    // Throttle the drag handler to avoid unnecessary computations
    // 16ms is for 60fps
    this.dragHandler = throttle((ev) => {
      this.drag(ev);
    }, 16);
  },
  methods: {
    enable() {
      if (this.enabled) return;
      this.enabled = true;
      document.body.addEventListener('touchmove', this.dragHandler, {passive: true});
      document.body.addEventListener('mousemove', this.dragHandler, {passive: true});
      window.addEventListener('mouseup', this.stopDrag);
      window.addEventListener('touchend', this.stopDrag);

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
      document.body.removeEventListener('touchmove', this.dragHandler, {passive: true});
      document.body.removeEventListener('mousemove', this.dragHandler, {passive: true});
      window.removeEventListener('mouseup', this.stopDrag);
      window.removeEventListener('touchend', this.stopDrag);
    },
    startDrag(ev) {
      if (!this.draggable) return;
      this.down = true;

      // Stop snap-back animation if there is one
      if (this.animation) this.animation.stop();

      this.$el.style.cursor = 'grab';

      // Update current mouse position
      let x = (ev.clientX !== undefined ? ev.clientX : ev.touches[0].clientX);
      let y = (ev.clientY !== undefined ? ev.clientY : ev.touches[0].clientY);
      this.pos = {
        x, y
      };
    },
    drag(ev) {
      if (!this.down) return;
      let dx = (ev.clientX !== undefined ? ev.clientX : ev.touches[0].clientX) - this.pos.x;
      let dy = (ev.clientY !== undefined ? ev.clientY : ev.touches[0].clientY) - this.pos.y;

      let minY = this.minY();
      let maxY = this.maxY();

      if (Math.abs(dy) > Math.abs(dx)) {
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
