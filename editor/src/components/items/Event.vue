<template>
<li class="item" :key="item.id" :id="item.id" ref="root">
  <Flags :invalid="invalid" :questions="questions" />
  <button class="edit-toggle" @click="toggleEditing">{{ this.editing ? '⮪' : '✎'}}</button>
  <template v-if="editing">
    <div class="event-variables" v-if="varMetas.length > 0">
        <span>Variables:</span>
        <div class="summary-pill event-variable" v-for="v in varMetas" :class="{invalid:v.invalid}">
          <div>{{v.name}}</div>
          <div v-if="v.values && v.values.length > 0" :style="{padding: '0.2em 0'}">
            <Tip :width="100"><div v-for="val in v.values">{{val}}</div></Tip>
          </div>
          <div v-else>NO VALUES DEFINED</div>
        </div>
    </div>

    <fieldset>
      <div>
        <div>
          <label>
            Short Name
            <Tip>The short name of the event to uniquely identify this event.</Tip>
          </label>
          <input type="text" placeholder="Name" v-model="localData.name" :class="flags('name')" />
        </div>
        <div>
          <label>
            Event Type
            <Tip>"World" = shows up in the world/event stream; "Planning" = shows up during planning sessions; "Breaks" = shows up between runs; "Icon" = shows up in the world/event stream, but only as an icon.</Tip>
          </label>
          <select v-model="localData.type" :class="flags('type')">
            <option v-for="type in EVENT_TYPES" :value="type">{{type}}</option>
          </select>
        </div>
        <fieldset>
          <div v-if="localData.type != 'Icon'">
            <label>
              Story Arc (optional)
              <Tip>If the event is part of or triggers an arc, note the arc name here.</Tip>
            </label>
            <input type="text" list="arcs" v-model="localData.arc" />
          </div>
          <div v-if="localData.type == 'Icon'">
            <label>
              Event Icon
              <Tip>Filename of the event icon.</Tip>
            </label>
            <input type="text" v-model="localData.icon" />
          </div>
          <div>
            <label :for="`${item.id}_locked`">
              Locked
              <Tip>Does this event start locked?</Tip>
            </label>
            <input type="checkbox" :id="`${item.id}_locked`" v-model="localData.locked">
          </div>
        </fieldset>
      </div>
      <div>
        <Image v-if="localData.type == 'World'" :image="localData.image" :dimensions="'480x630'" @update="saveData('image', $event)" />
      </div>
    </fieldset>

    <Probabilities :probabilities="localData.probabilities" @update="saveData('probabilities', $event)" />
    <Effects :effects="localData.effects" @update="saveData('effects', $event)" />

    <Dialogue v-if="localData.type !== 'Icon'" :id="item.id" :dialogue="localData.dialogue" @update="saveDialogue($event)"/>

    <Notes :notes="localData.notes" @blur="saveNotes" />

    <div class="additional-actions">
      <button @click="delete">Delete</button>
    </div>
  </template>

  <div v-else class="event-summary item-summary">
    <div class="item-meta">
      <div class="meta-pill">{{localData.name}}</div>
      <div class="meta-pill type-pill" :class="flags('type')">{{localData.type || 'MISSING TYPE'}}</div>
      <div class="meta-pill arc-pill" v-if="localData.arc">{{localData.arc}}</div>
      <div class="meta-pill" v-if="localData.locked" :class="flags('locked')">Locked{{flags('locked').invalid ? ' MISSING UNLOCKER' : ''}}</div>
      <div class="meta-pill" v-else-if="!localData.locked && flags('locked').invalid" :class="flags('locked')">UNLOCKABLE BUT NOT LOCKED</div>
    </div>
    <div class="event-variables" v-if="varMetas.length > 0">
        <span>Variables:</span>
        <div class="summary-pill event-variable" v-for="v in varMetas" :class="{invalid:v.invalid}">
          <div>{{v.name}}</div>
          <div v-if="v.values && v.values.length > 0" :style="{padding: '0.2em 0'}">
            <Tip :width="100"><div v-for="val in v.values">{{val}}</div></Tip>
          </div>
          <div v-else>NO VALUES DEFINED</div>
        </div>
    </div>
    <div class="item-summary-details">
      <div>
        <ProbabilitiesSummary v-if="definedWithValues('probabilities')" :probabilities="localData.probabilities" />
        <div class="item-missing invalid" v-else>[MISSING PROBABILITIES]</div>

        <EffectsSummary v-if="definedWithValues('effects')" :effects="localData.effects" />
        <div class="item-missing invalid" v-else-if="localData.type == 'Icon' || localData.type == 'World'">[MISSING EFFECTS]</div>
      </div>
      <div class="item-summary-image" v-if="localData.type == 'World' && localData.image">
        <img class="image-preview" v-if="localData.image.image" :src="`/image/${localData.image.image}`"/>
        <div class="image-attribution">{{localData.image.attribution}}</div>
      </div>
      <div class="item-missing invalid" v-else-if="localData.type == 'World'">[MISSING IMAGE]</div>
    </div>
    <DialogueSummary v-if="localData.type !== 'Icon'" :dialogue="localData.dialogue" />
    <div class="item-summary-notes" v-if="localData.notes" v-html="notesHtml"></div>
  </div>
</li>
</template>

<script>
import state from '../../state';
import ItemMixin from './ItemMixin';

export default {
  mounted() {
    if (!this.localData.variables) {
      this.parseVariables();
      /* this.save(); */
    }
  },
  methods: {
    parseVariables() {
      if (this.dialogue) {
        let matches = this.dialogue.lines.map((l) => {
          return [...(l.text || '').matchAll('\{([a-z_]+)\}')];
        }).flat();
        this.localData.variables = matches.map((group) => group[1]);
      }
    },
    saveDialogue(dialogue) {
      this.saveData('dialogue', dialogue);
      this.parseVariables();
      /* this.save(); */
    },
  },
  computed: {
    definedVariables() {
      return Object.values(state.items)
        .filter((i) => i._type == 'Variable')
        .reduce((acc, v) => {
          acc[v.name] = (v.values || '').split('\n').filter((x) => x !== '');
          return acc;
        }, {});
    },
    varMetas() {
      let definedVariables = this.definedVariables;
      return (this.localData.variables || []).map((name) => {
        let defined = Object.keys(definedVariables).includes(name);
        let values = definedVariables[name];
        return {name, values, invalid: (!defined) || values.length == 0};
      });
    }
  },
  mixins: [ItemMixin]
};
</script>

<style>
.event-variables {
	background: #eee;
	padding: 0.25em;
  border: 1px solid #aaa;
}
.event-variables > span {
	font-size: 0.8em;
	margin-right: 0.5em;
}
.event-variables .summary-pill > div:first-child {
  background: #ffc1fb;
}
.event-variables .summary-pill > div {
  background: #c3c3c3;
}
.event-variable .tip-icon {
  color: #2e2a2a;
  border-color: #2e2a2a;
  background: #eee;
}

.image-form .image-preview {
  max-width: 250px;
}

.event-summary .item-summary-details > div:first-child {
  flex: 1;
  margin-right: 1em;
}
.event-summary .item-summary-details .effects-summary {
  margin-top: 1em;
  padding-top: 1em;
  border-top: 1px solid #aaa;
}
.event-summary .image-preview {
  max-height: 250px;
}
.event-summary .meta-pill:first-child {
	background: #82ff9b;
}
.event-summary .arc-pill {
  background: #9eb4c7;
}
.event-summary .type-pill {
  background: #e7cb5d;
}
</style>
