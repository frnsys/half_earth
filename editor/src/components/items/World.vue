<template>
<li class="item" :id="item.id" ref="root">
  <Flags :invalid="localData._validation.invalid" :questions="localData._validation.questions" />
  <button class="edit-toggle" @click="toggleEditing">{{ this.editing ? '⮪' : '✎'}}</button>
  <template v-if="editing">
    <div>
      <label>
        Year
        <Tip>The starting year.</Tip>
      </label>
      <input class="title" type="text" placeholder="Year" v-model="localData.year" :class="flags('year')" />
    </div>
    <fieldset>
      <div>
        <label>
          CO2 Emissions (Gt/y)
          <Tip>Starting global CO2 emissions.</Tip>
        </label>
        <input v-model="localData.co2_emissions" type="number" min="0" :class="flags('co2_emissions')">
      </div>
      <div>
        <label>
          CH4 Emissions (Mt/y)
          <Tip>Starting global CH4 emissions.</Tip>
        </label>
        <input v-model="localData.ch4_emissions" type="number" min="0" :class="flags('ch4_emissions')">
      </div>
      <div>
        <label>
          N2O Emissions (Mt/y)
          <Tip>Starting global N2O emissions.</Tip>
        </label>
        <input v-model="localData.n2o_emissions" type="number" min="0" :class="flags('n2o_emissions')">
      </div>
      <div>
        <label>
          Extinction Rate (e/msy)
          <Tip>Starting extinction rate.</Tip>
        </label>
        <input v-model="localData.extinction_rate" type="number" min="0" :class="flags('extinction_rate')">
      </div>
      <div>
        <label>
          Temperature (C)
          <Tip>Starting global temperature average.</Tip>
        </label>
        <input v-model="localData.temperature" type="number" min="0" :class="flags('temperature')">
      </div>
    </fieldset>

    <div>
      <label>
        Flavor Text/Dialogue
        <Tip>Advisor dialogue introducing the event.</Tip>
      </label>
      <textarea v-model="localData.flavor" placeholder="Flavor text and dialogue" />
    </div>

    <Notes :notes="localData.notes" @blur="saveNotes" />
  </template>

  <div v-else class="earth-summary item-summary">
    <div class="item-meta">
      <div class="meta-pill split-pill" :class="{invalid: !localData.temperature}">
        <div>Temp</div><div>+{{localData.temperature || 'MISSING'}} C</div>
      </div>
      <div class="meta-pill split-pill" :class="{invalid: !localData.extinction_rate}">
        <div>Ext.Rate</div><div>{{localData.extinction_rate || 'MISSING'}} e/msy</div>
      </div>
      <div class="meta-pill split-pill" :class="{invalid: !localData.co2_emissions}">
        <div>CO2</div><div>{{localData.co2_emissions || 'MISSING'}} Gt/y</div>
      </div>
      <div class="meta-pill split-pill" :class="{invalid: !localData.ch4_emissions}">
        <div>CH4</div><div>{{localData.ch4_emissions || 'MISSING'}} Mt/y</div>
      </div>
      <div class="meta-pill split-pill" :class="{invalid: !localData.n2o_emissions}">
        <div>N2O</div><div>{{localData.n2o_emissions || 'MISSING'}} Mt/y</div>
      </div>
      <div class="meta-pill split-pill">
        <div>All Emissions</div><div>{{emissions.toFixed(2)}} GtCO2eq/y</div>
      </div>
    </div>
    <div class="item-summary-title" v-if="localData.year">{{localData.year}}</div>
    <div class="item-summary-title invalid" v-else>[MISSING YEAR]</div>
    <div class="item-summary-flavor" v-if="localData.flavor" v-html="flavorHtml"></div>
    <div class="item-summary-notes" v-if="localData.notes" v-html="notesHtml"></div>
  </div>
</li>
</template>

<script>
import ItemMixin from './ItemMixin';

const ch4_gwp = 25;
const n2o_gwp = 298;

export default {
  computed: {
    emissions() {
      let co2 = (this.localData.co2_emissions || 0); // Gt/y
      let ch4 = (this.localData.ch4_emissions || 0); // Mt/y
      let n2o = (this.localData.n2o_emissions || 0); // Mt/y
      return co2 + (ch4 * ch4_gwp)/1e3 + (n2o * n2o_gwp)/1e3;
    }
  },
  mixins: [ItemMixin]
};
</script>
