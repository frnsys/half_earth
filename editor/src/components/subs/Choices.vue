<template>
<div class="responses">
  <label>
    Responses
    <button @click="() => this.editing = !this.editing">{{ this.editing ? '⮪' : '✎'}}</button>
  </label>
  <div class="choices">
    <div class="field-group" v-for="(choice, i) in localData">
      <template v-if="editing">
        <div>
          <label>
            Choice Text
            <Tip>The text representing this choice, presented to the player.</Tip>
          </label>
          <input type="text" placeholder="Choice text" v-model="choice.text" @blur="update" :class="choiceFlag(i, 'text')" />
        </div>
        <div class="radio">
          <label>Type:</label>
          <div>
            <label :for="`${id}-${i}-none`">None</label>
            <input :id="`${id}-${i}-none`" type="radio" v-model="choice.type" value="none" @change="update">
          </div>
          <div>
            <label :for="`${id}-${i}-malthusian`">Malthusian</label>
            <input :id="`${id}-${i}-malthusian`" type="radio" v-model="choice.type" value="malthusian" @change="update">
          </div>
          <div>
            <label :for="`${id}-${i}-falc`">FALC</label>
            <input :id="`${id}-${i}-falc`" type="radio" v-model="choice.type" value="falc" @change="update">
          </div>
          <div>
            <label :for="`${id}-${i}-hes`">HES</label>
            <input :id="`${id}-${i}-hes`" type="radio" v-model="choice.type" value="hes" @change="update">
          </div>
        </div>
        <Effects :effects="choice.effects" @update="saveChoiceEffects(i, $event)" />
        <Conditions :conditions="choice.conditions" @update="saveChoiceConditions(i, $event)" />
      </template>
      <div v-else :class="flags(choice)" class="choice-summary">
        <div v-if="choice.text">
          <div class="choice-text">{{ choice.text }}</div>
          <template v-if="choice.effects && choice.effects.length > 0">
            <EffectsSummary :effects="choice.effects" />
          </template>
          <div class="choice-conditions" v-if="choice.conditions && choice.conditions.length > 0">
            <span>Available if:</span> <ConditionsSummary :conditions="choice.conditions" />
          </div>
        </div>
        <div class="missing-defined" v-else>None</div>
      </div>
    </div>
  </div>
</div>
</template>

<script>
import Tip from '../Tip.vue';
import Effects from './Effects.vue';
import Conditions from './Conditions.vue';
import EffectsSummary from './EffectsSummary.vue';
import ConditionsSummary from './ConditionsSummary.vue';

export default {
  props: ['id', 'choices'],
  components: {
    Tip, Effects, Conditions,
    EffectsSummary, ConditionsSummary
  },
  data() {
    return {
      editing: false,
      localData: this.choices || []
    };
  },
  methods: {
    flags(choice) {
      let invalid = choice.text === undefined || choice.text === '';
      return {invalid};
    },
    update() {
      this.$emit('update', this.localData);
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
  width: 49%;
}
.choices .effects {
  border: none;
  padding: 0;
}
.choice-text {
  margin: 1em 0 0.5em 0;
  font-weight: bold;
}
.choice-summary {
  display: flex;
  height: 100%;
}
.choice-summary.invalid {
  align-items: center;
}
.choice-summary .conditions-summary {
  display: inline-block;
}
.choice-conditions span {
  font-size: 0.8em;
}
</style>
