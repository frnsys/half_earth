<template>
<li class="item" :key="item.id" :id="item.id" ref="root">
  <div>
    <label>
      Name
      <Tip>The name of the process.</Tip>
    </label>
    <input class="title" type="text" placeholder="Name" v-model="localData.name" @blur="save" :class="flags('name')" />
  </div>
  <fieldset>
    <div>
      <label>
        Output
        <Tip>What this output produces.</Tip>
      </label>
      <select v-model="localData.output" @change="save" :class="flags('output')">
        <option v-for="k in Object.keys(OUTPUTS)">{{k}} ({{OUTPUTS[k]}})</option>
      </select>
    </div>
    <div class="checkbox">
      <label :for="`${item.id}_locked`">
        Locked
        <Tip>Is this process available to the player at the start?</Tip>
      </label>
      <input type="checkbox" :id="`${item.id}_locked`" v-model="localData.locked" @change="save">
    </div>
  </fieldset>
  <div>
    <label>
      Description
      <Tip>A 1-2 sentence description of the process.</Tip>
    </label>
    <input type="text" placeholder="Description" v-model="localData.description" @blur="save" :class="flags('description')" />
  </div>

  <div class="field-group">
    <Resources :resources="localData.reqs" @update="saveData('reqs', $event)"/>
    <Byproducts :byproducts="localData.byproducts" @update="saveData('byproducts', $event)"/>
  </div>

  <div>
    <label>
      Process Features
      <Tip>Special flags indicating additional process features/details. Used by (for example) events.</Tip>
    </label>
    <div class="checkbox-feature" v-for="k in Object.keys(PROCESS_FEATURES)">
      <input type="checkbox" :id="`${item.id}_${k}`">
      <label :for="`${item.id}_${k}`">{{k}}</label>
      <Tip>{{PROCESS_FEATURES[k]}}</Tip>
    </div>
  </div>

  <div>
    <label>
      Flavor Text/Dialogue
      <Tip>Advisor dialogue introducing the event.</Tip>
    </label>
    <textarea v-model="localData.flavor" placeholder="Flavor text and dialogue" @blur="save" />
  </div>

  <Notes :notes="localData.notes" @blur="saveNotes" />
</li>
</template>

<script>
import ItemMixin from './ItemMixin';

export default {
  computed: {
    validateKeys() {
      return ['name', 'description', 'output'];
    },
    questionKeys() {
      return ['name', 'description'];
    },
  },
  mixins: [ItemMixin]
};
</script>

<style>
.checkbox-feature {
	display: inline-block;
	background: #eee;
	padding: 0.1em 0.25em 0.2em 0.1em;
	border: 1px solid #aaa;
  margin-right: 1em;
  margin-bottom: 0.5em;
}
.checkbox-feature > input, .checkbox-feature > label {
  width: auto;
  display: inline;
}
.checkbox-feature .tip {
  font-size: 0.75em;
}
</style>
