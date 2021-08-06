<template>
<Card :class="{'in-progress': plot.project && plot.project.yearsLeft > 0}">
  <div v-if="plot.project">
    <div class="flip" @click="flip">⮌</div>
  </div>
  <div v-if="plot.project && !flipped">
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
      <div>
        <span v-for="(v, k) in project.base.operation.impacts">
          <b>{{VARI_ICONS[k]}}</b>:{{v}}/year
        </span>
      </div>
    </div>
    <div class="details">
      <b>Destruction</b>
      <div>{{plot.project.base.destruction.years}} years</div>
      <span v-for="(v, k) in plot.project.base.destruction.resources">
        <b>{{k}}</b>:{{v}}/year
      </span>
      <div>
        <span v-for="(v, k) in project.base.destruction.impacts">
          <b>{{VARI_ICONS[k]}}</b>:{{v}}/year
        </span>
      </div>
    </div>

    <div class="actions">
      <slot name="actions"></slot>
    </div>
  </div>

  <div v-else>
    <div class="plot-type">{{plot.type}}</div>
    <div class="plot-prop" v-for="(d, vari) in plot.props">
      <div>{{PLOT_ICONS[vari]}}{{d}}</div>
      <div class="plot-prop--name">{{PLOT_ABBREV[vari]}}</div>
    </div>
  </div>

  <div class="meta meta-bot">
    <div class="toxic" v-if="plot.toxic">☠Toxic</div>
  </div>
</Card>
</template>

<script>
import Card from '../Card.vue';
export default {
  props: ['plot'],
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

<style>
.plot-prop {
	text-align: center;
	padding: 0.5em;
	display: inline-block;
	width: 40%;
	margin: 0.1em;
}

.plot-type {
	text-align: center;
	border: 1px dotted #ccc;
	padding: 0 0.5em;
	background: #f0f0f0;
}

.plot-prop--name {
  color: #888;
}
</style>
