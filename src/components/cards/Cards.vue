<!--
  Renders child elements side-by-side with drag-to-scroll.
  Children should be wrapped in <li> tags.
-->

<template>
<ul class="cards" ref="scroller" @scroll.passive="onScroll">
  <slot></slot>
</ul>
</template>

<script>
import throttle from "lodash.throttle";
import {detectCenterElement} from 'lib/util';

export default {
  props: ['disabled'],
  data() {
    return {
      scrollTimeout: null,
      scrollLeft: 0,
    }
  },
  created() {
    this.scrollHandler = throttle((ev) => {
      this.scrolled(ev);
    }, 16);
  },
  beforeUnmount() {
    this.scrollHandler.cancel();
  },
  watch: {
    disabled(val) {
      if (val) {
        this.scrollLeft = this.$refs.scroller.scrollLeft;
      }
    }
  },
  mounted() {
    // Hack to start with first card focused
    this.$refs.scroller.scrollLeft = this.$refs.scroller.clientWidth/2;
  },
  methods: {
    onScroll(ev) {
      if (this.disabled) {
        this.$refs.scroller.scrollLeft = this.scrollLeft;
        return;
      }
      this.$emit('scrolled');
      this.scrollHandler(ev);
    },
    scrolled(ev) {
      if (this.scrollTimeout !== null) {
        clearInterval(this.scrollTimeout);
      }

      // Wait to see if we've stopped scrolling
      // If so, figure out what the focused/centered child is.
      let last = this.$refs.scroller.scrollLeft;
      this.scrollTimeout = setInterval(() => {
        let nextLast = this.$refs.scroller.scrollLeft;
        if (last == nextLast) {
          let idx = detectCenterElement(
            this.$refs.scroller,
            [...this.$refs.scroller.children]);
          this.$emit('focused', idx);
          this.$emit('scrollEnd');
          clearInterval(this.scrollTimeout);
        } else {
          last = nextLast;
        }
      }, 100);
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
  /* so there's enough space to center the
  first and last items */
  padding: 1em 25% 0;

  bottom: 0;
  top: 0;
  left: 0;
  right: 0;
  position: absolute;
}
.cards > * {
  scroll-snap-align: center;
}
.cards .card {
  display: inline-flex;
  min-width: 280px;
  margin: 0 0.5em;
  vertical-align: top;
  white-space: normal;
  user-select: none;
}

/* Margin on either side so
the first and last cards have room
to be centered */
.cards > div:first-child {
  margin-left: 20em;
}
.cards > div:last-child {
  margin-right: 20em;
}

.noscroll {
  overflow: hidden;
}
</style>
