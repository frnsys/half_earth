<template>
<li class="item" :key="item.id" :id="item.id" ref="root">
  <fieldset>
    <div class="flag-name">
      <label>
        Name
        <Tip>The flag name.</Tip>
      </label>
      <input type="text" placeholder="Flag name" v-model="localData.name" @blur="save" :class="flags('name')" />
    </div>
    <div class="flag-desc">
      <label>
        Description
        <Tip>Describe what this flag represents and how it's set (if not through an Effect)</Tip>
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
.flag-name {
  flex: 0.4;
  justify-content: normal;
}
.flag-name input {
  font-family: monospace;
}
.flag-desc textarea {
  font-size: 0.8em;
}
</style>
