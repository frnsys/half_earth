<template>
<li class="item" :id="item.id" ref="root">
  <Flags :invalid="localData._validation.invalid" :questions="localData._validation.questions" />
  <button class="edit-toggle" @click="toggleEditing">{{ this.editing ? '⮪' : '✎'}}</button>
  <template v-if="editing">
    <div>
      <label>
        Name
        <Tip>The name of the industry.</Tip>
      </label>
      <input class="title" type="text" placeholder="Name" v-model="localData.name" :class="flags('name')" />
    </div>
    <fieldset class="big-group">
      <div>
        <Image :image="localData.image" :dimensions="'360x240'" @update="saveData('image', $event)" />
      </div>
      <div>
        <Resources :resources="localData.resources" @update="saveData('resources', $event)"/>
        <Byproducts :byproducts="localData.byproducts" @update="saveData('byproducts', $event)"/>
      </div>
    </fieldset>

    <fieldset>
      <div>
        <label>
          Description
          <Tip>Describe this industry.</Tip>
        </label>
        <textarea v-model="localData.description" placeholder="A brief description" :class="flags('description')"/>
      </div>
    </fieldset>

    <Notes :notes="localData.notes" @blur="saveNotes" />

    <div class="additional-actions">
      <button @click="delete">Delete</button>
    </div>
  </template>

  <div v-else class="process-summary item-summary">
    <div class="item-summary-title" v-if="localData.name">{{localData.name}}</div>
    <div class="item-summary-title invalid" v-else>[MISSING NAME]</div>
    <fieldset class="big-group">
      <div class="item-summary-image" v-if="localData.image">
        <img class="image-preview" v-if="localData.image.image" :src="`/image/${localData.image.image}`"/>
        <div class="image-attribution">{{localData.image.attribution}}</div>
      </div>
      <div>
        <h5 class="kinds-summary-label">Per low-income-capita (LIC) per year:</h5>
        <ResourcesSummary :resources="localData.resources" />
        <ByproductsSummary :byproducts="localData.byproducts" :required="false" />
      </div>
    </fieldset>
    <p class="item-summary-desc" v-if="localData.description" v-html="descriptionHtml"></p>
    <p class="item-summary-desc invalid" v-else>[MISSING DESCRIPTION]</p>

    <div class="item-summary-notes" v-if="localData.notes" v-html="notesHtml"></div>
  </div>
</li>
</template>

<script>
import ItemMixin from './ItemMixin';
export default {
  computed: {
    descriptionHtml() {
      return this.localData.description.replaceAll('\n', '<br />');
    },
  },
  mixins: [ItemMixin]
};
</script>
