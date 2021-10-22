<template>
<div class="responses">
  <label>
    Responses
    <button @click="addResponse">+ Response</button>
  </label>
  <div class="choices">
    <div class="field-group" v-for="(choice, i) in localData">
      <div>
        <label>
          Choice Text
          <Tip>The text representing this choice, presented to the player.</Tip>
        </label>
        <input type="text" placeholder="Choice text" v-model="choice.text" @blur="update" :class="choiceFlag(i, 'text')" />
      </div>
      <fieldset>
        <Effects :effects="choice.effects" @update="saveChoiceEffects(i, $event)" />
        <Conditions :conditions="choice.conditions" @update="saveChoiceConditions(i, $event)" />
      </fieldset>
      <Dialogue :id="`${id}-${i}`" :dialogue="choice.dialogue" @update="saveDialogue(i, $event)"/>

      <div class="subitem-actions">
        <button @click="() => deleteResponse(choice)">X</button>
      </div>
    </div>
  </div>
</div>
</template>

<script>
import Tip from '../Tip.vue';
import Effects from './Effects.vue';
import Conditions from './Conditions.vue';

export default {
  props: ['id', 'choices'],
  components: {
    Tip, Effects, Conditions,
  },
  beforeCreate: function () {
    // Hack around circular references
    this.$options.components.Dialogue = require('./Dialogue.vue').default
  },
  data() {
    return {
      localData: this.choices || []
    };
  },
  methods: {
    addResponse() {
      this.localData.push({
        text: '',
        effects: [],
        conditions: [],
        dialogue: null
      });
      this.update();
    },
    deleteResponse(choice) {
      this.localData = this.localData.filter((e) => e != choice);
      this.update();
    },
    flags(choice) {
      let invalid = choice.text === undefined || choice.text === '';
      return {invalid};
    },
    update() {
      this.$emit('update', this.localData);
    },
    saveDialogue(i, dialogue) {
      this.localData[i].dialogue = dialogue;
      this.update();
    },
    saveChoiceEffects(i, effects) {
      this.localData[i].effects = effects;
      this.update();
    },
    saveChoiceConditions(i, conditions) {
      this.localData[i].conditions = conditions;
      this.update();
    },
    choiceFlag(i, key) {
      let val = this.localData[i][key];
      return {invalid: !(val && val.length > 0)};
    }
  }
}
</script>

<style>
.responses {
	background: #f5f5f5;
	padding: 0 0.5em 0.5em 0.5em;
	border: 1px solid #aaa;
	margin-top: 0.5em;
}
.responses > label {
  align-items: center;
}
.choices {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-around;
}
.choices .field-group {
  width: 100%;
  position: relative;
}
.choices .effects {
  border: none;
  padding: 0;
  margin: 0;
}
.responses .subitem-actions {
	position: absolute;
	top: 0;
	right: 0;
	transform: translate(0, -50%);
	font-size: 0.8em;
}
</style>
