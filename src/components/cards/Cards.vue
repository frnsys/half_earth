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
import {detectCenterElement} from 'lib/util';

export default {
  props: ['disabled'],
  data() {
    return {
      scrollTimeout: null,
      scrollLeft: 0,
    }
  },
  beforeUnmount() {
    clearInterval(this.scrollTimeout);
    document.removeEventListener('keydown', this.onKeyDown);
  },
  watch: {
    disabled(val) {
      if (val) {
        this.$refs.scroller.style.overflowX = 'hidden';
      } else {
        this.$refs.scroller.style.overflowX = 'visible';
      }
    }
  },
  mounted() {
    document.addEventListener('keydown', this.onKeyDown);

    // Hack to start with first card focused
    this.$refs.scroller.scrollLeft = this.$refs.scroller.clientWidth/2;
    this.last = this.$refs.scroller.scrollLeft;
    this.scrolling = false;

    // Wait to see if we've stopped scrolling
    // If so, figure out what the focused/centered child is.
    this.scrollTimeout = setInterval(() => {
      let nextLast = this.$refs.scroller.scrollLeft;
      if (this.scrolling && this.last == nextLast) {
        let idx = detectCenterElement(
          this.$refs.scroller,
          [...this.$refs.scroller.children]);
        this.$emit('focused', idx);
        this.$emit('scrollEnd');
        this.scrolling = false;
      } else {
        this.last = nextLast;
      }
    }, 100);
  },
  methods: {
    onScroll(ev) {
      if (!this.scrolling) {
        this.$emit('scrollStart');
        this.scrolling = true;
      }
    },
    onKeyDown(ev) {
      if (ev.key == 'ArrowUp') {
        let children = [...this.$refs.scroller.children];
        let idx = detectCenterElement(
          this.$refs.scroller, children);

        let left = 0;
        if (idx < children.length - 1) {
          let el = children[idx];
          left = el.offsetLeft - el.offsetWidth/2;
        } else {
          let el = children[0];
          left = el.offsetLeft - el.offsetWidth - el.offsetWidth/2;
        }
        this.$refs.scroller.scroll({
          left: left,
          behavior: 'smooth'
        });

        return false;
      } else if (ev.key == 'ArrowDown') {
        let children = [...this.$refs.scroller.children];
        let idx = detectCenterElement(
          this.$refs.scroller, children);

        let left = 0;
        if (idx > 0) {
          let el = children[idx-1];
          left = el.offsetLeft - el.offsetWidth - el.offsetWidth/2;
        } else {
          let el = children[children.length-1];
          left = el.offsetLeft - el.offsetWidth - el.offsetWidth/2;
        }
        this.$refs.scroller.scroll({
          left: left,
          behavior: 'smooth'
        });


        return false;
      }
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
