<template>
<li class="item" :key="item.id" :id="item.id" ref="root">
  <fieldset>
    <div class="variable-name">
      <label>
        Name
        <Tip>The variable name.</Tip>
      </label>
      <input type="text" placeholder="Variable" v-model="localData.name" @blur="save" :class="flags('name')" />
    </div>
    <div class="variable-desc">
      <label>
        Description
        <Tip>Describe what the variable represents and its range.</Tip>
      </label>
      <textarea placeholder="Description" v-model="localData.desc" @blur="save" :class="flags('desc')" />
    </div>
  </fieldset>
</li>
</template>

<script>
import ItemMixin from './ItemMixin';

export default {
  computed: {
    validateKeys() {
      return ['name', 'desc'];
    },
    questionKeys() {
      return ['name', 'desc'];
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
.variable-desc textarea {
  font-size: 0.8em;
}
</style>
