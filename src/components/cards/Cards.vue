<!--
  Renders child elements side-by-side with drag-to-scroll.
  Children should be wrapped in <li> tags.
-->

<template>
<ul class="cards" :class="{noscroll: disabled}" ref="scroller" @scroll="scrolled">
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
    }
  },
  methods: {
    scrolled(ev) {
      if (this.scrollTimeout !== null) {
        clearTimeout(this.scrollTimeout);
      }

      // Wait to see if we've stopped scrolling
      // If so, figure out what the focused/centered child is.
      this.scrollTimeout = setTimeout(() => {
        let idx = detectCenterElement(
          this.$refs.scroller,
          [...this.$refs.scroller.children]);
        if (idx >= 0) {
          this.$emit('focused', idx);
        }
      }, 50);
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
  padding: 0 25%;
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
