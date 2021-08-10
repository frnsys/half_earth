<template>
<div>
  <div class="flip" @click="flip">⮌</div>
  {{response.name}}

  <div v-if="!flipped">
    <div class="details">
      <div>
        <b>Costs: </b>
        <span v-for="(v, k) in response.costs">
          <b>{{k}}</b>:{{v < 0 ? '+' : '-'}}{{Math.abs(v)}}
        </span>
      </div>
      <div>
        <b>Impacts: </b>
        <span v-for="(v, k) in response.impacts">
          <b>{{VARI_ICONS[k]}}</b>:{{v > 0 ? '+' : '-'}}{{Math.abs(v)}}
        </span>
      </div>
    </div>
  </div>
  <div v-else>
    <em class="notes">Reminder text etc</em>
  </div>
</div>
</template>

<script>
export default {
  props: ['response'],
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
}
</script>
