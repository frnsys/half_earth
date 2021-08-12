<template>
<div class="tip">
  <div class="tip-icon" :class="{active:visible}"
    @click="toggle"
    @mouseenter="show"
    @mouseleave="hide">?</div>
  <div class="tip-tooltip" v-if="visible" :style="style">
    <slot></slot>
  </div>
</div>
</template>

<script>
export default {
  props: {
    placement: {
      type: String,
      default: 'bottom'
    },
    width: {
      type: Number,
      default: 200
    }
  },
  data() {
    return {
      visible: false
    };
  },
  mounted() {
    this._clickOutside = (ev) => {
      if (this.visible) {
        let clickedOutside = !(this.$el == ev.target || this.$el.contains(ev.target))
        if (clickedOutside) {
          this.visible = false;
        }
      }
    };
    document.addEventListener('click', this._clickOutside);
  },
  unmounted(el) {
    document.removeEventListener('click', this._clickOutside);
  },
  methods: {
    toggle() {
      this.visible = !this.visible;
    },
    show() {
      this.visible = true;
    },
    hide() {
      this.visible = false;
    }
  },
  computed: {
    style() {
      let style = {};
      if (this.placement == 'top') {
        style.top = 'calc(-100% - 2px)';
      } else {
        style.top = 'calc(100% + 2px)';
      }
      style.width = `${this.width}px`;
      return style;
    }
  }
}
</script>

<style>
.tip {
  position: relative;
  display: inline-block;
}
.tip-icon {
  color: #ccc;
	border: 1px solid #ccc;
	border-radius: 20em;
	padding: 0 0.4em;
  cursor: pointer;
  margin: 0 0.25em;
  font-size: 0.8em;
  user-select: none;
}
.tip-icon.active,
.tip-icon.active:hover {
  color: #222;
  border-color: #222;
}
.tip-icon:hover {
  color: #aaa;
  border-color: #aaa;
}
.tip-tooltip {
  color: #fff;
  background: #222;
  text-align: left;
  padding: 0.25em 0.5em;

  position: absolute;
  left: 50%;
  transform: translate(-50%, 0);
}
</style>
