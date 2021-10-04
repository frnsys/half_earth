<template>
<div class="effects">
  <label>
    Effects
    <button @click="addEffect">+ Effect</button>
  </label>
  <ul>
    <li v-for="effect in localData" :key="effect.id">
      <Effect :effect="effect" @update="update" /> <button @click="() => deleteEffect(effect)">X</button>
    </li>
  </ul>
</div>
</template>

<script>
import uuid from '../../uuid';
import Effect from './Effect.vue';

export default {
  props: ['effects'],
  components: {
    Effect,
  },
  data() {
    return {
      localData: this.effects || []
    };
  },
  methods: {
    update() {
      this.$emit('update', this.localData);
    },
    deleteEffect(effect) {
      this.localData = this.localData.filter((e) => e != effect);
      this.update();
    },
    addEffect() {
      this.localData.push({
        id: uuid(),
        type: 'WorldVariable',
        subtype: 'Emissions',
        params: {'Change': 0}
      });
      this.update();
    },
  }
}
</script>

<style>
.effects {
	background: #f5f5f5;
	padding: 0 0.5em 0.5em 0.5em;
	border: 1px solid #aaa;
	margin-top: 0.5em;
}
.effects label {
  align-items: center;
}
.effects label button {
  font-size: 0.95em;
  line-height: 1.2;
  margin-left: 0.5em;
}
.effects li {
  display: flex;
  align-items: end;
}
.effects li button {
  height: 20px;
}
.effect {
  flex: 1;
}
</style>
