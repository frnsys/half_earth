<template>
<div class="effects">
  <div class="header">
    Effects
    <button @click="addEffect">+ Effect</button>
  </div>
  <ul>
    <li v-for="effect in effects" :key="effect.id">
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
  methods: {
    update() {
      this.$emit('update', this.effects);
    },
    deleteEffect(effect) {
      let idx = this.effects.findIndex((e) => e == effect);
      this.effects.splice(idx, 1);
      this.update();
    },
    addEffect() {
      this.effects.push({
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
.effects .header {
  display: flex;
  align-items: center;
  font-size: 0.7em;
  justify-content: space-between;
}
.effects .header button {
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
