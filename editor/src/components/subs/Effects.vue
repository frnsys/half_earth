<template>
<div class="effects">
  <label>
    Effects
    <div>
      <button @click="addEffect">+ Effect</button>
      <button v-if="toggle" @click="() => this.editing = !this.editing">{{ this.editing ? '⮪' : '✎'}}</button>
    </div>
  </label>
  <ul v-if="editing">
    <li v-for="effect in localData" :key="effect.id">
      <Effect :effect="effect" @update="update" /> <button @click="() => deleteEffect(effect)">X</button>
    </li>
  </ul>
  <EffectsSummary v-else :effects="localData" />
</div>
</template>

<script>
import uuid from '../../uuid';
import Effect from './Effect.vue';
import EffectsSummary from './EffectsSummary.vue';

export default {
  props: ['effects', 'toggle'],
  components: {
    Effect,
    EffectsSummary
  },
  data() {
    return {
      editing: this.toggle ? false : true,
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
      this.editing = true;
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
