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
      <Effects :effects="effects" v-if="effects && revealed" />
    </div>
  </div>
  <div class="dialogue--choices">
    <template v-if="revealed && isLastLine">
      <div v-if="current.choices.length === 0" class="dialogue--choice" @click="end">
        (Continue)
      </div>
      <template v-else v-for="choice, i in current.choices" :key="i">
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
import display from 'lib/display';
import {clone} from 'lib/util';
import Effects from 'components/Effects.vue';

// Extract "chars" which might be
// actual chars or be HTML elements
function extractChars(el) {
	let chars = [];
  el.childNodes.forEach((n) => {
    switch (n.nodeType) {
      case Node.TEXT_NODE:
        chars = chars.concat(n.textContent.split(''));
        return;
      case Node.ELEMENT_NODE:
        if (n.childNodes.length === 0) {
          chars.push(n);
        } else {
          let node = n.cloneNode();
          node.innerHTML = '';
          chars.push({
            node,
            chars: extractChars(n)
          });
        }
        return;
    }
  });
  return chars;
}

// Reveal "chars"
function revealChars(parentEl, chars, {speed, onReveal, onStart}) {
  speed = speed || 3.5;
  let currentNode = null;
  return new Promise((resolve, reject) => {
    let revealAnim = setInterval(() => {
   		let char = chars.shift();
      if (char == '<END>') {
        currentNode = null;
      } else if (typeof char == 'string') {
      	if (!currentNode || currentNode.nodeType == Node.TEXT_NODE) {
        	currentNode = document.createTextNode('');
          parentEl.appendChild(currentNode);
        }
        currentNode.textContent += char;
      } else if (char instanceof HTMLElement){
      	parentEl.appendChild(char);
      } else {
      	currentNode = char.node;
        parentEl.appendChild(currentNode);
        chars = char.chars.concat(['<END>']).concat(chars);
      }
      if (onReveal) onReveal(char);
      if (chars.length == 0) {
        clearInterval(revealAnim);
        resolve();
      }
    }, 100/speed);
    if (onStart) onStart(revealAnim);
  });
}

// Parse special entities out of text
function parseText(text, context) {
  return display.fillIcons(display.fillVars(text, context));
}

export default {
  props: ['dialogue', 'effects'],
  components: {
    Effects,
  },
  data() {
    return {
      current: clone(this.dialogue),
      revealed: false,
    }
  },
  mounted() {
    if (this.current) {
      this.play();
    }
  },
  watch: {
    dialogue(dialogue) {
      if (dialogue !== null) {
        this.revealed = false;
        this.current = dialogue;
        this.play();
      }
    }
  },
  computed: {
    line() {
      let line = this.current.lines[0];
      console.log(line);
      return {
        text: this.dialogue.context ? parseText(line.text) : line.text,
        speaker: line.speaker,
      };
    },
    isLastLine() {
      return this.current.lines.length <= 1;
    },
  },
  methods: {
    play() {
      this.revealed = false;
      this.$refs.text.innerHTML = '';
      let el = document.createElement('div');
      el.innerHTML = this.line.text;
      revealChars(this.$refs.text, extractChars(el), {
        onStart: (revealAnim) => this.revealAnim = revealAnim
      }).then(() => {
        this.revealed = true;
      });
    },
    end() {
      if (this.revealAnim) clearInterval(this.revealAnim);
      this.current = null;
      this.revealed = false;
      this.$emit('done');
    },
    selectChoice(ev, i) {
      ev.stopImmediatePropagation();

      let choice = this.current.choices[i];
      this.$emit('select', choice.id);

      this.current = choice.dialogue;
      if (this.current) {
        this.play();
      } else {
        this.end();
      }
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
        this.end();
      } else {
        this.current.lines.shift();
        this.play();
      }
    },
    skipReveal() {
      if (this.revealAnim) clearInterval(this.revealAnim);
      if (this.current !== null) {
        this.$refs.text.innerHTML = this.line.text;
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
  z-index: 10;
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

.dialogue--text img {
  width: 16px;
  vertical-align: middle;
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
