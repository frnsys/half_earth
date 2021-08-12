<template>
<div class="fader">
  <div v-for="pip in pips"
    @click="() => setValue(pip.value)"
    class="fader--pip"
    :style="{background:pip.color}"
    :class="{
      current: pip.current
    }">
      <div class="fader--pip--label" v-if="pip.label">{{pip.label}}</div>
    </div>
</div>
</template>

<script>
const colorA = [235, 55, 52];
const colorB = [61, 191, 66];

function colorsLerp(color1, color2, p) {
    var w1 = p;
    var w2 = 1 - w1;
    return [
      Math.round(color1[0] * w1 + color2[0] * w2),
      Math.round(color1[1] * w1 + color2[1] * w2),
      Math.round(color1[2] * w1 + color2[2] * w2)
    ];
}

export default {
  props: {
    steps: Number,
    value: Number,
    current: Number,
    reverse: Boolean,
    maxLabel: String,
    minLabel: String
  },
  computed: {
    pips() {
      let startColor = this.reverse ? colorB : colorA;
      let endColor = this.reverse ? colorA : colorB;
      return [...Array(this.steps)].map((_, i) => {
        let value = this.steps - i + 1; // Make 1-indexed for values
        let current = value == this.current;
        let selected = (value >= this.value && this.reverse) || (value <= this.value && !this.reverse);
        let color = colorsLerp(startColor, endColor, i/this.steps);
        let label = null;
        if (i == 0) {
          label = `< ${this.reverse ? this.minLabel : this.maxLabel}`;
        } else if (i == this.steps - 1) {
          label = `< ${this.reverse ? this.maxLabel : this.minLabel}`;
        }
        return {
          value, current, label,
          color: selected ? `rgb(${color.join()})` : '#ededed'
        }
      });
    },
  },
  methods: {
    setValue(val) {
      this.$emit('change', val);
    }
  }
}
</script>

<style>
.fader--pip {
  background: #EDEDED;
  width: 80px;
  height: 16px;
  margin: 0.2em auto;
  cursor: pointer;
  position: relative;
}
.fader--pip:hover {
  opacity: 0.8;
}
.fader--pip.selected {
  background: #43CC70;
}
.fader--pip.current::before {
  font-size: 0.6em;
  content: "Current >";
  position: absolute;
  left: -75%;
  line-height: 1.7;
}
.fader--pip--label {
  position: absolute;
  left: 100%;
  font-size: 0.6em;
  line-height: 1.7;
  padding-left: 0.5em;
  width: 80px;
}
</style>
