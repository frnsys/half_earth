<template>
<div class="project">
  <div class="flip" @click="flip">⮌</div>

  {{project.name}}

  <div v-if="!flipped">
    <div class="details">
      <b>Construction</b>:
      <span>{{project.construction.years ? `${project.construction.years} years` : '🎲'}}</span>
      <span v-for="(v, k) in project.construction.resources">
        <b>{{k}}</b>:{{v > 0 ? '+' : '-'}}{{Math.abs(v)}}/year
      </span>
      <span v-for="(v, k) in project.construction.impacts">
        <b>{{VARI_ICONS[k]}}</b>:{{v > 0 ? '+' : '-'}}{{Math.abs(v)}}/year
      </span>
    </div>

    <div class="details" v-if="project.type !== PROJECT_TYPE.RESEARCH">
      <b>Operation</b>:
      <span v-for="(v, k) in project.operation.resources">
        <b>{{k}}</b>:{{v > 0 ? '+' : '-'}}{{Math.abs(v)}}/year
      </span>
      <span v-for="(v, k) in project.operation.impacts">
        <b>{{VARI_ICONS[k]}}</b>:{{v > 0 ? '+' : '-'}}{{Math.abs(v)}}/year
      </span>
    </div>
  </div>
  <div v-else>(more details)</div>
</div>
</template>

<script>
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
}
</script>

<style>
.project {
  border: 1px solid #000;
  cursor: pointer;
  position: relative;
  margin: 0.5em 0;
  padding: 0.25em 0.5em;
}
.project:hover {
  background: #f0f0f0;
}
.project.selected {
  border: 1px solid #000;
  box-shadow: 3px 3px 0 rgba(0,0,0,0.5);
}
.project.selected::before {
  content: '✔️';
  position: absolute;
  top: -0.5em;
  left: -0.5em;
  font-size: 0.75em;
}
.project .details span {
  margin-left: 0.25em;
}

.flip {
  position: absolute;
  right: 0.5em;
  top: 0;
  opacity: 0;
  z-index: 1;
}
.project:hover .flip {
  opacity: 0.5;
}
.project:hover .flip:hover {
  opacity: 1;
}
</style>
