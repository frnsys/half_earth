<template>
  <div id="hand">
    <!-- TODO temporary: slidesPerView for 480 should be 5 and 720 should be 7-->
    <swiper
      :slides-per-view="3"
      :space-between="5"
      :centered-slides=true
      :breakpoints="{480: {slidesPerView: 3}, 720: {slidesPerView: 3}}"
      :touchStartPreventDefault=false
      @swiper="onSwiper"
      @slideChange="onSlideChange"
      @sliderMove="onSliderMove"
      @touchEnd="onTouchEnd"
      @setTranslate="setTranslate"
      @setTransition="setTransition"
    >
      <swiper-slide v-for="(card, idx) in cards" :key="card.id">
        <Card :ref="setCardRef"
          @onDragStart="onDragStart"
          @onDragStop="onDragStop"
          @onDrag="onDrag">
          <Response :response="card" />
        </Card>
      </swiper-slide>
    </swiper>
  </div>
  <div id="playzone" ref="playzone">Play</div>
</template>

<script>
import util from '../../util';
import Card from './Card.vue'
import Response from './Response.vue'
import SwiperCore, { A11y } from 'swiper';
import { Swiper, SwiperSlide } from 'swiper/vue';
import 'swiper/swiper.scss';
SwiperCore.use([A11y]);

const params = {
  stretch: 0,
  rotate: 50,
  depth: 100,
  scale: 0.8,
  modifier: 1
};

const targetCardScale = 1.1;
const zoneRadius = 0.5;

// A little hacky, but drop zone is in the center of the globe div
const dragTarget = () => {
  let globe = document.querySelector('#globe');
  let rect = globe.getBoundingClientRect();
  return rect.top + rect.height/2;
}

