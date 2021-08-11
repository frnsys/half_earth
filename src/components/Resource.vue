<template>
<div class="resource-pip">
  {{VARI_ICONS[name]}}
  <div class="resource-pip--amount">
    <template v-for="i in Math.abs(value)">{{amountIcon}}</template>
  </div>
</div>
</template>

<script>
export default {
  props: {
    name: String,
    value: Number
  },
  computed: {
    amountIcon: {
      get: function() {
        return this.value < 0 ? '-' : '+';
      }
    }
  }
}
</script>

<style>
.resource-pip {
  position: relative;
  display: inline-block;
}
.resource-pip--amount {
  position: absolute;
  right: 0;
  top: 0;
  font-size: 0.5em;
}
</style>
