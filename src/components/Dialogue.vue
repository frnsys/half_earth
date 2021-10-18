<template>
<div class="dialogue" @click="skipReveal">
  <div class="dialogue--speech">
    <div class="dialogue--speaker">
      <img :src="`/assets/characters/${current.speaker}.png`" />
    </div>
    <div class="dialogue-body" ref="body"></div>
  </div>
  <div class="dialogue--choices" v-if="showChoices">
    <div v-if="current.choices.length === 0" class="dialogue--choice" @click="endDialogue">
      <p>(Continue)</p>
    </div>
    <template v-else v-for="choice, i in current.choices">
      <div class="dialogue--choice" @click="(ev) => {
          ev.stopImmediatePropagation();
          this.selectChoice(i)
        }">
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
      showChoices: false,
      current: this.dialogue,
    }
  },
  mounted() {
    this.playDialogue();
  },
  methods: {
    endDialogue() {
      if (this.revealAnim) clearInterval(this.revealAnim);
      this.current = null;
      this.$emit('done');
    },
    playDialogue() {
      this.revealText(this.current.text).then(() => {
        this.showChoices = true;
      });
    },
    selectChoice(i) {
      let choice = this.current.choices[i];
      this.$emit('select', choice.id);

      this.current = this.current.choices[i].dialogue;
      this.showChoices = false;
      this.playDialogue();
      if (!this.current) {
        this.endDialogue();
      }
    },
    revealText(text, speed) {
        let revealed = '';
        speed = speed || 3;
        const chars = text.split('');
        return new Promise((resolve, reject) => {
          this.revealAnim = setInterval(() => {
            // Have to keep the revealed text
            // separate from innerText
            // b/c innerText will strip trailing spaces
            revealed += chars.shift();
            this.$refs.body.innerText = revealed;
            if (chars.length == 0) {
              clearInterval(this.revealAnim);
              this.revealAnim = null;
              resolve();
            }
          }, 100/speed);
        });
    },
    skipReveal() {
      if (this.revealAnim) clearInterval(this.revealAnim);
      if (this.current !== null) {
        this.$refs.body.innerText = this.current.text;
        this.showChoices = true;
      }
    }
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
