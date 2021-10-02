<template>
<div class="probabilities">
  <label>
    Probabilities
    <div>
      <button @click="addProbability">+ Probability</button>
      <button @click="() => this.editing = !this.editing">{{ this.editing ? '⮪' : '✎'}}</button>
    </div>
  </label>
  <ul v-if="editing">
    <li v-for="(probability, i) in localData" :key="probability.id">
      <Probability :probability="probability" @update="update" />
      <div class="subitem-actions">
        <button @click="() => deleteProbability(probability)">X</button>
        <button v-if="i > 0" @click="() => moveProbability(i, i-1)">ᐱ</button>
        <button v-if="i < localData.length - 1" @click="() => moveProbability(i, i+1)">ᐯ</button>
      </div>
    </li>
  </ul>
  <ul v-else class="probabilities-summary">
    <li v-for="probability in localData" :key="probability.id">
      <div class="probability-type">{{ probability.type }}</div>
      <span> if </span>
      <ConditionsSummary :conditions="probability.conditions" />
    </li>
  </ul>
</div>
</template>

<script>
import uuid from '../../uuid';
import Probability from './Probability.vue';
import ConditionsSummary from './ConditionsSummary.vue';

export default {
  props: ['probabilities'],
  components: {
    Probability,
    ConditionsSummary
  },
  data() {
    return {
      editing: false,
      localData: this.probabilities || []
    };
  },
  methods: {
    update() {
      this.$emit('update', this.localData);
    },
    // https://stackoverflow.com/a/6470794
    moveProbability(fromIdx, toIdx) {
      let item = this.localData[fromIdx];
      this.localData.splice(fromIdx, 1);
      this.localData.splice(toIdx, 0, item);
      this.update();
    },
    deleteProbability(probability) {
      this.localData = this.localData.filter((e) => e != probability);
      this.update();
    },
    addProbability() {
      this.localData.push({
        id: uuid(),
        type: 'Impossible',
        conditions: [],
      });
      this.update();
      this.editing = true;
    },
  }
}
</script>

<style>
.probabilities {
	background: #f5f5f5;
	padding: 0 0.5em 0.5em 0.5em;
	border: 1px solid #aaa;
	margin-top: 0.5em;
}
.probabilities > ul > li {
  margin: 0.5em 0 0 0 !important;
  display: flex;
  background: #e8e8e8;
	padding: 0.5em;
	border: 1px solid #aaa;
}
.probabilities li button {
  height: 20px;
}
.probabilities > label {
  align-items: center;
}
.probabilities > label button {
  font-size: 0.9em;
  line-height: 1.2;
  margin-left: 0.5em;
}
.probability {
  flex: 1;
}
.subitem-actions {
  padding-left: 0.5em;
}
.subitem-actions button {
  display: block;
}
.probabilities-summary .conditions-summary {
  margin: 0;
  display: inline-block;
}
.probabilities-summary > li {
  display: block !important;
	margin: 0 !important;
	padding: 0 !important;
	background: 0 !important;
	border: 0 !important;
}
.probability-type {
  font-size: 0.7em;
  display: inline-block;
  padding: 0.25em;
  border: 1px solid #856363;
  background: #FABA93;
  border-radius: 0.2em;
  width: 70px;
}
</style>
