<template>
<div class="conditions">
  <label>
    Conditions
    <button @click="addCondition">+ Condition</button>
  </label>
  <ul v-for="condition in localData" :key="condition.id">
    <li><Condition :condition="condition" @update="update" /> <button @click="() => deleteCondition(condition)">X</button></li>
  </ul>
</div>
</template>

<script>
import uuid from '../../uuid';
import Condition from './Condition.vue';

export default {
  props: ['conditions'],
  components: {
    Condition
  },
  data() {
    return {
      localData: this.conditions || []
    };
  },
  methods: {
    update() {
      this.$emit('update', this.localData);
    },
    deleteCondition(condition) {
      this.localData = this.localData.filter((e) => e != condition);
      this.update();
    },
    addCondition() {
      this.localData.push({
        id: uuid(),
        type: 'WorldVariable',
        subtype: 'Emissions',
        comparator: '>',
        value: 0
      });
      this.update();
    },
  }
}
</script>

<style>
.conditions label {
  align-items: center;
}
.conditions label button {
  font-size: 0.95em;
}
.conditions li {
  margin: 0 !important;
  display: flex;
  align-items: end;
}
.conditions li button {
  height: 20px;
}
.condition {
  flex: 1;
}
</style>

