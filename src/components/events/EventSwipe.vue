<template>
<div id="event-area">
  <div ref="choice-top" class="choice choice-top" v-if="event.choices[2]">
    {{event.choices[2].text || 'MISSING TEXT'}}
  </div>
  <div class="center-thing">
    <div ref="choice-left" class="choice choice-left" v-if="event.choices[0]">
      {{event.choices[0].text || 'MISSING TEXT'}}
    </div>
    <Draggable ref="draggable"
      @drag="onCardDrag" @dragStop="onCardDragStop">
      <div class="event-card">{{event.text}}</div>
    </Draggable>
    <div ref="choice-right" class="choice choice-right" v-if="event.choices[1]">
      {{event.choices[1].text || 'MISSING TEXT'}}
    </div>
  </div>
  <div ref="choice-bottom" class="choice choice-bottom" v-if="event.choices[3]">
    {{event.choices[3].text || 'MISSING TEXT'}}
  </div>
</div>
</template>

<script>
import Draggable from './Draggable.vue';

const dirToIndex = {
  'left': 0,
  'right': 1,
  'top': 2,
  'bottom': 3
};

export default {
  props: ['event'],
  beforeUpdate() {
    // TODO this is a temporary placeholder
    // until we figure out if we want to do both
    // swiper events and dialogue
    if (!this.event.choices) {
      this.event.choices = [{
        'text': 'Choice A',
      }, {
        'text': 'Choice B',
      }, {
        'text': 'Choice C',
      }, {
        'text': 'Choice D',
      }]
    }
  },
  data() {
    // TODO this is a temporary placeholder
    // until we figure out if we want to do both
    // swiper events and dialogue
    if (!this.event.choices) {
      this.event.choices = [{
        'text': 'Choice A',
      }, {
        'text': 'Choice B',
      }, {
        'text': 'Choice C',
      }, {
        'text': 'Choice D',
      }]
    }
    return {
      choiceDir: null,
      selectedChoice: null
    }
  },
  components: {
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
        this.resetChoice();
        let dir = dirs[0];
        // Choose most offscreen direction
        if (dirs.length > 1) {
          dir = dirs.reduce((a, b) => offscreen[a] > offscreen[b] ? a : b);
        }
        let optEl = this.$refs[`choice-${dir}`];
        if (optEl) {
          // How much of the card needs to be offscreen to choose the choice.
          // Vertical drag distance is longer on phones, so require less of the
          // card to be offscreen.
          let pOff = dir == 'top' || dir == 'bottom' ? 1/16 : 1/10;
          let p = Math.min(50, 50 * offscreen[dir]/pOff);
          optEl.style[dir] = `${p}%`;
          if (p >= 50) {
            optEl.classList.add('selected');
            this.selectedChoice = dirToIndex[dir];
          } else {
            optEl.classList.remove('selected');
            this.selectedChoice = null;
          }
          this.choiceDir = dir;
          draggable.$el.style.opacity = `${100-p*1.75}%`;
          optEl.style.background = `rgba(20,20,20,${p*2/100})`;
        }
      }
    },
    onCardDragStop() {
      if (this.selectedChoice) {
        this.$emit('selected', this.selectedChoice);
        this.resetChoice();
      } else {
        this.resetChoice();
      }
    },
    resetChoice() {
      if (this.choiceDir) {
        let optEl = this.$refs[`choice-${this.choiceDir}`];
        optEl.style[this.choiceDir] = 0;
        optEl.classList.remove('selected');
        this.$refs.draggable.$el.style.opacity = 1;
        optEl.style.background = `rgba(20,20,20,0)`;
      }
      this.selectedChoice = null;
    },
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
.choice {
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
.choice p {
  margin-bottom: 0;
}
.choice.selected {
  color: #43CC70;
}
.choice-top {
  transform: translate(0, -50%);
}
.choice-bottom {
  transform: translate(0, 50%);
}
.choice-left {
  transform: translate(-50%, -50%);
  left: 0;
  top: 50%;
}
.choice-right {
  transform: translate(50%, -50%);
  right: 0;
  top: 50%;
}

.center-thing {
  display: flex;
  justify-content: space-around;
  position: relative;
}
.center-thing .choice {
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  position: absolute;
}

.event-card {
  color: #fff;
  background: #1b1b1b;
  width: 280px;
  height: 400px;
  margin: 0 auto;
  padding: 1em;
  border-radius: 1em;
  text-align: center;
}

#event-area {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  min-height: 100vh;
}
</style>
