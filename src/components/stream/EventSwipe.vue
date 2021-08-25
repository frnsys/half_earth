<template>
<div id="event-area">
  <Globe />
  <div ref="option-top" class="option option-top">
    Option A
    <p>These are some details.</p>
  </div>
  <div class="center-thing">
    <div ref="option-left" class="option option-left">Option C</div>
    <Draggable ref="draggable" @drag="onCardDrag" @dragStop="resetOption">
      <div class="test-card">testing</div>
    </Draggable>
    <div ref="option-right" class="option option-right">Option D</div>
  </div>
  <div ref="option-bottom" class="option option-bottom">
    🔒
    <p>If we had more people, maybe we could try something else...</p>
  </div>
</div>
</template>

<script>
import util from 'lib/util';
import Draggable from './Draggable.vue';
import Globe from '../Globe.vue';

export default {
  data() {
    return {
      optionDir: null
    }
  },
  components: {
    Globe,
    Draggable
  },
  methods: {
    onCardDrag(draggable) {
      const card = draggable.$el.getBoundingClientRect();
      const parent = this.$el.getBoundingClientRect();

      // p card off screen in any given direction
      let topBleed = card.y;
      let bottomBleed = card.y - parent.height + card.height;
      let leftBleed = card.x - parent.x;
      let rightBleed = card.x - parent.x - parent.width + card.width;
      let offscreen = {
        top: topBleed < 0 ? -topBleed/card.height : 0,
        bottom: bottomBleed > 0 ? bottomBleed/card.height : 0,
        left: leftBleed < 0 ? -leftBleed/card.width : 0,
        right: rightBleed > 0 ? rightBleed/card.width : 0,
      };

      let dirs = Object.keys(offscreen).filter((k) => offscreen[k] > 0);
      if (dirs.length > 0) {
        this.resetOption();
        let dir = dirs[0];
        // Choose most offscreen direction
        if (dirs.length > 1) {
          dir = dirs.reduce((a, b) => offscreen[a] > offscreen[b] ? a : b);
        }
        let optEl = this.$refs[`option-${dir}`];

        // How much of the card needs to be offscreen to choose the option.
        // Vertical drag distance is longer on phones, so require less of the
        // card to be offscreen.
        let pOff = dir == 'top' || dir == 'bottom' ? 1/16 : 1/10;
        let p = Math.min(50, 50 * offscreen[dir]/pOff);
        optEl.style[dir] = `${p}%`;
        if (p >= 50) {
          optEl.classList.add('selected');
        } else {
          optEl.classList.remove('selected');
        }
        this.optionDir = dir;
        draggable.$el.style.opacity = `${100-p*1.75}%`;
        optEl.style.background = `rgba(255,255,255,${p*2/100})`;
      }
    },
    resetOption() {
      if (this.optionDir) {
        let optEl = this.$refs[`option-${this.optionDir}`];
        optEl.style[this.optionDir] = 0;
        optEl.classList.remove('selected');
        this.$refs.draggable.$el.style.opacity = 1;
        optEl.style.background = `rgba(255,255,255,0)`;
      }
    }
  }
}
</script>

<style>
#event-area {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  height: 100vh;
  overflow: hidden;
}
.option {
  color: #aaa;
  margin: 0 auto;
  text-align: center;
  position: relative;
  z-index: 2;
  max-width: 200px;

  padding: 1em;
  border-radius: 1em;

  /* So it doesn't interfere with draggable interaction */
  user-select: none;
  pointer-events: none;
}
.option p {
  margin-bottom: 0;
}
.option.selected {
  color: #43CC70;
}
.option-top {
  transform: translate(0, -50%);
}
.option-bottom {
  transform: translate(0, 50%);
}
.option-left {
  transform: translate(-50%, -50%);
  left: 0;
  top: 50%;
}
.option-right {
  transform: translate(50%, -50%);
  right: 0;
  top: 50%;
}

.center-thing {
  display: flex;
  justify-content: space-around;
  position: relative;
}
.center-thing .option {
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  position: absolute;
}

.test-card {
  background: #fff;
  width: 280px;
  height: 400px;
  margin: 0 auto;
  padding: 1em;
  border-radius: 1em;
  text-align: center;
}

#event-area {
  position: relative;
  min-height: 100vh;
}
#event-area #globe {
  position: absolute;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  z-index: -1;
}
</style>
