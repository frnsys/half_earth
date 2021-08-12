<template>
<div class="event">
  <div class="event--type">{{event.type}}</div>
  <p>{{event.body}}</p>
</div>
</template>

<script>
import state from '../../state';
export default {
  props: ['event'],
  data() {
    return {
      state
    };
  },
}
</script>

<style>
.event {
  text-align: center;
  border: 1px solid #000;
  padding: 0.25em 0.5em 1em;
  flex: 1;
  margin: 0 1em;
  background: #fff;
  height: 120px;
}
.event--type {
  font-size: 0.75em;
}
</style>
