<template>
<div class="dialogue" @click="advance" v-if="current">
  <div class="dialogue--speech">
    <div class="dialogue--speaker">
      <img :src="`/assets/characters/${line.speaker}.png`" onerror="this.src='/assets/placeholders/character.png';" />
      <div class="dialogue--speaker-name">{{line.speaker}}</div>
    </div>
    <div class="dialogue--body" ref="body"></div>
  </div>
  <div class="dialogue--choices">
    <template v-if="showChoices">
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
      readyNext: false,
      showChoices: false,
      lineIdx: 0,
      current: this.dialogue,
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
        this.current = newDialogue;
        this.lineIdx = 0;
        this.readyNext = false;
        this.showChoices = false;
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
    }
  },
  methods: {
    endDialogue() {
      if (this.revealAnim) clearInterval(this.revealAnim);
      this.current = null;
      this.$emit('done');
    },
    playDialogue() {
      this.readyNext = false;
      this.revealText(this.lineText).then(() => {
        if (this.isLastLine) {
          this.showChoices = true;
        } else {
          this.readyNext = true;
        }
      });
    },
    selectChoice(i) {
      let choice = this.current.choices[i];
      this.$emit('select', choice.id);

      this.current = this.current.choices[i].dialogue;
      this.showChoices = false;
      this.readyNext = false;
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
    advance() {
      if (this.current === null) return;
      if (this.readyNext) {
        this.nextLine();
      } else {
        this.skipReveal();
      }
    },
    nextLine() {
      if (!this.showChoices) {
        // If this is the last line
        // and there are no choices to advance
        // the dialogue further, just end it
        if (this.lastLine) {
          this.endDialogue();
        } else {
          this.lineIdx++;
          this.playDialogue();
        }
      }
    },
    skipReveal() {
      if (this.revealAnim) clearInterval(this.revealAnim);
      if (this.current !== null) {
        this.$refs.body.innerText = this.lineText;
        if (this.isLastLine) {
          this.showChoices = true;
        } else {
          this.readyNext = true;
        }
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
  user-select: none;
}
.dialogue--speaker img {
  width: 72px;
}
.dialogue--speaker {
  position: absolute;
  left: 0;
  bottom: 0;
  transform: translate(10%, 95%);
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
  user-select: none;
}
.dialogue--choice:hover {
  background: #FF6B56;
}
</style>
