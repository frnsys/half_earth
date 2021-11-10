<template>
<div class="outputs-summary">
  <ul v-if="validOutputs.length > 0" class="kind-summaries">
    <li v-for="k in validOutputs" class="summary-pill">
      <div>{{k}}</div>
      <div>{{outputs[k]}} {{OUTPUTS[k]}}</div>
    </li>
  </ul>
  <div v-else>
    <div class="missing-defined">Missing Outputs</div>
  </div>
</div>
</template>

<script>
import consts from '../../consts';

export default {
  props: ['outputs'],
  computed: {
    validOutputs() {
      return Object.keys(consts.OUTPUTS).filter((k) => {
        return this.outputs[k] !== undefined && this.outputs[k] !== '';
      });
    }
  }
}
</script>
