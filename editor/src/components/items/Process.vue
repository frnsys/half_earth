<template>
<li class="item" :id="item.id" ref="root">
  <Flags :invalid="localData._validation.invalid" :questions="localData._validation.questions" />
  <button class="edit-toggle" @click="toggleEditing">{{ this.editing ? '⮪' : '✎'}}</button>
  <template v-if="editing">
    <div>
      <label>
        Name
        <Tip>The name of the process.</Tip>
      </label>
      <input class="title" type="text" placeholder="Name" v-model="localData.name" :class="flags('name')" />
    </div>
    <fieldset>
      <div>
        <label>
          Output
          <Tip>What this output produces.</Tip>
        </label>
        <select v-model="localData.output" :class="flags('output')">
          <option v-for="k in Object.keys(OUTPUTS)" :value="k">{{k}} ({{OUTPUTS[k]}})</option>
        </select>
      </div>
      <div>
        <label>
          Feedstock
          <Tip>The main feedstock this process depends on.</Tip>
        </label>
        <select v-model="localData.feedstock" :class="flags('feedstock')">
          <option v-for="k in Object.keys(FEEDSTOCKS)" :value="k">{{k}} ({{FEEDSTOCKS[k]}})</option>
        </select>
      </div>
      <div>
        <label>
          Feedstock per output
          <Tip>Amount of feedstock required per unit output</Tip>
        </label>
        <input type="number" min="0" v-model="localData.feedstock_amount" :class="flags('feedstock_amount')" />
      </div>
      <div>
        <label>
          Mix Share
          <Tip>The mix share (%) of this process for its output.</Tip>
        </label>
        <input type="number" min="0" v-model="localData.mix_share" :class="flags('mix_share')" />
      </div>
      <div>
        <label>
          Output Limit
          <Tip>Optional maximum output this process can produce.</Tip>
        </label>
        <input type="number" min="0" v-model="localData.limit" />
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
        <label>
          Description
          <Tip>A 1-2 sentence description of the project.</Tip>
        </label>
        <textarea v-model="localData.description" placeholder="A brief description" :class="flags('description')"/>
      </div>
      <div>
        <Resources :resources="localData.resources" @update="saveData('resources', $event)"/>
        <Byproducts :byproducts="localData.byproducts" @update="saveData('byproducts', $event)"/>
        <div>
          <label>
            Process Features
            <Tip>Special flags indicating additional process features/details. Used by (for example) events.</Tip>
          </label>
          <div class="checkbox-feature" v-for="k in Object.keys(PROCESS_FEATURES)">
            <input :checked="getFeature(k)" type="checkbox" :id="`${item.id}_${k}`" @change="(ev) => updateFeature(k, ev.target.checked)">
            <label :for="`${item.id}_${k}`">{{k}}</label>
            <Tip>{{PROCESS_FEATURES[k]}}</Tip>
          </div>
        </div>
      </div>
    </fieldset>

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

    <Notes :notes="localData.notes" @blur="saveNotes" />

    <div class="additional-actions">
      <button @click="delete">Delete</button>
    </div>
  </template>

  <div v-else class="process-summary item-summary">
    <div class="item-meta">
      <div class="meta-pill">{{localData.output}}</div>
      <div class="meta-pill split-pill" :class="flags('mix_share')">
        <div>Mix Share</div>
        <div>{{flags('mix_share').invalid ? 'MISSING' : localData.mix_share }}%</div>
      </div>
      <div class="meta-pill split-pill" v-if="localData.limit">
        <div>Output Limit</div>
        <div>{{localData.limit}}{{OUTPUTS[localData.output]}}</div>
      </div>
      <div class="meta-pill" v-if="localData.locked" :class="flags('locked')">Locked{{flags('locked').invalid ? ' MISSING UNLOCKER' : ''}}</div>
      <div class="meta-pill" v-else-if="!localData.locked && flags('locked').invalid" :class="flags('locked')">UNLOCKABLE BUT NOT LOCKED</div>
      <template v-for="k in Object.keys(PROCESS_FEATURES)" v-if="localData.features">
        <div class="meta-pill feature-pill" v-if="localData.features[k]"><div>{{k}}</div></div>
      </template>
    </div>
    <fieldset class="big-group">
      <div>
        <div class="item-summary-title" v-if="localData.name">{{localData.name}}</div>
        <div class="item-summary-title invalid" v-else>[MISSING NAME]</div>
        <div>
          <h5 class="kinds-summary-label">Per {{OUTPUTS[localData.output]}}:</h5>
          <div class="summary-pill feedstock-pill">
            <div>{{localData.feedstock || '[MISSING]'}}</div>
            <div>{{localData.feedstock_amount || '[MISSING]'}} {{FEEDSTOCKS[localData.feedstock]}}</div>
          </div>
          <ResourcesSummary :resources="localData.resources" />
          <ByproductsSummary :byproducts="localData.byproducts" />
          <p v-if="localData.description">{{localData.description}}</p>
          <p v-else class="invalid">[MISSING DESCRIPTION]</p>
        </div>
      </div>
      <div class="item-summary-image" v-if="localData.image">
        <img class="image-preview" v-if="localData.image.image" :src="`/image/${localData.image.image}`"/>
        <div class="image-attribution">{{localData.image.attribution}}</div>
      </div>
    </fieldset>

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
import state from '../../state';
import ItemMixin from './ItemMixin';
import SlimSelect from 'slim-select';

export default {
  computed: {
    npcs() {
      return Object.values(state.items)
        .filter((i) => i._type == 'NPC')
    }
  },
  mounted() {
    if (!this.localData.features) {
      this.localData.features = {};
      /* this.save(); */
    }
    this.setupSelect();
  },
  updated() {
    this.setupSelect();
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
    updateFeature(key, val) {
      this.localData.features[key] = val;
      /* this.save(); */
    },
    getFeature(key) {
      return this.localData.features ? this.localData.features[key] : false;
    }
  },
  mixins: [ItemMixin]
};
</script>

<style>
.checkbox-feature {
	display: inline-block;
	background: #eee;
	padding: 0.1em 0.25em 0.2em 0.1em;
	border: 1px solid #aaa;
  margin-right: 1em;
  margin-bottom: 0.5em;
}
.checkbox-feature > input, .checkbox-feature > label {
  width: auto;
  display: inline;
}
.checkbox-feature .tip {
  font-size: 0.75em;
}

.process-summary .item-summary-details > * {
  width: 50%;
}
.process-summary .meta-pill:first-child {
	background: #82ff9b;
}
.meta-pill.feature-pill {
  background: #98dca6;
}

.feedstock-pill > div:first-child {
  background: #9898fd;
}
</style>
