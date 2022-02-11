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
          Color
          <Tip>The color for this faction (used in parliament seats)</Tip>
        </label>
        <textarea v-model="localData.color" placeholder="Seat color" :class="flags('color')"/>
      </div>
      <div class="checkbox">
        <label :for="`${item.id}_locked`">
          Locked
          <Tip>Is this NPC available to the player at the start?</Tip>
        </label>
        <input type="checkbox" :id="`${item.id}_locked`" v-model="localData.locked">
      </div>
    </fieldset>
    <fieldset>
      <div>
        <label>
          Description
          <Tip>Describe this NPC.</Tip>
        </label>
        <textarea v-model="localData.description" placeholder="A brief description" :class="flags('description')"/>
      </div>
      <div>
        <label>
          Effects
          <Tip>Describe the effects of having this NPC in your coalition.</Tip>
        </label>
        <textarea v-model="localData.effects" placeholder="Describe the NPC's effects" :class="flags('effects')"/>
      </div>
    </fieldset>
    <fieldset>
      <div>
        <label>
          Likes
          <Tip>List this NPC's "likes"</Tip>
        </label>
        <textarea v-model="localData.likes" placeholder="This NPC's 'likes'" :class="flags('likes')"/>
      </div>
      <div>
        <label>
          Dislikes
          <Tip>List this NPC's "dislikes"</Tip>
        </label>
        <textarea v-model="localData.dislikes" placeholder="This NPC's 'dislikes'" :class="flags('dislikes')"/>
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
      <div class="meta-pill" :style="{background: localData.color}">{{localData.color}}</div>
    </div>
    <div class="item-summary-title" v-if="localData.name">{{localData.name}}</div>
    <div class="item-summary-title invalid" v-else>[MISSING NAME]</div>
    <p class="item-summary-desc" v-if="localData.effects" v-html="effectsHtml"></p>
    <p class="item-summary-desc invalid" v-else>[MISSING EFFECTS]</p>
    <p class="item-summary-desc" v-if="localData.description" v-html="descriptionHtml"></p>
    <p class="item-summary-desc invalid" v-else>[MISSING DESCRIPTION]</p>
    <fieldset class="likes-dislikes">
      <p class="item-summary-desc" v-if="localData.likes">Likes: {{localData.likes}}</p>
      <p class="item-summary-desc invalid" v-else>[MISSING LIKES]</p>
      <p class="item-summary-desc" v-if="localData.dislikes">Dislikes: {{localData.dislikes}}</p>
      <p class="item-summary-desc invalid" v-else>[MISSING DISLIKES]</p>
    </fieldset>

    <div class="item-summary-notes" v-if="localData.notes" v-html="notesHtml"></div>
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
    effectsHtml() {
      return this.localData.effects.replaceAll('\n', '<br />');
    },
  },
  mixins: [ItemMixin]
};
</script>

<style>
.likes-dislikes > p {
  flex: 1;
}
</style>
