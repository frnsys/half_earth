<template>
<div>
  <label>
    Byproducts
    <button @click="() => this.editing = !this.editing">{{ this.editing ? '⮪' : '✎'}}</button>
  </label>
  <div class="kind-quantities" v-if="editing">
    <div v-for="k in Object.keys(BYPRODUCTS)">
      <label>{{k}} <span class="units">{{BYPRODUCTS[k]}}</span></label>
      <input type="number" min="0"
        v-model="localData[k]"
        @change="$emit('update', localData)" />
    </div>
  </div>
  <ul v-else-if="validByproducts.length > 0" class="kind-summaries">
    <li v-for="k in validByproducts">
      <div class="kind-summary">
        <div class="kind-name">{{k}}</div>
        <div class="kind-value">{{localData[k]}} {{BYPRODUCTS[k]}}</div>
      </div>
    </li>
  </ul>
  <div v-else>
    <div class="missing-defined">None</div>
  </div>
</div>
</template>

<script>
import consts from '../../consts';

export default {
  props: ['byproducts'],
  data() {
    return {
      editing: false,
      localData: Object.assign({}, this.byproducts)
    };
  },
  computed: {
    validByproducts() {
      return Object.keys(consts.BYPRODUCTS).filter((k) => {
        return this.localData[k] !== undefined && this.localData[k] !== '' && this.localData[k] > 0;
      });
    }
  }
}
</script>
