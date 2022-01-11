<template>
<li class="item" :id="item.id" ref="root">
  <Flags :invalid="localData._validation.invalid" :questions="localData._validation.questions" />
  <button class="edit-toggle" @click="toggleEditing">{{ this.editing ? '⮪' : '✎'}}</button>
  <template v-if="editing">
    <div>
      <label>
        Name
        <Tip>The name of the region.</Tip>
      </label>
      <input class="title" type="text" placeholder="Name" v-model="localData.name" :class="flags('name')" />
    </div>
    <fieldset class="big-group">
      <div>
        <Image :image="localData.image" :dimensions="'360x240'" @update="saveData('image', $event)" />
      </div>
      <div>
        <div>
          <label>
            Latitude
            <Tip>The latitude that best describes this region</Tip>
          </label>
          <select v-model="localData.latitude" :class="flags('latitude')">
            <option v-for="k in LATITUDES" :value="k">{{k}}</option>
          </select>
        </div>
        <div>
          <label>
            Income Level
            <Tip>Starting income level for the region. This is used to scale per-capita impacts/demand.</Tip>
          </label>
          <select v-model="localData.income_level" :class="flags('income_level')">
            <option v-for="k in INCOME_LEVELS" :value="k">{{k}}</option>
          </select>
        </div>
        <div>
          <label>
            Health
            <Tip>Starting public health, from 0 to 1, with 1 being everyone in perfect health with perfect access to top-quality healthcare and 0 being no healthcare system whatsoever amidst widespread pollution and contamination.</Tip>
          </label>
          <input v-model="localData.health" type="number" min="0" :class="flags('health')">
        </div>
        <div>
          <label>
            Starting Population
            <Tip>Starting population</Tip>
          </label>
          <input v-model="localData.population" type="number" min="0" :class="flags('population')">
        </div>
        <div>
          <label>
            Starting Development Progress
            <Tip>Starting progress to the next income level</Tip>
          </label>
          <input v-model="localData.development" type="number" min="0" :class="flags('development')">
        </div>
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

    <div class="additional-actions">
      <button @click="delete">Delete</button>
    </div>
  </template>
  <div v-else class="region-summary item-summary">
    <div class="item-meta">
      <div class="meta-pill" :class="{invalid: !localData.latitude}">
        <div>{{localData.latitude || 'MISSING'}}</div>
      </div>
      <div class="meta-pill split-pill" :class="{invalid: !localData.income_level}">
        <div>Income Level</div><div>{{localData.income_level || 'MISSING'}}</div>
      </div>
      <div class="meta-pill split-pill" :class="{invalid: !localData.health}">
        <div>Health</div><div>{{localData.health || 'MISSING'}}</div>
      </div>
      <div class="meta-pill split-pill" :class="{invalid: !localData.population}">
        <div>Population</div><div>{{localData.population || 'MISSING'}}</div>
      </div>
      <div class="meta-pill split-pill" :class="{invalid: localData.development == undefined}">
        <div>Development</div><div>{{localData.development !== undefined ? localData.development : 'MISSING'}}</div>
      </div>
    </div>
    <div class="item-summary-title" v-if="localData.name">{{localData.name}}</div>
    <div class="item-summary-title invalid" v-else>[MISSING NAME]</div>
    <div class="item-summary-image" v-if="localData.image">
      <img class="image-preview" v-if="localData.image.image" :src="`/image/${localData.image.image}`"/>
      <div class="image-attribution">{{localData.image.attribution}}</div>
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
