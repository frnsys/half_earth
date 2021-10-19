<template>
<li class="item" :key="item.id" :id="item.id" ref="root">
  <Flags :invalid="invalid" :questions="questions" />
  <button class="edit-toggle" @click="toggleEditing">{{ this.editing ? '⮪' : '✎'}}</button>
  <template v-if="editing">
    <div>
      <label>
        Name
        <Tip>The name of the industry.</Tip>
      </label>
      <input class="title" type="text" placeholder="Name" v-model="localData.name" :class="flags('name')" />
    </div>
    <div class="field-group">
      <Resources :resources="localData.resources" @update="saveData('resources', $event)"/>
      <Byproducts :byproducts="localData.byproducts" @update="saveData('byproducts', $event)"/>
    </div>

    <Notes :notes="localData.notes" @blur="saveNotes" />
  </template>

  <div v-else class="process-summary item-summary">
    <div class="item-summary-title" v-if="localData.name">{{localData.name}}</div>
    <div class="item-summary-title invalid" v-else>[MISSING NAME]</div>
    <div>
      <h5 class="kinds-summary-label">Per low-income-capita (LIC) per year:</h5>
      <ResourcesSummary :resources="localData.resources" />
      <ByproductsSummary :byproducts="localData.byproducts" :required="false" />
    </div>
    <div class="item-summary-notes" v-if="localData.notes" v-html="notesHtml"></div>
  </div>
</li>
</template>

<script>
import ItemMixin from './ItemMixin';
export default {
  mixins: [ItemMixin]
};
</script>
