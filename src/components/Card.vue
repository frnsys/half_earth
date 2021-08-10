<template>
<div class="card"
  @mousedown="onDragStart"
  @touchstart="onDragStart"
  @mousemove="onDrag"
  @touchmove="onDrag"
  @mouseup="onDragStop"
  @touchend="onDragStop"
  @mouseleave="onLeave"
  >{{card.name}}</div>
</template>

<script>
import util from '../util';

export default {
  props: {
    card: Object,
    draggable: Boolean
  },
  data() {
    return {
      isDraggable: this.draggable,
      dragging: false,
      snapTimeout: null,
      prevTouch: Object
    }
  },
  methods: {
    // We could use the HTML drag-and-drop API
    // but it doesn't seem quite suited to the simple
    // use-case here and seems like it might be complicated
    // to adapt it for this case
    // This is buggy but fine for now (e.g. if you drag outside the window)
    // TODO the better way to do this is:
    // on click, get cursor/touch offset from card container
    // set card position to cursor/touch position on move, preserving original offset
    onDragStart(ev) {
      console.log(ev);
      if (!this.isDraggable) return;
      this.dragging = true;
      /* this.$el.style.transform = 'rotate(-2deg)'; */
      util.updateTransform(this.$el, {rotate: '-2deg'});
      this.$el.style.cursor = 'grab';
      if (this.snapTimeout) clearTimeout(this.snapTimeout);
      this.$emit('onDragStart', this);
    },
    onDrag(ev) {
      if (!this.isDraggable) return;
      if (this.dragging) {
        const $card = this.$el;
        let delta = this.getMovementDelta(ev);
        let top = $card.style.top || 0;
        let newTop = `${parseInt(top) + delta.y}px`;
        let left = $card.style.left || 0;
        let newLeft = `${parseInt(left) + delta.x}px`;
        $card.style.top = newTop;
        $card.style.left = newLeft;
        this.$emit('onDrag', this);
      }
    },
    onDragStop() {
      if (!this.isDraggable) return;
      this.dragging = false;
      this.prevTouch = null;
      this.$emit('onDragStop', this);
    },
    resetDrag() {
      // Snap card back to position
      this.$el.style.transform = '';
      this.$el.style.cursor = 'default';
      this.$el.style.transition = 'all 0.2s';
      this.$el.style.top = `0px`;
      this.$el.style.left = `0px`;
      this.snapTimeout = setTimeout(() => {
        this.$el.style.transition = '';
      }, 200);
    },
    stopDrag() {
      this.resetDrag();
    },
    onLeave() {
      this.onDragStop();
    },
    getMovementDelta(ev) {
      // Mobile/touch
      if (ev.touches) {
        let touch = ev.touches[0];
        let delta = this.prevTouch ? {
          x: touch.pageX - this.prevTouch.pageX,
          y: touch.pageY - this.prevTouch.pageY
        } : {
          x: 0, y: 0
        };
        this.prevTouch = touch;
        return delta;

      // Mouse
      } else {
        return {
          x: ev.movementX,
          y: ev.movementY
        };
      }
    }
  }
}
</script>

<style scoped>
.card {
  position: relative;
  border-radius: 0.5em;
  width: 200px;
  height: 250px;
  background: #202020;
  background: #F17F5A;
  color: #fff;
  text-align: center;
  display: flex;
  align-items: center;
  justify-content: space-around;
  z-index: 1;
}
</style>
