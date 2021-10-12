<template>
<li class="item" :key="item.id" :id="item.id" ref="root">
  <Flags :invalid="invalid" :questions="questions" />
  <button class="edit-toggle" @click="() => this.editing = !this.editing">{{ this.editing ? '⮪' : '✎'}}</button>
  <template v-if="editing">
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
      <div>
        <label>
          Years to Completion
          <Tip>For uncertain projects, this is the minimum years required to start rolling for success.</Tip>
        </label>
        <input type="number" v-model="localData.years" @change="save" :class="flags('years')"/>
      </div>
      <div class="checkbox">
        <label :for="`${item.id}_uncertain`">
          Uncertain
          <Tip>Is this project guaranteed to finish or not?</Tip>
        </label>
        <input type="checkbox" :id="`${item.id}_uncertain`" v-model="localData.uncertain" @change="save">
      </div>
      <div class="checkbox">
        <label :for="`${item.id}_ongoing`">
          Ongoing
          <Tip>Is this a one-and-done project, or does it need continued maintenance?</Tip>
        </label>
        <input type="checkbox" :id="`${item.id}_ongoing`" v-model="localData.ongoing" @change="save">
      </div>
      <div class="checkbox">
        <label :for="`${item.id}_locked`">
          Locked
          <Tip>Is this process available to the player at the start?</Tip>
        </label>
        <input type="checkbox" :id="`${item.id}_locked`" v-model="localData.locked" @change="save">
      </div>
    </fieldset>
    <fieldset class="big-group">
      <div>
        <Image :image="localData.image" @update="saveData('image', $event)" />
      </div>
      <div>
        <div>
          <label>
            Description
            <Tip>A 1-2 sentence description of the project.</Tip>
          </label>
          <textarea v-model="localData.description" placeholder="A brief description" @blur="save" :class="flags('description')"/>
        </div>
        <Effects :toggle="true" :effects="localData.effects" @update="saveData('effects', $event)" />
      </div>
    </fieldset>

    <div v-if="localData.type == 'Initiative'">
      <h3>Implementation (per year)</h3>
      <Resources :resources="localData.resources" @update="saveData('resources', $event)"/>
      <Byproducts :byproducts="localData.byproducts" @update="saveData('byproducts', $event)"/>
    </div>

    <Outcomes :outcomes="localData.outcomes" @update="saveData('outcomes', $event)" />

    <div>
      <label>
        Flavor Text/Dialogue
        <Tip>Possible dialogue or other flavor to text to accompany the policy (e.g.an advisor giving their take on the policy when you first select/unlock it).</Tip>
      </label>
      <textarea v-model="localData.flavor" placeholder="Flavor text and dialogue" @blur="save" />
    </div>

    <Notes :notes="localData.notes" @blur="saveNotes" />

    <div class="additional-actions">
      <button @click="delete">Delete</button>
    </div>
  </template>

  <div v-else class="project-summary item-summary">
    <div class="item-meta">
      <div class="meta-pill">{{localData.type}}</div>
      <div class="meta-pill split-pill" :class="{invalid: !localData.years}">
        <div>Years</div>
        <div>{{localData.years || 'MISSING'}}</div>
      </div>
      <div class="meta-pill" v-if="localData.uncertain">Uncertain</div>
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
        <div class="item-missing invalid" v-else>[MISSING EFFECTS]</div>
      </div>
      <div class="item-summary-image" v-if="localData.image">
        <img class="image-preview" v-if="localData.image.image" :src="`/image/${localData.image.image}`"/>
        <div class="image-attribution">{{localData.image.attribution}}</div>
      </div>
    </fieldset>
    <div v-if="localData.type == 'Initiative'">
        <h5>Implementation (per year)</h5>
        <ResourcesSummary :resources="localData.resources" />
        <ByproductsSummary :byproducts="localData.byproducts" />
      </div>
    <h5>Outcomes</h5>
    <OutcomesSummary :outcomes="localData.outcomes" />
    <div class="item-summary-notes" v-if="localData.notes" v-html="notesHtml"></div>
  </div>
</li>
</template>

<script>
import uuid from '../../uuid';
import ItemMixin from './ItemMixin';
export default {
  mounted() {
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
      this.save();
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
