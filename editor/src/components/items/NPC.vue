<template>
<li class="item" :id="item.id" ref="root">
  <Flags :invalid="localData._validation.invalid" :questions="localData._validation.questions" />
  <button class="edit-toggle" @click="toggleEditing">{{ this.editing ? '⮪' : '✎'}}</button>
  <template v-if="editing">
    <div>
      <label>
        Name
        <Tip>Name of the NPC.</Tip>
      </label>
      <input class="title" type="text" placeholder="Name" v-model="localData.name" :class="flags('name')" />
    </div>
    <fieldset>
      <div>
        <label>
          Description
          <Tip>Describe the effects of having this NPC in your coalition.</Tip>
        </label>
        <textarea v-model="localData.description" placeholder="A brief description" :class="flags('description')"/>
      </div>
      <div class="checkbox">
        <label :for="`${item.id}_locked`">
          Locked
          <Tip>Is this NPC available to the player at the start?</Tip>
        </label>
        <input type="checkbox" :id="`${item.id}_locked`" v-model="localData.locked">
      </div>
    </fieldset>
    <Notes :notes="localData.notes" @blur="saveNotes" />

    <div class="additional-actions">
      <button @click="delete">Delete</button>
    </div>
  </template>

  <div v-else class="npc-summary item-summary">
    <div class="item-meta">
      <div class="meta-pill" v-if="localData.locked" :class="flags('locked')">Locked{{flags('locked').invalid ? ' MISSING UNLOCKER' : ''}}</div>
      <div class="meta-pill" v-else-if="!localData.locked && flags('locked').invalid" :class="flags('locked')">UNLOCKABLE BUT NOT LOCKED</div>
    </div>
    <div class="item-summary-title" v-if="localData.name">{{localData.name}}</div>
    <div class="item-summary-title invalid" v-else>[MISSING NAME]</div>
    <p class="item-summary-desc" v-if="localData.description" v-html="descriptionHtml"></p>
    <p class="item-summary-desc invalid" v-else>[MISSING DESCRIPTION]</p>
  </div>
</li>
</template>

<script>
import uuid from '../../uuid';
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
