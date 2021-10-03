<template>
<li class="item" :key="item.id" :id="item.id" ref="root">
  <fieldset>
    <div class="variable-name">
      <label>
        Name
        <Tip>The variable name.</Tip>
      </label>
      <input type="text" placeholder="Variable name" v-model="localData.name" @blur="save" :class="flags('name')" />
    </div>
    <div class="variable-values">
      <label>
        Values
        <Tip>A list of values the variable can have (one per line).</Tip>
      </label>
      <textarea placeholder="Values" v-model="localData.values" @blur="save" :class="flags('values')" />
    </div>
  </fieldset>
</li>
</template>

<script>
import ItemMixin from './ItemMixin';

export default {
  computed: {
    validateKeys() {
      return ['name', 'values'];
    },
    questionKeys() {
      return ['name', 'values'];
    },
  },
  mixins: [ItemMixin]
};
</script>

<style>
.variable-name {
  flex: 0.4;
  justify-content: normal;
}
.variable-name input {
  font-family: monospace;
}
.variable-values textarea {
  font-size: 0.8em;
}
</style>
