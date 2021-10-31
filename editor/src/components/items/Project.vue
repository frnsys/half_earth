<template>
<li class="item" :key="item.id" :id="item.id" ref="root">
  <Flags :invalid="invalid" :questions="questions" />
  <button class="edit-toggle" @click="toggleEditing">{{ this.editing ? '⮪' : '✎'}}</button>
  <template v-if="editing">
    <div>
      <label>
        Name
        <Tip>Name of the initiative/policy/research.</Tip>
      </label>
      <input class="title" type="text" placeholder="Name" v-model="localData.name" :class="flags('name')" />
    </div>
    <fieldset>
      <div>
        <label>
          Type
          <Tip>The type of project.</Tip>
        </label>
        <select v-model="localData.type" :class="flags('type')">
          <option value="Initiative">Initiative</option>
          <option value="Policy">Policy</option>
          <option value="Research">Research</option>
        </select>
      </div>
      <div>
        <label>
          Cost/Years to Completion
          <Tip>Political capital cost for policies, otherwise years to completion. If the cost is dynamic, this is the multiplier for the output demand.</Tip>
        </label>
        <input type="number" v-model="localData.cost" :class="flags('cost')"/>
      </div>
      <div class="checkbox">
        <label :for="`${item.id}_dynamic_cost`">
          Dynamic Cost
          <Tip>Is the cost of this policy dynamic based on demand?</Tip>
        </label>
        <input type="checkbox" :id="`${item.id}_dynamic_cost`" v-model="localData.dynamic_cost">
      </div>
      <div class="checkbox" v-if="localData.dynamic_cost">
        <label :for="`${item.id}_dynamic_cost_demand`">
          Dynamic Cost Demand
          <Tip>The demand used to calculate the dynamic cost.</Tip>
        </label>
        <select v-model="localData.dynamic_cost_demand">
          <option v-for="k in Object.keys(OUTPUTS)" :value="k">{{k}}</option>
        </select>
      </div>
      <div class="checkbox">
        <label :for="`${item.id}_ongoing`">
          Ongoing
          <Tip>Is this a one-and-done project, or does it need continued maintenance?</Tip>
        </label>
        <input type="checkbox" :id="`${item.id}_ongoing`" v-model="localData.ongoing">
      </div>
      <div class="checkbox">
        <label :for="`${item.id}_locked`">
          Locked
          <Tip>Is this process available to the player at the start?</Tip>
        </label>
        <input type="checkbox" :id="`${item.id}_locked`" v-model="localData.locked">
      </div>
    </fieldset>
    <fieldset class="big-group">
      <div>
        <Image :image="localData.image" :dimensions="'640x420'" @update="saveData('image', $event)" />
      </div>
      <div>
        <div>
          <label>
            Description
            <Tip>A 1-2 sentence description of the project.</Tip>
          </label>
          <textarea v-model="localData.description" placeholder="A brief description" :class="flags('description')"/>
        </div>
        <Effects :effects="localData.effects" @update="saveData('effects', $event)" />
      </div>
    </fieldset>

    <Outcomes :outcomes="localData.outcomes" @update="saveData('outcomes', $event)" />
    <Upgrades :upgrades="localData.upgrades" @update="saveData('upgrades', $event)" />

    <div>
      <label>
        Flavor Text/Dialogue
        <Tip>Possible dialogue or other flavor to text to accompany the policy (e.g.an advisor giving their take on the policy when you first select/unlock it).</Tip>
      </label>
      <textarea v-model="localData.flavor" placeholder="Flavor text and dialogue" />
    </div>

    <Notes :notes="localData.notes" @blur="saveNotes" />

    <div class="additional-actions">
      <button @click="delete">Delete</button>
    </div>
  </template>

  <div v-else class="project-summary item-summary">
    <div class="item-meta">
      <div class="meta-pill">{{localData.type}}</div>
      <div class="meta-pill split-pill" :class="{invalid: !localData.cost}">
        <div>{{localData.type == 'Policy' ? 'Cost' : 'Years'}}</div>
        <div>{{localData.cost || 'MISSING'}}</div>
      </div>
      <div class="meta-pill split-pill" v-if="localData.dynamic_cost">
        <div>Dynamic Cost</div>
        <div>{{localData.dynamic_cost_demand}}</div>
      </div>
      <div class="meta-pill" v-if="localData.locked" :class="flags('locked')">Locked{{flags('locked').invalid ? ' MISSING UNLOCKER' : ''}}</div>
      <div class="meta-pill" v-else-if="!localData.locked && flags('locked').invalid" :class="flags('locked')">UNLOCKABLE BUT NOT LOCKED</div>
    </div>
    <fieldset class="big-group">
      <div>
        <div class="item-summary-title" v-if="localData.name">{{localData.name}}</div>
        <div class="item-summary-title invalid" v-else>[MISSING NAME]</div>
        <p class="item-summary-desc" v-if="localData.description" v-html="descriptionHtml"></p>
        <p class="item-summary-desc invalid" v-else>[MISSING DESCRIPTION]</p>
        <EffectsSummary v-if="defined('effects')" :effects="localData.effects" />
        <div class="item-missing invalid" v-else-if="localData.outcomes && localData.outcomes.length == 1">[MISSING EFFECTS]</div>
      </div>
      <div class="item-summary-image" v-if="localData.image">
        <img class="image-preview" v-if="localData.image.image" :src="`/image/${localData.image.image}`"/>
        <div class="image-attribution">{{localData.image.attribution}}</div>
      </div>
    </fieldset>
    <template v-if="localData.outcomes.length > 1">
      <h5>Outcomes</h5>
      <OutcomesSummary :outcomes="localData.outcomes" />
    </template>
    <template v-if="localData.upgrades.length >= 1">
      <h5>Upgrades</h5>
      <UpgradesSummary :upgrades="localData.upgrades" />
    </template>
    <div class="item-summary-notes" v-if="localData.notes" v-html="notesHtml"></div>
  </div>
</li>
</template>

<script>
import uuid from '../../uuid';
import ItemMixin from './ItemMixin';
export default {
  created() {
    if (!this.localData.outcomes) {
      // Default outcome
      this.localData.outcomes = [{
        id: uuid(),
        effects: [],
        probability: {
          id: uuid(),
          type: 'Guaranteed',
          conditions: [],
        }
      }];
      /* this.save(); */
    }
    if (!this.localData.upgrades) {
      this.localData.upgrades = [];
    }
  },
  computed: {
    descriptionHtml() {
      return this.localData.description.replaceAll('\n', '<br />');
    },
  },
  mixins: [ItemMixin]
};
</script>

<style>
.project-summary .item-summary-details > * {
  width: 50%;
}
.project-summary .meta-pill:first-child {
	background: #82ff9b;
}
.project-summary .meta-pill:nth-child(2) {
  background: #9eb4c7;
}
</style>
