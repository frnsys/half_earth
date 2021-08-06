<template>
  <Planning v-if="state.phase == 'PLANNING'" />
  <Stream v-else-if="state.phase == 'IMPLEMENTATION'" />
  <Report v-else-if="state.phase == 'REPORT'" />
</template>

<script>
import state from '../../state';
import Planning from './Planning.vue'
import Stream from './Stream.vue'
import Report from './Report.vue'
export default {
  data() {
    return {
      state
    };
  },
  components: {
    Planning,
    Stream,
    Report
  }
}
</script>
