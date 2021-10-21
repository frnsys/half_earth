<template>
<div class="dialogue" @click="advance" v-if="current">
  <div class="dialogue--speech">
    <div class="dialogue--speaker">
      <img
        :src="`/assets/characters/${line.speaker}.png`"
        onerror="this.src='/assets/placeholders/character.png';" />
      <div class="dialogue--speaker-name">{{line.speaker}}</div>
    </div>
    <div class="dialogue--body">
      <div class="dialogue--text" ref="text"></div>
      <div class="dialogue--effects" v-if="effects && revealed">
        <div class="dialogue--effect" v-for="effect in effectTexts">
          {{effect}}
        </div>
      </div>
    </div>
  </div>
  <div class="dialogue--choices">
    <template v-if="revealed && isLastLine">
      <div v-if="current.choices.length === 0" class="dialogue--choice" @click="endDialogue">
        (Continue)
      </div>
      <template v-else v-for="choice, i in current.choices">
        <div class="dialogue--choice" @click="(ev) => selectChoice(ev, i)">
          {{choice.text}}
        </div>
      </template>
    </template>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import {describeEffect} from '/src/effects';

export default {
  props: ['dialogue', 'effects'],
  data() {
    return {
      lineIdx: 0,
      current: this.dialogue,
      revealed: false,
    }
  },
  mounted() {
    if (this.current) {
      this.playDialogue();
    }
  },
  watch: {
    dialogue(newDialogue) {
      if (newDialogue !== null) {
        this.revealed = false;
        this.current = newDialogue;
        this.lineIdx = 0;
        this.playDialogue();
      }
    }
  },
  computed: {
    line() {
      return this.current.lines[this.lineIdx];
    },
    isLastLine() {
      return this.lineIdx == this.current.lines.length - 1;
    },
    lineText() {
      let text = this.line.text;
      if (this.dialogue.context) {
        let vars = [...text.matchAll('{([a-z]+)}')];
        for (const match of vars) {
          text = text.replaceAll(match[0], this.dialogue.context[match[1]]);
        }
      }
      return text;
    },
    effectTexts(effect) {
      let texts = [];
      if (this.effects) {
        this.effects.forEach((e) => {
          let text = describeEffect(e);
          if (text) texts.push(text);
        });
      }
      return texts;
    }
  },
  methods: {
    playDialogue() {
      this.revealed = false;
      this.revealText(this.lineText).then(() => {
        this.revealed = true;
      });
    },
    endDialogue() {
      if (this.revealAnim) clearInterval(this.revealAnim);
      this.current = null;
      this.revealed = false;
      this.$emit('done');
    },
    selectChoice(ev, i) {
      ev.stopImmediatePropagation();

      let choice = this.current.choices[i];
      this.$emit('select', choice.id);

      this.lineIdx = 0;
      this.current = choice.dialogue;
      if (this.current) {
        this.playDialogue();
      } else {
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
            this.$refs.text.innerText = revealed;
            if (chars.length == 0) {
              clearInterval(this.revealAnim);
              this.revealAnim = null;
              resolve();
            }
          }, 100/speed);
        });
    },
    advance() {
      if (this.current === null) return;
      if (this.revealed && !this.isLastLine) {
        this.nextLine();
      } else {
        this.skipReveal();
      }
    },
    nextLine() {
      // If this is the last line
      // and there are no choices to advance
      // the dialogue further, just end it
      if (this.isLastLine && this.current.choices.length === 0) {
        this.endDialogue();
      } else {
        this.lineIdx++;
        this.playDialogue();
      }
    },
    skipReveal() {
      if (this.revealAnim) clearInterval(this.revealAnim);
      if (this.current !== null) {
        this.$refs.text.innerText = this.lineText;
        this.revealed = true;
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
  display: flex;
  flex-direction: column;
  user-select: none;
}

.dialogue--speech {
  position: relative;
  flex: 1;
  display: flex;
  align-items: end;
}
.dialogue--body {
  background: #fff;
  border: 1px solid #000;
  margin: 1em 0;
  padding: 0.5em;
  width: 100%;
  border-radius: 0.3em;
  box-shadow: 2px 2px 6px rgb(0 0 0 / 70%);
}
.dialogue--speaker img {
  width: 72px;
  display: block;
  margin: 0 auto;
}
.dialogue--speaker {
  position: absolute;
  left: 0;
  bottom: 0;
  transform: translate(0%, 95%);
  background: #222;
  border-radius: 0.3em;
  padding: 0.25em;
  max-width: 82px;
  box-shadow: 2px 2px 6px rgb(0 0 0 / 70%);
}
.dialogue--speaker-name {
	font-size: 0.8em;
	line-height: 1;
	text-align: center;
	background: #222;
	color: #fff;
	padding: 0 0 0.1em 0;
	border-radius: 0.2em;
}

.dialogue--effect {
  font-size: 0.8em;
  text-align: right;
}

.dialogue--choices {
  text-align: right;
  flex: 1;
}
.dialogue--choice {
  background: #fff;
  text-align: right;
  display: inline-block;
  padding: 0.5em;
  cursor: pointer;
  border: 1px solid #000;
  border-radius: 0.3em;
  user-select: none;
  box-shadow: 2px 2px 6px rgb(0 0 0 / 70%);
}
.dialogue--choice:hover {
  background: #FF6B56;
}
</style>
