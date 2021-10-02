<template>
<div>
  <label>
    Resources
    <button @click="() => this.editing = !this.editing">{{ this.editing ? '⮪' : '✎'}}</button>
  </label>
  <div class="kind-quantities" v-if="editing">
    <div v-for="k in Object.keys(RESOURCES)">
      <label>{{k}} <span class="units">{{RESOURCES[k]}}</span></label>
      <input type="number" min="0"
        v-model="localData[k]"
        @change="$emit('update', localData)" />
    </div>
  </div>
  <ul v-else-if="validResources.length > 0" class="kind-summaries">
    <li v-for="k in validResources">
      <div class="kind-summary">
        <div class="kind-name">{{k}}</div>
        <div class="kind-value">{{localData[k]}} {{RESOURCES[k]}}</div>
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
  props: ['resources'],
  data() {
    return {
      editing: false,
      localData: Object.assign({}, this.resources)
    };
  },
  computed: {
    validResources() {
      return Object.keys(consts.RESOURCES).filter((k) => {
        return this.localData[k] !== undefined && this.localData[k] !== '' && this.localData[k] > 0;
      });
    }
  }
}
</script>
