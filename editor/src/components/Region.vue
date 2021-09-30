<template>
<li class="item" :id="item.id" ref="root">
  <div>
    <label>
      Name
      <Tip>The name of the region.</Tip>
    </label>
    <input class="title" type="text" placeholder="Name" v-model="localData.name" @blur="save" :class="flags('name')" />
  </div>
  <div>
    <label>
      Countries
      <Tip>The countries that are aggreated to form this region.</Tip>
    </label>
    <input type="text" placeholder="Countries" v-model="localData.countries" @blur="save" :class="flags('countries')" />
  </div>
  <fieldset>
    <div>
      <label>
        Satiety
        <Tip>Starting "satiety", a catch-all for none-baseline-survival satisfication, like agency, community, etc, from 0 to 1, with 0 being a region filled with isolated lonely people and no social cohesion, and 1 being a communist utopia.</Tip>
      </label>
      <input v-model="localData.satiety" type="number" min="0">
    </div>
    <div>
      <label>
        Health
        <Tip>Starting public health, from 0 to 1, with 1 being everyone in perfect health with perfect access to top-quality healthcare and 0 being no healthcare system whatsoever amidst widespread pollution and contamination.</Tip>
      </label>
      <input v-model="localData.health" type="number" min="0">
    </div>
    <div>
      <label>
        Safety
        <Tip>Starting public safety, from 0 to 1, with 1 meaning no one is ever worried about crime or harm, and 0 means everyone is living in constant fear.</Tip>
      </label>
      <input v-model="localData.safety" type="number" min="0">
    </div>
    <div>
      <label>
        Outlook
        <Tip>Starting outlook, from 0 to 1, with 1 meaning people are excited and optimistic about the future, and 0 meaning a region full of hopeless nihilists.</Tip>
      </label>
      <input v-model="localData.outlook" type="number" min="0">
    </div>
  </fieldset>
  <div class="field-group">
    <h3>Starting Per-Capita Demand</h3>
    <div>
      <label>
        Output Demand
        <Tip>Per-capita demand captured by procsss outputs.</Tip>
      </label>
      <Outputs :outputs="localData.demand" @update="saveData('demand', $event)"/>
    </div>
    <div>
      <label>
        Everything Else
        <Tip>Other per-capita demand not captured by the above.</Tip>
      </label>
      <Resources :resources="localData.other_demand" @update="saveData('other_demand', $event)"/>
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
      return ['name', 'description', 'countries', 'satiety', 'safety', 'health', 'outlook'];
    },
    questionKeys() {
      return ['name', 'description', 'countries'];
    },
  },
  mixins: [ItemMixin]
};
</script>
