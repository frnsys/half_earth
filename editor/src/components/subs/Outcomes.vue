<template>
<div>
  <label>
    Outcomes
    <button @click="addOutcome">+ Outcome</button>
  </label>
  <div class="outcomes">
    <div class="field-group" v-for="(outcome, i) in localData" :key="outcome.id">
      <div>
        <label>
          Outcome Text
          <Tip>The text presented to the player when this outcome occurs.</Tip>
        </label>
        <input type="text" placeholder="Outcome text" v-model="outcome.text" @blur="update" :class="outcomeFlag(i, 'text')" />
      </div>
      <Probability v-if="i < localData.length - 1" :probability="outcome.probability" @update="saveOutcomeProbability(i, $event)" />
      <Effects :effects="outcome.effects" @update="saveOutcomeEffects(i, $event)" />
      <div v-if="i < localData.length - 1" class="subitem-actions">
        <button @click="() => deleteOutcome(outcome)">X</button>
        <button v-if="i > 0" @click="() => moveOutcome(i, i-1)">ᐱ</button>
        <button v-if="i < localData.length - 2" @click="() => moveOutcome(i, i+1)">ᐯ</button>
      </div>
      <div v-else class="default-label">Default Outcome</div>
    </div>
  </div>
</div>
</template>

<script>
import uuid from '../../uuid';
import Tip from '../Tip.vue';
import Effects from './Effects.vue';
import Probability from './Probability.vue';

export default {
  props: ['id', 'outcomes'],
  components: {
    Tip, Effects, Probability,
  },
  data() {
    return {
      localData: this.outcomes || []
    };
  },
  methods: {
    update() {
      this.$emit('update', this.localData);
    },
    addOutcome() {
      this.localData.unshift({
        id: uuid(),
        text: '',
        effects: [],
        probability: {
          id: uuid(),
          type: 'Impossible',
          conditions: [],
        }
      });
      this.update();
    },
    // https://stackoverflow.com/a/6470794
    moveOutcome(fromIdx, toIdx) {
      let item = this.localData[fromIdx];
      this.localData.splice(fromIdx, 1);
      this.localData.splice(toIdx, 0, item);
      this.update();
    },
    deleteOutcome(outcome) {
      this.localData = this.localData.filter((e) => e != outcome);
      this.update();
    },
    saveOutcomeEffects(i, effects) {
      this.localData[i].effects = effects;
      this.update();
    },
    saveOutcomeProbability(i, probability) {
      console.log(`setting probability for ${i}`);
      console.log(probability);
      this.localData[i].probability = probability;
      console.log(this.localData);
      this.update();
    },
    outcomeFlag(i, key) {
      let val = this.localData[i][key];
      return {invalid: !(val && val.length > 0)};
    }
  }
}
</script>

<style>
.outcomes {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-around;
}
.outcomes .field-group {
  width: 49%;
  position: relative;
}
.outcomes .effects {
  border: none;
  padding: 0;
}
.outcomes .subitem-actions,
.outcomes .default-label {
	position: absolute;
	top: 0;
	right: 0;
	transform: translate(0, -50%);
	font-size: 0.8em;
}
.outcomes .subitem-actions button {
  display: inline-block;
  font-size: 0.6em;
  padding: 0 0.3em;
}
.outcomes .default-label {
  background: #BDEFF4;
	padding: 0 0.2em;
	font-size: 0.6em;
	border: 1px solid #000;
	border-radius: 0.2em;
	transform: translate(50%, -50%);
	right: 50%;
}
</style>
