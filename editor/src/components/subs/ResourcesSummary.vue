<template>
<div class="resources-summary">
  <ul v-if="validResources.length > 0" class="kind-summaries">
    <li v-for="k in validResources" class="summary-pill">
      <div>{{k}}</div>
      <div>{{resources[k]}} {{RESOURCES[k]}}</div>
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
  computed: {
    validResources() {
      let resources = this.resources || {};
      return Object.keys(consts.RESOURCES).filter((k) => {
        return resources[k] !== undefined && resources[k] !== '';
      });
    }
  }
}
</script>
