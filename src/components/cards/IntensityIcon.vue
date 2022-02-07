<template>
<div class="card-icon">
  <img :src="icons[resource]"/>
  <IntensityBar :intensity="intensity" :invert="invert" :max="max" />
</div>
</template>

<script>
import IntensityBar from './IntensityBar.vue';
import intensity from '/src/display/intensity';

export default {
  props: {
    resource: String,
    intensity: Number,
    invert: Boolean,
    max: {
      type: Number,
      default: 4
    }
  },
  components: {
    IntensityBar
  }
}
</script>
