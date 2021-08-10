<template>
    <div class="event">
      <div class="event--type">{{event.type}}</div>
      <p>{{event.body}}</p>
    </div>
    <Hand :cards="event.responses" />
</template>

<script>
import state from '../../state';
import Hand from './Hand.vue'
export default {
  props: ['event'],
  data() {
    return {
      state
    };
  },
  components: {
    Hand,
  },
}
</script>

<style>
.event {
  text-align: center;
  border: 1px solid #888;
  padding: 0.25em 0.5em 1em;
  flex: 1;
  margin: 0 1em;
  background: #8E905D;
}
.event--type {
  font-size: 0.75em;
}
</style>
