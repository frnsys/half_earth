<template>
<div class="dialogue">
  <h4 class="dialogue-label">
    Dialogue
    <button @click="addLine">+ Line</button>
  </h4>
  <fieldset v-for="line, i in localData.lines">
    <div class="speaker">
      <label>Speaker</label>
      <select v-model="line.speaker" @change="update" :class="lineFlag(i, 'speaker')">
        <option v-for="k in SPEAKERS" :value="k">{{k}}</option>
      </select>
    </div>
    <div>
      <label>Line</label>
      <input type="text" placeholder="Dialogue line" v-model="line.text" @blur="update" :class="lineFlag(i, 'text')" />
    </div>
    <div class="dialogue-actions">
      <button v-if="localData.lines.length > 1" @click="() => deleteLine(line)">X</button>
      <button v-if="i > 0" @click="() => moveLine(i, i-1)">ᐱ</button>
      <button v-if="i < localData.lines.length - 1" @click="() => moveLine(i, i+1)">ᐯ</button>
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
    let localData = this.dialogue || {};
    if (!localData.lines) localData.lines = [{
      speaker: null,
      text: null,
    }];
    return {
      localData
    };
  },
  components: {
    Choices,
  },
  methods: {
    addLine() {
      this.localData.lines.push({
        speaker: null,
        text: null
      });
      this.update();
    },
    deleteLine(line) {
      this.localData.lines = this.localData.lines.filter((l) => l != line);
      this.update();
    },
    // https://stackoverflow.com/a/6470794
    moveLine(fromIdx, toIdx) {
      let item = this.localData.lines[fromIdx];
      this.localData.lines.splice(fromIdx, 1);
      this.localData.lines.splice(toIdx, 0, item);
      this.update();
    },

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
    lineFlag(i, key) {
      let val = this.localData.lines[i][key];
      return {invalid: !(val && val.length > 0)};
    }
  },
}
</script>

<style>
.dialogue-label {
  margin: 0.5em 0 0 0;
  border-bottom: 1px solid black;
}
.dialogue .speaker {
  flex: 0.25;
}

.dialogue-actions {
  flex: unset;
  flex-direction: row;
  align-items: end;
}
.dialogue-actions button {
  display: inline-block;
}
</style>
