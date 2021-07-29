<template>
<Card>
  <div class="flip" @click="flip">⮌</div>
  {{project.name}}

  <div v-if="!flipped">
    <div class="details">
      <b>Construction</b>
      <div>{{project.construction.years}} years</div>
      <div>
        <span v-for="(v, k) in project.construction.resources">
          <b>{{k}}</b>:{{v}}/year
        </span>
      </div>
    </div>
    <div class="details">
      <b>Operation</b>
      <div>
        <span v-for="(v, k) in project.operation.resources">
          <b>{{k}}</b>:{{v}}/year
        </span>
      </div>
    </div>
    <div class="actions">
      <slot name="actions"></slot>
    </div>
  </div>
  <div v-else>card back</div>
</Card>
</template>

<script>
import Card from './Card.vue';
export default {
  props: ['project'],
  data() {
    return {
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