export default {
  props: {
    cards: Array,
  },
  data() {
    return {
      swiper: null,
      breakpoints: {
        480: {
          slidersPerView: 3
        },
      },
      cardRefs: []
    }
  },
  components: {
    Card,
    Swiper,
    SwiperSlide,
    Response,
  },
  beforeUpdate() {
    this.cardRefs = [];
  },
  methods: {
    setCardRef(el) {
      if (el) this.cardRefs.push(el);
    },
    onDrag(card) {
      const box = card.$el.getBoundingClientRect();
      const yCenter = dragTarget();
      const yCenterOffset = yCenter - box.height/2;
      const distToCenter = yCenterOffset - box.top;
      const pDistToCenter = distToCenter/yCenter;
      const p = (1 + pDistToCenter);
      const scale = Math.min(1 + (targetCardScale - 1) * p, targetCardScale);
      util.updateTransform(card.$el, {scale});

      const opacity = Math.min(1 * p, 1);
      this.$refs.playzone.style.opacity = opacity * 0.2;

      const $playzone = this.$refs.playzone;
      $playzone.style.opacity = 1. + pDistToCenter + zoneRadius;
    },
    onDragStart() {
      this.swiper.disable();
    },
    onDragStop(card) {
      const $card = card.$el;
      const box = $card.getBoundingClientRect();
      const $playzone = this.$refs.playzone;

      const yCenter = dragTarget();
      const yCenterOffset = yCenter - box.height/2;
      const distToCenter = yCenterOffset - box.top;
      const pDistToCenter = distToCenter/yCenter;

      // Play card if close enough to drop zone
      $playzone.style.opacity = 0.0;
      if (pDistToCenter + zoneRadius > 0) {
        $card.style.left = 0;
        $card.style.top = `${parseInt($card.style.top) + distToCenter}px`;
        util.updateTransform($card, {rotate: '0deg'});
        $card.classList.add('card-preview');

        // TODO finish
        this.$emit('cardPlayed', card);

      // Reset card to hand
      } else {
        $card.classList.remove('card-preview');

        card.resetDrag();
        this.swiper.enable();
      }
    },
    onSwiper(swiper) {
      this.swiper = swiper;
      this.cardRefs[swiper.activeIndex].isDraggable = true;
    },
    onSliderMove(swiper) {
      this.cardRefs[swiper.activeIndex].isDraggable = false;
    },
    onTouchEnd(swiper) {
      this.cardRefs[swiper.activeIndex].isDraggable = true;
    },

    // This determines card positions in the swiper
    // Adapted from:
    // https://raw.githubusercontent.com/nolimits4web/swiper/master/src/components/effect-coverflow/effect-coverflow.js
    setTranslate(swiper) {
      swiper.updateSlidesOffset();
      const { width: swiperWidth, height: swiperHeight, slides, slidesSizesGrid } = swiper;
      /* const isHorizontal = swiper.isHorizontal(); */
      const isHorizontal = true;
      const transform = swiper.translate;
      const center = isHorizontal ? -transform + swiperWidth / 2 : -transform + swiperHeight / 2;
      const rotate = isHorizontal ? params.rotate : -params.rotate;
      const translate = params.depth;
      // Each slide offset from center
      for (let i = 0, length = slides.length; i < length; i += 1) {
        const $slideEl = slides.eq(i);
        const slideSize = slidesSizesGrid[i];
        const slideOffset = $slideEl[0].swiperSlideOffset;
        const offsetMultiplier =
        ((center - slideOffset - slideSize / 2) / slideSize) * params.modifier;

        let rotateY = isHorizontal ? rotate * offsetMultiplier : 0;
        let rotateX = isHorizontal ? 0 : rotate * offsetMultiplier;
        // var rotateZ = 0

        let translateZ = -translate * Math.abs(offsetMultiplier);

        let stretch = params.stretch;
        // Allow percentage to make a relative stretch for responsive sliders
        if (typeof stretch === 'string' && stretch.indexOf('%') !== -1) {
        stretch = (parseFloat(params.stretch) / 100) * slideSize;
        }
        let translateY = isHorizontal ? 0 : stretch * offsetMultiplier;
        let translateX = isHorizontal ? stretch * offsetMultiplier : 0;

        let scale = 1 - (1 - params.scale) * Math.abs(offsetMultiplier);

        // Limit rotation and scale
        rotateY = Math.max(Math.min(rotateY, 45), -45);
        scale = Math.max(scale, 0.75);

        // Fix for ultra small values
        if (Math.abs(translateX) < 0.001) translateX = 0;
        if (Math.abs(translateY) < 0.001) translateY = 0;
        if (Math.abs(translateZ) < 0.001) translateZ = 0;
        if (Math.abs(scale) < 0.001) scale = 0;
        if (Math.abs(rotateY) < 0.001) rotateY = 0;
        if (Math.abs(rotateX) < 0.001) rotateX = 0;

        /* const slideTransform = `translate3d(${translateX}px,${translateY}px,${translateZ}px) scale(${scale})`; */
        const slideTransform = `translate3d(${translateX}px,${translateY}px,${translateZ}px)  rotateX(${rotateX}deg) rotateY(${rotateY}deg) scale(${scale})`;

        $slideEl.transform(slideTransform);
        $slideEl[0].style.zIndex = -Math.abs(Math.round(offsetMultiplier)) + 1;
      }
    },
    setTransition(swiper, duration) {
      swiper.slides.transition(duration);
    },
    onSlideChange(swiper) {
      this.cardRefs[swiper.previousIndex].isDraggable = false;
      this.cardRefs[swiper.previousIndex].stopDrag();
      this.cardRefs[swiper.activeIndex].isDraggable = true;
    },
  },
};
</script>

<style scoped>
#hand {
  position: relative;
  z-index: 2;
  width: 100%;
  height: 200px;
  /* overflow: hidden; */
}
.swiper-container {
  overflow: visible;

  /* Let the container spill out over
  the sides so we only show peeks of next/prev cards */
  width: 150%;
  margin-left: -24%; /* Re-center the container */
}
.swiper-slide {
  /* Scale down next/prev cards */
  transform: scale(80%);
}
.swiper-slide-active {
  transform: scale(100%);
}

#stage {
  display: flex;
  align-items: center;
  justify-content: space-around;
}
#playzone {
  color: red;
  text-align: center;
  font-size: 1.5em;
  padding: 0.5em;
  opacity: 0.0;
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 3;
  pointer-events: none;
}

.card {
  max-width: 100%;
  width: 200px;
  height: 250px;
}
</style>
