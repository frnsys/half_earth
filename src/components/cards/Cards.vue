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
    // document.removeEventListener('keydown', this.onKeyDown);
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
    // document.addEventListener('keydown', this.onKeyDown);

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
  },
  methods: {
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
    onKeyDown(ev) {
      // TODO this is all messed up
      //if (ev.key == 'ArrowUp' || ev.key == 'ArrowRight') {
      //  ev.preventDefault();
      //  let children = [...this.$refs.scroller.children];
      //  let idx = detectCenterElement(
      //    this.$refs.scroller, children);

      //  let left = 0;
      //  if (idx < children.length - 1) {
      //    let el = children[idx];
      //    left = el.offsetLeft - el.offsetWidth/2;
      //  } else {
      //    let el = children[0];
      //    left = el.offsetLeft - el.offsetWidth - el.offsetWidth/2;
      //  }
      //  this.$refs.scroller.scroll({
      //    left: left,
      //    behavior: 'smooth'
      //  });

      //  return false;
      //} else if (ev.key == 'ArrowDown' || ev.key == 'ArrowLeft') {
      //  ev.preventDefault();
      //  let children = [...this.$refs.scroller.children];
      //  let idx = detectCenterElement(
      //    this.$refs.scroller, children);

      //  let left = 0;
      //  if (idx > 0) {
      //    let el = children[idx-1];
      //    left = el.offsetLeft - el.offsetWidth - el.offsetWidth/2;
      //  } else {
      //    let el = children[children.length-1];
      //    left = el.offsetLeft - el.offsetWidth - el.offsetWidth/2;
      //  }
      //  this.$refs.scroller.scroll({
      //    left: left,
      //    behavior: 'smooth'
      //  });


      //  return false;
      //}
    }
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
}

.noscroll {
  overflow: hidden;
}
</style>
