<template>
<li class="item" :id="item.id" ref="root">
  <Flags :invalid="localData._validation.invalid" :questions="localData._validation.questions" />
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
  <Notes :notes="localData.notes" @blur="saveNotes" />
  <div class="additional-actions">
    <button @click="delete">Delete</button>
  </div>
</li>
</template>

<script>
import ItemMixin from './ItemMixin';
export default {
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
