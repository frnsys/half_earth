<template>
<div class="conditions">
  <div class="header">
    <slot name="title">Conditions</slot>
    <div class="conditions--actions">
      <button @click="addCondition">+ Condition</button>
      <slot name="actions"></slot>
    </div>
  </div>
  <ul>
    <li v-for="condition in localData" :key="condition.id">
      <Condition :condition="condition" @update="update" /> <button @click="() => deleteCondition(condition)">X</button>
    </li>
  </ul>
</div>
</template>

<script>
import uuid from '../../uuid';
import Condition from './Condition.vue';

export default {
  props: ['conditions'],
  components: {
    Condition,
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
.conditions .header {
  display: flex;
  align-items: center;
  font-size: 0.7em;
  justify-content: space-between;
}
.conditions .header button {
  font-size: 0.95em;
  margin-left: 0.5em;
  line-height: 1.2;
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

.conditions--actions {
  display: flex;
}
</style>
