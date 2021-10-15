<template>
<div class="dialogue">
  <div class="dialogue--speech">
    <div class="dialogue--speaker">
      <img :src="`/assets/characters/${current.speaker}.png`" />
    </div>
    <div class="dialogue-body">
      <p>{{current.text}}</p>
    </div>
  </div>
  <div class="dialogue--choices">
    <div v-if="current.choices.length === 0" class="dialogue--choice" @click="endDialogue">
      <p>(Continue)</p>
    </div>
    <template v-else v-for="choice, i in current.choices">
      <div class="dialogue--choice" @click="selectChoice(i)">
        <p>{{choice.text}}</p>
      </div>
    </template>
  </div>
</div>
</template>

<script>
import state from '../state';

export default {
  props: ['dialogue'],
  data() {
    return {
      state,
      current: this.dialogue,
    }
  },
  methods: {
    endDialogue() {
      this.$emit('done');
    },
    selectChoice(i) {
      let choice = this.current.choices[i];
      this.$emit('select', choice.id);

      this.current = this.current.choices[i].dialogue;
      if (!this.current) {
        this.endDialogue();
      }
    },
  },
}
</script>

<style>
/* Dialogue overlaid everything else */
.dialogue {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  top: 0;
  padding: 1em;
  background: rgba(255,255,255,0.8);
}

.dialogue--speech {
  background: #fff;
  margin: 1em 0;
  padding: 0.5em;
  position: relative;
  border: 1px solid #000;
}
.dialogue--speaker img {
  width: 72px;
}
.dialogue--speaker {
  position: absolute;
  left: 0;
  bottom: 0;
  transform: translate(10%, 75%);
}

.dialogue--body p {
  margin: 0;
}

.dialogue--choices {
  text-align: right;
}
.dialogue--choice {
  background: #fff;
  text-align: right;
  display: inline-block;
  padding: 0.5em;
  cursor: pointer;
  border: 1px solid #000;
}
.dialogue--choice:hover {
  background: #FF6B56;
}
</style>
