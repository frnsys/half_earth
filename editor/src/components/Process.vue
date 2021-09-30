<template>
<li class="item" :id="item.id" ref="root">
  <div>
    <label>
      Name
      <Tip>The name of the process.</Tip>
    </label>
    <input class="title" type="text" placeholder="Name" v-model="localData.name" @blur="save" :class="flags('name')" />
  </div>
  <div>
    <label>
      Output
      <Tip>What this output produces.</Tip>
    </label>
    <select v-model="localData.output" @change="save" :class="flags('output')">
      <option v-for="k in Object.keys(OUTPUTS)">{{k}} ({{OUTPUTS[k]}})</option>
    </select>
  </div>
  <div>
    <label>
      Description
      <Tip>A 1-2 sentence description of the process.</Tip>
    </label>
    <input type="text" placeholder="Description" v-model="localData.description" @blur="save" :class="flags('description')" />
  </div>

  <div class="field-group">
    <div>
      <label>
        Resource Requirements
        <Tip>What resources are required per unit output of the process.</Tip>
      </label>
      <Resources :resources="localData.reqs" @update="saveData('reqs', $event)"/>
    </div>
    <div>
      <label>
        Byproducts
        <Tip>The byproducts per unit output of the process.</Tip>
      </label>
      <Byproducts :byproducts="localData.byproducts" @update="saveData('byproducts', $event)"/>
    </div>
  </div>

  <div>
    <label>
      Process Features
      <Tip>Special flags indicating additional process features/details. Used by (for example) events.</Tip>
    </label>
    <div class="checkbox" v-for="k in Object.keys(PROCESS_FEATURES)">
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

