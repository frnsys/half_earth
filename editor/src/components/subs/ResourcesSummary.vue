<template>
<div class="resources-summary">
  <ul v-if="validResources.length > 0" class="kind-summaries">
    <li v-for="k in validResources" class="summary-pill">
      <div>{{k}}</div>
      <div>{{localData[k]}} {{RESOURCES[k]}}</div>
    </li>
  </ul>
  <div v-else>
    <div class="missing-defined">Missing Resources</div>
  </div>
</div>
</template>

<script>
import consts from '../../consts';

export default {
  props: ['resources'],
  data() {
    return {
      localData: Object.assign({}, this.resources)
    };
  },
  computed: {
    validResources() {
      return Object.keys(consts.RESOURCES).filter((k) => {
        return this.localData[k] !== undefined && this.localData[k] !== '';
      });
    }
  }
}
</script>
