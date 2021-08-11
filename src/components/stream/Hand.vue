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
                <Card :card="card" :ref="setCardRef"
                    @onDragStart="onDragStart"
                    @onDragStop="onDragStop"
                    @onDrag="onDrag" />
            </swiper-slide>
        </swiper>
    </div>
    <div id="overlay" ref="overlay"></div>
    <div id="implement" ref="implement">Implement</div>
</template>

<script>
import util from '../../util';
import Card from './Card.vue'
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

const targetCardScale = 1.8;

const dragTarget = () => window.innerHeight/2;

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
    },
    beforeUpdate() {
        this.cardRefs = [];
    },
    methods: {
        setCardRef(el) {
            if (el) this.cardRefs.push(el);
        },
        setAdjacentCardOpacity(opacity) {
            if (this.swiper.activeIndex > 0) {
                this.cardRefs[this.swiper.activeIndex-1].$el.style.opacity = opacity;
            }
            if (this.swiper.activeIndex < this.cardRefs.length - 1) {
                this.cardRefs[this.swiper.activeIndex+1].$el.style.opacity = opacity;
            }
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
            this.$refs.overlay.style.opacity = opacity;
            this.$refs.overlay.style.display = 'block';
            this.$refs.implement.style.opacity = opacity * 0.2;
            this.setAdjacentCardOpacity(1 - opacity);

            const $implement = this.$refs.implement;
            if (box.top < 0) {
                $implement.style.opacity = box.top/(-box.height/3);
            }
        },
        onDragStart() {
            this.swiper.disable();
        },
        onDragStop(card) {
            const $card = card.$el;
            const box = $card.getBoundingClientRect();
            const $implement = this.$refs.implement;

            // Card must be pushed a third off screen to implement
            let played = false;
            if (box.top < -box.height/3) {
                // TODO card is played
                alert("played card");
                played = true;
            }

            const yCenter = window.innerHeight/2;
            const yCenterOffset = yCenter - box.height/2;
            const distToCenter = yCenterOffset - box.top;
            const pDistToCenter = distToCenter/yCenter;

            // Preview card
            // if dragged past (yCenter - 0.1) and not played
            if (pDistToCenter + 0.1 > 0 && !played) {
                $card.style.left = 0;
                $card.style.top = `${parseInt($card.style.top) + distToCenter}px`;
                util.updateTransform($card, {rotate: '0deg'});
                $card.classList.add('card-preview');

            // Reset card to hand
            } else {
                const $overlay = this.$refs.overlay;
                $overlay.style.transition = 'all 0.2s';
                $overlay.style.opacity = 0;
                $overlay.style.display = 'none';
                $implement.style.opacity = 0.0;
                $card.classList.remove('card-preview');
                this.overlayTimeout = setTimeout(() => {
                    $overlay.style.transition = '';
                }, 200);

                card.resetDrag();
                this.setAdjacentCardOpacity(1);
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
    clip-path: polygon(0 0, 100% 0, 100% 200px, 0% 200px);
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

#overlay {
    background: rgba(0,0,0,0.8);
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 1;
    opacity: 0;
    display: none;
}
#stage {
    display: flex;
    align-items: center;
    justify-content: space-around;
}
#implement {
    color: #fff;
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
