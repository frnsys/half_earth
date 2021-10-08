<template>
<li class="item" :key="item.id" :id="item.id" ref="root">
  <Flags :invalid="invalid" :questions="questions" />
  <fieldset>
    <div class="const-name">
      <label>
        Name
        <Tip>The const name.</Tip>
      </label>
      <input type="text" placeholder="Const name" v-model="localData.name" @blur="save" :class="flags('name')" />
    </div>
    <div class="const-value">
      <label>
        Value
        <Tip>The const's value.</Tip>
      </label>
      <input type="text" placeholder="Value" v-model="localData.value" @blur="save" :class="flags('value')" />
    </div>
  </fieldset>
  <Notes :notes="localData.notes" @blur="saveNotes" />
</li>
</template>

<script>
import ItemMixin from './ItemMixin';
export default {
  mixins: [ItemMixin]
};
</script>

<style>
.const-name {
  flex: 0.4;
  justify-content: normal;
}
.const-name input,
.const-value input {
  font-family: monospace;
}
</style>
