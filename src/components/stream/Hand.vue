<template>
  <div id="hand">
    <Cards>
      <li v-for="(card, idx) in cards" :key="card.id">
        <Card
          @onDragStart="onDragStart"
          @onDragStop="onDragStop"
          @onDrag="onDrag">
          <Response :response="card" />
        </Card>
      </li>
    </Cards>
  </div>
  <div id="playzone" ref="playzone">Play</div>
</template>

<script>
import util from '../../util';
import Card from './Card.vue'
import Cards from '../Cards.vue'
import Response from './Response.vue'

// TODO basically none of this is being used right now
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
  components: {
    Card,
    Cards,
    Response,
  },
  methods: {
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
    }
  }
};
</script>

<style>
</style>
