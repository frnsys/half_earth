<template>
<li class="item" :id="item.id" ref="root">
  <Flags :invalid="localData._validation.invalid" :questions="localData._validation.questions" />
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
          Group
          <Tip>The grouping this project falls under.</Tip>
        </label>
        <select v-model="localData.group">
          <option v-for="k in PROJECT_GROUPS" :value="k">{{k}}</option>
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
        <label :for="`${item.id}_dynamic_cost_factor`">
          Dynamic Cost Factor
          <Tip>The demand or other factor used to calculate the dynamic cost.</Tip>
        </label>
        <select v-model="localData.dynamic_cost_factor">
          <option v-for="k in DYNAMIC_COST_FACTORS" :value="k">{{k}}</option>
        </select>
      </div>
      <div class="checkbox">
        <label :for="`${item.id}_ongoing`">
          Ongoing
          <Tip>Is this a one-and-done project, or does it need continued maintenance?</Tip>
        </label>
        <input type="checkbox" :id="`${item.id}_ongoing`" v-model="localData.ongoing">
      </div>
      <div class="checkbox" v-if="localData.type == 'Initiative'">
        <label :for="`${item.id}_gradual`">
          Gradual
          <Tip>Does this project have to be 100% finished before the effects occur, or do they develop as the project is developed?</Tip>
        </label>
        <input type="checkbox" :id="`${item.id}_gradual`" v-model="localData.gradual">
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
        <Image :image="localData.image" :dimensions="'360x240'" @update="saveData('image', $event)" />
      </div>
      <div>
        <div>
          <label>
            Description
            <Tip>A 1-2 sentence description of the project.</Tip>
          </label>
          <textarea v-model="localData.description" placeholder="A brief description" :class="flags('description')"/>
        </div>
        <Effects :effects="localData.effects || []" @update="saveData('effects', $event)" />
      </div>
    </fieldset>

    <Outcomes :outcomes="localData.outcomes" @update="saveData('outcomes', $event)" />
    <Upgrades :upgrades="localData.upgrades" @update="saveData('upgrades', $event)" />

    <fieldset>
      <div>
        <label>
          Supporters
          <Tip>Which NPCs support this project.</Tip>
        </label>
        <select multiple v-model="localData.supporters" :id="`${item.id}_supporters`">
          <option v-for="npc in npcs" :value="npc.id">{{npc.name}}</option>
        </select>
      </div>
      <div>
        <label>
          Opposers
          <Tip>Which NPCs oppose this project.</Tip>
        </label>
        <select multiple v-model="localData.opposers" :id="`${item.id}_opposers`">
          <option v-for="npc in npcs" :value="npc.id">{{npc.name}}</option>
        </select>
      </div>
    </fieldset>

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
        <div>{{localData.dynamic_cost_factor}}</div>
      </div>
      <div class="meta-pill group-pill" v-if="localData.group">{{localData.group}}</div>
      <div class="meta-pill invalid" v-else>MISSING GROUP</div>
      <div class="meta-pill" v-if="localData.ongoing">Ongoing</div>
      <div class="meta-pill" v-if="localData.gradual">Gradual</div>
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
    <div class="project-outcomes" v-if="localData.outcomes && localData.outcomes.length > 0">
      <h5>Outcomes</h5>
      <OutcomesSummary :outcomes="localData.outcomes" />
    </div>
    <template v-if="localData.upgrades && localData.upgrades.length > 0">
      <h5>Upgrades</h5>
      <UpgradesSummary :upgrades="localData.upgrades" />
    </template>

    <fieldset class="stances">
      <div v-if="localData.supporters && localData.supporters.length > 0">
        <h5>Supporters</h5>
        <span v-for="id in localData.supporters">{{npc(id) ? npc(id).name : id}}</span>
      </div>
      <div v-if="localData.opposers && localData.opposers.length > 0">
        <h5>Opposers</h5>
        <span v-for="id in localData.opposers">{{npc(id) ? npc(id).name : id}}</span>
      </div>
    </fieldset>

    <div class="item-summary-notes" v-if="localData.notes" v-html="notesHtml"></div>
  </div>
</li>
</template>

<script>
import uuid from '../../uuid';
import state from '../../state';
import ItemMixin from './ItemMixin';
import SlimSelect from 'slim-select';

export default {
  mounted() {
    this.setupSelect();
  },
  updated() {
    this.setupSelect();
  },
  created() {
    if (!this.localData.outcomes) {
      // Default outcome
      this.localData.outcomes = [{
        id: uuid(),
        effects: [],
        dialogue: {
          root: 0,
          lines: {
            0: {
              id: 0,
              speaker: 'Gossy',
              text: '',
              next: null,
            }
          }
        },
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
    npcs() {
      return Object.values(state.items)
        .filter((i) => i._type == 'NPC')
    }
  },
  methods: {
    npc(id) {
      return state.items[id];
    },
    setupSelect() {
      if (this.selects) {
        this.selects.forEach((select) => select.destroy());
      }
      this.selects = [];
      let sels = [
        `${this.localData.id}_supporters`,
        `${this.localData.id}_opposers`,
      ];
      sels.forEach((sel) => {
        let el = document.getElementById(sel);
        if (el) {
          this.selects.push(new SlimSelect({
            select: el
          }));
        }
      });
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

.project-outcomes {
  margin-top: 1em;
}

.stances {
  margin: 0.5em 0 0 0;
}
.stances > div {
  display: block;
}
.stances h5 {
  border-bottom: 1px solid #000;
}
.stances span {
  color: #fff;
  margin: 0.1em;
  font-size: 0.8em;
  background: #4b5a85;
  padding: 0 0.1em;
  border-radius: 0.2em;
  display: inline-block;
  border: 1px solid #000;
}

.group-pill {
  color: #fff !important;
  background: #1A73E8 !important;
}
</style>
