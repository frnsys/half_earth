<template>
<li class="item" :key="item.id" :id="item.id" ref="root">
  <Flags :invalid="invalid" :questions="questions" />
  <div class="indicators">
    <div class="indicator indicator--missing" v-if="invalid.length > 0">Missing data</div>
    <div class="indicator indicator--question" v-if="questions.length > 0">Question(s)</div>
  </div>
  <div>
    <label>
      Name
      <Tip>Name of the initiative/policy/research.</Tip>
    </label>
    <input class="title" type="text" placeholder="Name" v-model="localData.name" @blur="save" :class="flags('name')" />
  </div>
  <fieldset>
    <div>
      <label>
        Type
        <Tip>The type of project.</Tip>
      </label>
      <select v-model="localData.type" @change="save" :class="flags('type')">
        <option value="Initiative">Initiative</option>
        <option value="Policy">Policy</option>
        <option value="Research">Research</option>
      </select>
    </div>
    <div class="checkbox">
      <label :for="`${item.id}_ongoing`">
        Ongoing
        <Tip>Is this a one-and-done project, or does it need continued maintenance?</Tip>
      </label>
      <input type="checkbox" :id="`${item.id}_ongoing`" v-model="localData.ongoing" @change="save">
    </div>
  </fieldset>
  <div>
    <label>
      Description
      <Tip>A 1-2 sentence description of the project.</Tip>
    </label>
    <textarea v-model="localData.description" placeholder="A brief description" @blur="save" :class="flags('description')"/>
  </div>
  <div class="field-group">
    <h3>Implementation (per year)</h3>
    <Resources :resources="localData.construction" @update="saveData('construction', $event)"/>
    <Byproducts :byproducts="localData.construction_byproducts" @update="saveData('construction_byproducts', $event)"/>
  </div>
  <div class="field-group" v-if="localData.ongoing">
    <h3>Maintenance (per year)</h3>
    <Resources :resources="localData.maintenance" @update="saveData('maintenance', $event)"/>
    <Byproducts :byproducts="localData.maintenance_byproducts" @update="saveData('maintenance_byproducts', $event)"/>
  </div>

  <Effects :toggle="true" :effects="localData.effects" @update="saveData('effects', $event)" />

  <div>
    <label>
      Flavor Text/Dialogue
      <Tip>Possible dialogue or other flavor to text to accompany the policy (e.g.an advisor giving their take on the policy when you first select/unlock it).</Tip>
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
      return ['name', 'description', 'type'];
    },
    questionKeys() {
      return ['name', 'description'];
    },
  },
  mixins: [ItemMixin]
};
</script>
