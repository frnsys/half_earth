<template>
  <div class="indicators">
    <div class="indicator indicator--missing" v-if="invalid.length > 0">Missing data</div>
    <div class="indicator indicator--question" v-if="questions.length > 0">Question(s)</div>
  </div>
</template>

<script>
export default {
  props: ['invalid', 'questions'],
};
</script>

<style>
.indicators {
  position: absolute;
  left: calc(100% + 0.5em);
}
.indicator {
  font-size: 0.75em;
  padding: 0.25em;
  white-space: nowrap;
  border-radius: 0.25em;
  font-weight: bold;
  margin-bottom: 0.25em;
  display: inline-block;
}
.indicator--missing {
  color: #fff;
  background: #f54242;
  border: 1px solid #a22727;
}
.indicator--question {
  color: #000;
  background: #efcf40;
  border: 1px solid #927c18;
}
</style>
