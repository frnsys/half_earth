<template>
<Card :class="{'in-progress': plot.project && plot.project.yearsLeft > 0}">
  <div v-if="plot.project">
    <div class="flip" @click="flip">⮌</div>
    <div v-if="!flipped">
      <b>{{plot.project.base.name}}</b>
      <div class="details">
        <div v-if="plot.project.status == PROJECT_STATE.CONSTRUCTING">construction years left:{{plot.project.yearsLeft}}</div>
        <div v-else-if="plot.project.status == PROJECT_STATE.DESTRUCTING">destructing years left:{{plot.project.yearsLeft}}</div>
      </div>

      <div class="details">
        <b>Operation</b>
        <div>
          <span v-for="(v, k) in plot.project.base.operation.resources">
            <b>{{k}}</b>:{{v > 0 ? '-' : '+'}}{{Math.abs(v)}}/year
          </span>
        </div>
      </div>
      <div class="details">
        <b>Destruction</b>
        <div>{{plot.project.base.destruction.years}} years</div>
        <span v-for="(v, k) in plot.project.base.destruction.resources">
          <b>{{k}}</b>:{{v}}/year
        </span>
      </div>

      <div class="actions">
        <slot name="actions"></slot>
      </div>
    </div>
    <div v-else>
      <div v-for="(d, vari) in plot.props">{{vari}}:{{d}}</div>
    </div>
  </div>

  <div v-else>
    <div v-for="(d, vari) in plot.props">{{vari}}:{{d}}</div>
  </div>

  <div class="meta meta-bot">
    <div class="toxic" v-if="plot.toxic">☠Toxic</div>
  </div>
</Card>
</template>

<script>
import Card from './Card.vue';
import {PROJECT_STATE} from '../consts';
export default {
  props: ['plot'],
  data() {
    return {
      PROJECT_STATE,
      flipped: false
    };
  },
  methods: {
    flip(ev) {
      this.flipped = !this.flipped;
      ev.stopPropagation();
    }
  },
  components: {
    Card
  },
}
</script>
