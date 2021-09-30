<template>
<li class="item" :key="item.id" :id="item.id" ref="root">
  <div>
    <label>
      Year
      <Tip>The starting year.</Tip>
    </label>
    <input class="title" type="text" placeholder="Year" v-model="localData.year" @blur="save" :class="flags('year')" />
  </div>
  <fieldset>
    <div>
      <label>
        Emissions (GtCO2eq, or per-GHG?)
        <Tip>Starting global emissions.</Tip>
      </label>
      <input v-model="localData.emissions" type="number" min="0" @blur="save" :class="flags('emissions')">
    </div>
    <div>
      <label>
        Atmospheric GHG (ppm)
        <Tip>Starting atmospheric GHGs.</Tip>
      </label>
      <input v-model="localData.atmospheric_ghg" type="number" min="0" @blur="save" :class="flags('atmospheric_ghg')">
    </div>
    <div>
      <label>
        Biodiversity (unit?)
        <Tip>Starting biodiversity.</Tip>
      </label>
      <input v-model="localData.biodiversity" type="number" min="0" @blur="save" :class="flags('biodiversity')">
    </div>
    <div>
      <label>
        Temperature (C)
        <Tip>Starting global temperature average.</Tip>
      </label>
      <input v-model="localData.temperature" type="number" min="0" @blur="save" :class="flags('temperature')">
    </div>
    <div>
      <label>
        Ozone Damage (% eroded)
        <Tip>Starting ozone layer damage.</Tip>
      </label>
      <input v-model="localData.ozone_damage" type="number" min="0" @blur="save" :class="flags('ozone_damage')">
    </div>
  </fieldset>

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
      return ['year', 'emissions', 'atmospheric_ghg', 'biodiversity', 'temperature', 'ozone_damage'];
    },
    questionKeys() {
      return ['year'];
    },
  },
  mixins: [ItemMixin]
};
</script>
