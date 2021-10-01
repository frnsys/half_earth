<template>
<div class="probability">
  <select v-model="localData.type" @change="update">
    <option v-for="type in PROBABILITIES" :value="type">{{type}}</option>
  </select>
  <Conditions :conditions="localData.conditions" @update="update" />
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
    update() {
      this.$emit('update', this.localData);
    },
  }
}
</script>
