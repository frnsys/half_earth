<template>
    <div id="hand">
        <swiper
            :slides-per-view="3"
            :space-between="50"
            :centered-slides=true
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
import Card from './Card.vue'
import SwiperCore, { A11y } from 'swiper';
import { Swiper, SwiperSlide } from 'swiper/vue';
import 'swiper/swiper.scss';
SwiperCore.use([A11y]);

const params = {
  stretch: 0,
  depth: 100,
  scale: 0.8,
  modifier: 1
};

export default {
    props: {
        cards: Array,
    },
    data() {
        return {
            swiper: null,
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
        onDrag(card) {
            const targetScale = 1.2;
            const box = card.$el.getBoundingClientRect();
            const yCenter = window.innerHeight/2;
            const yCenterOffset = yCenter - box.height/2;
            const distToCenter = yCenterOffset - box.top;
            const pDistToCenter = distToCenter/yCenter;
            const p = (1 + pDistToCenter);
            const scale = Math.min(1 + (targetScale - 1) * p, targetScale);
            card.$el.style.transform = `scale(${scale})`;

            const opacity = Math.min(1 * p, 1);
            this.$refs.overlay.style.opacity = opacity;
            this.$refs.implement.style.opacity = opacity * 0.2;

            const $implement = this.$refs.implement;
            if (box.top < $implement.offsetHeight) {
                $implement.style.opacity = 1.0;
            } else {
                $implement.style.opacity = 0.2;
            }
        },
        onDragStart() {
            this.swiper.disable();
        },
        onDragStop(card) {
            const $card = card.$el;
            const box = $card.getBoundingClientRect();
            const $implement = this.$refs.implement;
            let played = false;
            if (box.top < $implement.offsetHeight) {
                // TODO card is played
                alert("played card");
                played = true;
            }

            const yCenter = window.innerHeight/2;
            const yCenterOffset = yCenter - box.height/2;
            const distToCenter = yCenterOffset - box.top;
            const pDistToCenter = distToCenter/yCenter;

            // Preview card
            if (Math.abs(pDistToCenter) < 0.1 && !played) {
                $card.style.left = 0;
                $card.style.top = `${parseInt($card.style.top) + distToCenter}px`;

            // Reset card to hand
            } else {
                const $overlay = this.$refs.overlay;
                $overlay.style.transition = 'all 0.2s';
                $overlay.style.opacity = 0;
                $implement.style.opacity = 0.0;
                this.overlayTimeout = setTimeout(() => {
                    $overlay.style.transition = '';
                }, 200);

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

        setTranslate(swiper) {
            swiper.updateSlidesOffset();
            const { width: swiperWidth, height: swiperHeight, slides, slidesSizesGrid } = swiper;
            /* const isHorizontal = swiper.isHorizontal(); */
            const isHorizontal = true;
            const transform = swiper.translate;
            const center = isHorizontal ? -transform + swiperWidth / 2 : -transform + swiperHeight / 2;
            const translate = params.depth;
            // Each slide offset from center
            for (let i = 0, length = slides.length; i < length; i += 1) {
              const $slideEl = slides.eq(i);
              const slideSize = slidesSizesGrid[i];
              const slideOffset = $slideEl[0].swiperSlideOffset;
              const offsetMultiplier =
                ((center - slideOffset - slideSize / 2) / slideSize) * params.modifier;

              let translateZ = -translate * Math.abs(offsetMultiplier);

              let stretch = params.stretch;
              // Allow percentage to make a relative stretch for responsive sliders
              if (typeof stretch === 'string' && stretch.indexOf('%') !== -1) {
                stretch = (parseFloat(params.stretch) / 100) * slideSize;
              }
              let translateY = isHorizontal ? 0 : stretch * offsetMultiplier;
              let translateX = isHorizontal ? stretch * offsetMultiplier : 0;

              let scale = 1 - (1 - params.scale) * Math.abs(offsetMultiplier);

              // Fix for ultra small values
              if (Math.abs(translateX) < 0.001) translateX = 0;
              if (Math.abs(translateY) < 0.001) translateY = 0;
              if (Math.abs(translateZ) < 0.001) translateZ = 0;
              if (Math.abs(scale) < 0.001) scale = 0;

              const slideTransform = `translate3d(${translateX}px,${translateY}px,${translateZ}px) scale(${scale})`;

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
    position: fixed;
    left: 0;
    bottom: -180px;
    right: 0;
    z-index: 2;
}
.swiper-container {
    overflow: visible;
}
.swiper-slide {
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
    margin-bottom: 1.5em;
    background: rgba(0,0,0,0.5);
    opacity: 0.0;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 3;
}
</style>
