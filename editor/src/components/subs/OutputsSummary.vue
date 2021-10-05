<template>
<div>
  <ul v-if="validOutputs.length > 0" class="kind-summaries">
    <li v-for="k in validOutputs" class="summary-pill">
      <div>{{k}}</div>
      <div>{{localData[k]}} {{OUTPUTS[k]}}</div>
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
  data() {
    return {
      localData: Object.assign({}, this.outputs)
    };
  },
  computed: {
    validOutputs() {
      return Object.keys(consts.OUTPUTS).filter((k) => {
        return this.localData[k] !== undefined && this.localData[k] !== '';
      });
    }
  }
}
</script>
