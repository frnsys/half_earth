<template>
  <Planning v-if="state.phase == 'PLANNING'" />
  <Implementation v-else-if="state.phase == 'IMPLEMENTATION'" />
  <Report v-else-if="state.phase == 'REPORT'" />
</template>

<script>
import state from '../../state';
import Planning from './Planning.vue'
import Implementation from './Implementation.vue'
import Report from './Report.vue'
export default {
  data() {
    return {
      state
    };
  },
  components: {
    Planning,
    Implementation,
    Report
  }
}
</script>
