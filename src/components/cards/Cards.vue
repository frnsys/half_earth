<!--
  Renders child elements side-by-side with drag-to-scroll.
  Children should be wrapped in <li> tags.
-->

<template>
<div class="cards" ref="scroller" @scroll.passive="onScroll">
  <slot></slot>
</div>
</template>

<script>
import {detectCenterElement} from 'lib/util';

// How many scroll events we wait until
// declaring that scrolling has started
const SCROLL_COUNTDOWN = 10;

export default {
  props: ['disabled'],
  data() {
    return {
      scrollTimeout: null,
    }
  },
  beforeUnmount() {
    clearInterval(this.scrollTimeout);
    clearInterval(this.focusCheck);

    this.$refs.scroller.removeEventListener('mousedown', this.dragStart);
    this.$refs.scroller.removeEventListener('mousemove', this.drag);
    this.$refs.scroller.removeEventListener('click', this.dragStop, true);
  },
  watch: {
    disabled(val) {
      if (val) {
        this.$refs.scroller.style.overflowX = 'hidden';
        this.$refs.scroller.style.paddingBottom = `${this.scrollbarHeight}px`;
      } else {
        this.$refs.scroller.style.overflowX = 'visible';
        this.$refs.scroller.style.paddingBottom = '0px';
      }
    }
  },
  mounted() {
    // Horizontally dragging for desktop
    // Use 'click' instead of 'mouseup' so we can properly
    // intercept other click events to the cards
    this.pos = {left: 0, x: 0};
    this.down = false;
    this.dragging = false;
    this.$refs.scroller.addEventListener('mousedown', this.dragStart);
    this.$refs.scroller.addEventListener('mousemove', this.drag);
    this.$refs.scroller.addEventListener('click', this.dragStop, true);

    // Hack to start with first card focused
    this.$refs.scroller.scrollLeft = this.$refs.scroller.clientWidth/2;

    // We use this to determine if the scrolling (and its momentum)
    // have stopped
    this.last = this.$refs.scroller.scrollLeft;

    // Just a flag to identify if we just started scrolling
    // as opposed to if we're in the middle of scrolling
    this.scrolling = false;

    this.scrollCountdown = SCROLL_COUNTDOWN;

    // Calculate scroll bar height so we can accommodate it
    // when it disappears when overflowX is set to hidden.
    // This prevents the layout from shifting when a card is being dragged.
    this.scrollbarHeight = this.$refs.scroller.offsetHeight - this.$refs.scroller.clientHeight;

    // Wait to see if we've stopped scrolling
    // If so, figure out what the focused/centered child is.
    this.scrollTimeout = setInterval(() => {
      let nextLast = this.$refs.scroller.scrollLeft;

      // If we are still within a scroll action and
      // momentum/snapping has finished
      // (i.e. the scroll left position hasn't changed),
      // we're done scrolling.
      if (this.scrolling && this.last == nextLast) {
        let idx = detectCenterElement(
          this.$refs.scroller,
          [...this.$refs.scroller.children]);
        this.$emit('focused', idx);
        this.$emit('scrollEnd');
        this.scrolling = false;
        this.scrollCountdown = SCROLL_COUNTDOWN;
      } else {
        this.last = nextLast;
      }
    }, 16);

    // Fallback for if the focused card detection messes up
    this.focusCheck = setInterval(() => {
      if (!this.scrolling) {
        let idx = detectCenterElement(
          this.$refs.scroller,
          [...this.$refs.scroller.children]);
        this.$emit('focused', idx);
      }
    }, 100);
  },
  methods: {
    // Drag to scroll horizontally on desktop
    dragStart(ev) {
      if (this.disabled) return;
      this.pos = {
        left: this.$refs.scroller.scrollLeft,
        x: ev.clientX,
      };
      this.down = true;
      this.$refs.scroller.style.userSelect = 'none';
    },
    drag(ev) {
      if (this.disabled) return;
      const dx = ev.clientX - this.pos.x;
      if (this.down && Math.abs(dx) > 10) {
        this.dragging = true;
        this.$refs.scroller.classList.add('unlock-scroll');
        this.$refs.scroller.scrollLeft = this.pos.left - dx;
      }
    },
    dragStop(ev) {
      if (this.dragging) {
        this.$refs.scroller.classList.remove('unlock-scroll');

        // Necessary for firefox to snap to the nearest card
        this.$refs.scroller.scroll();
        ev.preventDefault();
        ev.stopImmediatePropagation();
      }
      this.down = false;
      this.dragging = false;
    },

    onScroll(ev) {
      // If we're not already in a scroll action
      // and a scroll event is fired, that means
      // we started scrolling.
      // But we wait until seeing a certain number of
      // scroll events until firing a scrollStart event
      // to deal with some timing issues that cause
      // mobile dragging to be wonky
      if (!this.scrolling) {
        if (this.scrollCountdown > 0) {
          this.scrollCountdown--;
        } else {
          this.scrolling = true;
          this.$emit('scrollStart');
        }
      }
    },
  }
}
</script>

<style>
.cards {
  cursor: grab;
  white-space: nowrap;
  overflow-x: scroll;
  overflow-y: hidden;
  width: 100%;
  position: relative;
  display: flex;
  align-items: center;
  scroll-snap-type: x mandatory;
  scroll-snap-stop: always;
  scrollbar-color: #aaa transparent;
  padding: 1em 0 0;

  bottom: 0;
  top: 0;
  left: 0;
  right: 0;
  position: absolute;
}

.unlock-scroll {
  scroll-snap-type: none;
  scroll-snap-stop: none;
}

.cards > div {
  scroll-snap-align: center;
}

/* Margin on either side so
the first and last cards have room
to be centered */
.cards > div:first-child {
  margin-left: 50vw;
}
.cards > div:last-child {
  margin-right: 50vw;
}

@supports (-webkit-touch-callout: none) {
  /* This is really hacky,
  but for some reason mobile Safari doesn't respect
  the margin-right rule above, so the last card doesn't
  properly center except with this rule. */
  .cards > div:last-child {
    padding-right: 60px;
  }
}

.cards .card {
  display: inline-flex;
  min-width: 280px;
  margin: 0 0.5em;
  vertical-align: top;
  white-space: normal;
  user-select: none;
  -webkit-user-select: none;
}

.noscroll {
  overflow: hidden;
}
</style>
