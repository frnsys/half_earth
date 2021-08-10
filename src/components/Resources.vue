<template>
<div class="resources">
  <b>Resources:</b>
  <span v-for="(d, vari) in state.player.resources">
    <b>{{vari}}</b>:{{d.value}}
  </span>
</div>
</template>

<script>
import state from '../state';
export default {
  data() {
    return {
      state,
    };
  }
};
</script>
