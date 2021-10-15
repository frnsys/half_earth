<template>
<div class="dialogue">
  <fieldset>
    <div class="speaker">
      <label>Speaker</label>
      <select v-model="localData.speaker" @change="update" :class="flags('speaker')">
        <option v-for="k in SPEAKERS" :value="k">{{k}}</option>
      </select>
    </div>
    <div>
      <label>Line</label>
      <input type="text" placeholder="Dialogue line" v-model="localData.text" @blur="update" :class="flags('text')" />
    </div>
  </fieldset>
  <Choices :id="id" :choices="localData.choices" @update="saveData('choices', $event)"/>
</div>
</template>

<script>
import state from '../../state';
import Choices from './Choices.vue';

export default {
  props: ['id', 'dialogue'],
  data() {
    return {
      localData: this.dialogue || {}
    };
  },
  components: {
    Choices,
  },
  methods: {
    update() {
      this.$emit('update', this.localData);
    },
    saveData(key, data) {
      this.localData[key] = data;
      this.update();
    },
    flags(key) {
      let invalid = false;
      let val = this.localData[key];
      switch (key) {
        case 'subtype':
          invalid = val === undefined || !this.spec.choices.includes(val);
          break;
        default:
          invalid = val === undefined
      }
      return {
        invalid
      };
    },

  },
}
</script>

<style>
.dialogue .speaker {
  flex: 0.25;
}
</style>
