<template>
<div>
  <ul v-if="validByproducts.length > 0" class="kind-summaries">
    <li v-for="k in validByproducts" class="summary-pill">
      <div>{{k}}</div>
      <div>{{localData[k]}} {{BYPRODUCTS[k]}}</div>
    </li>
  </ul>
  <div v-else>
    <div class="missing-defined">Missing Byproducts</div>
  </div>
</div>
</template>

<script>
import consts from '../../consts';

export default {
  props: ['byproducts'],
  data() {
    return {
      localData: Object.assign({}, this.byproducts)
    };
  },
  computed: {
    validByproducts() {
      return Object.keys(consts.BYPRODUCTS).filter((k) => {
        return this.localData[k] !== undefined && this.localData[k] !== '';
      });
    }
  }
}
</script>
