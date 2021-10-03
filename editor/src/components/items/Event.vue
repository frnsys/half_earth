<template>
<li class="item" :key="item.id" :id="item.id" ref="root">
  <div>
    <label>
      Short Name
      <Tip>The short name of the event</Tip>
    </label>
    <input type="text" placeholder="Name" v-model="localData.name" @blur="save" :class="flags('name')" />
  </div>

  <div>
    <label>
      Description
      <Tip>A more detailed narrative description of the event.</Tip>
    </label>
    <textarea class="title" placeholder="Description" v-model="localData.description" @blur="save" :class="flags('description')" />
  </div>

  <div class="event-variables" v-if="variables.length > 0">
      <span>Variables:</span>
      <div class="summary-pill event-variable" v-for="v in variables" :class="{invalid:v.invalid}">
        <div>{{v.name}}</div>
        <div>
          <Tip v-if="v.values.length > 0" :width="100"><div v-for="val in v.values">{{val}}</div></Tip>
          <template v-else>NO VALUES DEFINED</template>
        </div>
      </div>
  </div>

  <fieldset>
    <div>
      <label>
        Story Arc (optional)
        <Tip>If the event is part of or triggers an arc, note the arc name here</Tip>
      </label>
      <input type="text" list="arcs" v-model="localData.arc" @blur="save" />
    </div>
    <div class="checkbox">
      <label :for="`${item.id}_repeats`">
        Repeats
        <Tip>Can this event occur more than once?</Tip>
      </label>
      <input type="checkbox" :id="`${item.id}_repeats`" v-model="localData.repeats" @change="save">
    </div>
    <div class="checkbox">
      <label :for="`${item.id}_decision`">
        Decision
        <Tip>Is this an informative event or does the player need to make a decision?</Tip>
      </label>
      <input type="checkbox" :id="`${item.id}_decision`" v-model="localData.decision" @change="save">
    </div>
    <div class="checkbox">
      <label :for="`${item.id}_local`">
        Local
        <Tip>Is this event something that happens locally or globally?</Tip>
      </label>
      <input type="checkbox" :id="`${item.id}_local`" v-model="localData.local" @change="save">
    </div>
  </fieldset>

  <Probabilities :probabilities="localData.probabilities" @update="saveData('probabilities', $event)" />
  <Effects :toggle="true" :effects="localData.effects" @update="saveData('effects', $event)" />
  <Choices :id="item.id" :choices="localData.choices" v-if="localData.decision" @update="saveData('choices', $event)"/>

  <div>
    <label>
      Flavor Text/Dialogue
      <Tip>Advisor dialogue introducing the event.</Tip>
    </label>
    <textarea v-model="localData.flavor" placeholder="Flavor text and dialogue" @blur="save" />
  </div>

  <Notes :notes="localData.notes" @blur="saveNotes" />
</li>
</template>

<script>
import state from '../../state';
import ItemMixin from './ItemMixin';
import Choices from '../subs/Choices.vue';

export default {
  components: {
    Choices
  },
  mounted() {
    if (!this.localData.choices) {
      this.localData.choices = [...Array(4)].map(() => ({
        text: '',
        condition: '',
        effects: []
      }));
      this.save();
    }
  },
  computed: {
    validateKeys() {
      return ['name', 'description'];
    },
    questionKeys() {
      return ['name', 'description'];
    },
    variables() {
      let definedVariables = Object.values(state.items)
        .filter((i) => i._type == 'Variable')
        .reduce((acc, v) => {
          acc[v.name] = (v.values || '').split('\n').filter((x) => x !== '');
          return acc;
        }, {});
      let matches = [...(this.localData.description || '').matchAll('\{([a-z_]+)\}')];
      return matches.map((group) => {
        let name = group[1];
        let defined = Object.keys(definedVariables).includes(name);
        let values = definedVariables[name];
        return {
          name,
          values,
          invalid: (!defined) || values.length == 0
        }
      });
    },
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
</style>
