<template>
<div class="probability">
  <select v-model="localData.type" @change="update">
    <option v-for="type in PROBABILITIES" :value="type">{{type}}</option>
  </select>
  <Conditions :conditions="localData.conditions" @update="updateConditions($event)" />
</div>
</template>

<script>
import Conditions from './Conditions.vue';

export default {
  props: ['probability'],
  data() {
    return {
      localData: this.probability
    };
  },
  components: {
    Conditions
  },
  methods: {
    updateConditions(conditions) {
      this.localData.conditions = conditions;
      this.update();
    },
    update() {
      this.$emit('update', this.localData);
    },
  }
}
</script>

<style>
.probability > select {
  background: #F49D69;
}
</style>
