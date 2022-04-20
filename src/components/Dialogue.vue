<template>
<div class="dialogue" v-if="current !== null">
  <div class="dialogue--inner">
  <div class="dialogue--speech">
    <div class="dialogue--speaker" v-if="line.speaker !== '[GAME]'">
      <img
        :src="`/assets/characters/${line.speaker}.webp`"
        @error="fallbackPortrait" />
    </div>
    <div class="dialogue--body" @click="advance">
      <div class="dialogue--speaker-name" v-if="line.speaker !== '[GAME]'">
        {{line.speaker}}
      </div>
      <div class="dialogue--text" ref="text"></div>
      <div class="dialogue--effects" v-if="effects && revealed && isLastLine">
        <Effects :effects="effects"  />
      </div>
    </div>
  </div>
  <div class="dialogue--choices">
    <template v-if="revealed">
      <div v-if="isLastLine" class="dialogue--choice" @click="end">
        (Continue)
      </div>
      <template v-else-if="line.decision" v-for="branch in line.next" :key="branch.id">
        <div class="dialogue--choice" @click="(ev) => selectChoice(ev, branch)">
          {{branch.text}}
        </div>
      </template>
      <div v-else class="dialogue--choice" @click="advance">
        (Next)
      </div>
    </template>
  </div>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import display from '/src/display/display';
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
  props: ['dialogue', 'effects', 'eventId', 'regionId'],
  components: {
    Effects,
  },
  data() {
    return {
      current: this.dialogue.root,
      revealed: false,
    }
  },
  mounted() {
    if (this.current !== null) {
      this.play();
    }

    document.addEventListener('keydown', this.onKeydown);

  },
  beforeDestroy: function () {
    document.removeEventListener('keydown', this.onKeydown)
  },
  watch: {
    dialogue(dialogue) {
      if (dialogue !== null) {
        this.revealed = false;
        this.current = dialogue.root;
        this.$emit('started');
        this.$nextTick(() => {
          this.play();
        });
      }
    }
  },
  computed: {
    line() {
      let line = this.dialogue.lines[this.current];
      return {
        ...line,
        text: this.dialogue.context ? parseText(line.text, this.dialogue.context) : line.text,
      };
    },
    isLastLine() {
      return this.line.next == null;
    },

  },
  methods: {
    fallbackPortrait(e){
      e.target.src ='/assets/characters/' + this.line.speaker + '.png'
    },
    onKeydown(e){
      if (VERSION === 'dev' && e.key === 'Escape') {
        this.end();
      } else if (e.key === 'Enter' && this.current !== null) {
        if (this.isLastLine) {
          this.end();
        } else if (!this.line.decision) {
          this.advance();
        }
      }
    },
    play() {
      this.revealed = false;
      if (this.$refs.text) {
        this.$refs.text.innerHTML = '';
        let el = document.createElement('div');
        el.innerHTML = this.line.text;
        if (this.line.text.length > 0) {
          if (this.revealAnim) clearInterval(this.revealAnim);
          revealChars(this.$refs.text, extractChars(el), {
            onStart: (revealAnim) => this.revealAnim = revealAnim
          }).then(() => {
            this.revealed = true;
          });
        } else {
          this.revealed = true;
        }
      }
    },
    end() {
      if (this.revealAnim) clearInterval(this.revealAnim);
      this.current = null;
      this.revealed = false;
      this.$emit('done');
    },
    selectChoice(ev, branch) {
      ev.stopImmediatePropagation();

      // this.eventID will be undefined
      // for project outcome dialogues.
      // The whole dialogue system was really written with
      // events in mind; it'd be a pretty big rewrite to
      // fully support project dialogues with branch effects.
      // So we just assume project dialogues won't have branch effects
      // which, at time of writing, none of them do.
      if (this.eventId !== undefined) {
        game.applyBranchEffects(this.eventId, this.regionId, branch.id);
      }

      this.current = branch.line_id;
      if (this.current !== null) {
        this.play();
      } else {
        this.end();
      }
    },
    advance() {
      if (this.current === null) return;
      if (this.revealed && !this.isLastLine && !this.line.decision) {
        this.nextLine();
        this.$emit('advanced');
      } else {
        this.skipReveal();
      }
    },
    nextLine() {
      if (Array.isArray(this.line.next)) {
        // This occurs for project outcomes, which have no associated event
        // For project outcomes we effectively don't support branching
        if (this.eventId === undefined) {
          let branch = this.line.next[0];
          this.current = branch.line_id;
        } else {
          let branch = this.line.next.find((b) => {
            return game.evalBranchConditions(this.eventId, this.regionId, b.id);
          });
          this.current = branch.line_id;
        }
      } else {
        this.current = this.line.next;
      }
      if (this.current === null) {
        this.end();
      } else {
        this.play();
      }
    },
    skipReveal() {
      if (this.revealAnim) clearInterval(this.revealAnim);
      if (this.current !== null) {
        if (this.$refs.text) {
          this.$refs.text.innerHTML = this.line.text;
          this.revealed = true;
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

  background-image: url('/assets/backgrounds/screen-door.png');
  background-repeat: repeat;

  display: flex;
  flex-direction: column;
  user-select: none;
  z-index: 11;
  align-items: center;

  image-rendering: pixelated;

  animation: fade-in 0.5s;
}

.dialogue--inner{
  height: 100%;
  display: flex;
  flex-direction: column;

}
@media only screen and (min-width: 481px) {
  .dialogue--inner{
    width:481px;
  }
}
@media only screen and (max-width: 480px) {
  .dialogue--inner{
    width: 100%;
  }
}

.dialogue--speech {
  position: relative;
  flex: 1;
  display: flex;
  align-items: flex-end;
  width: 100%;
}
.dialogue--body {
  /* background: #FCF6BB; */
  /* background: #222; */
  /* color: #fff; */
  color: #000;
  background: #fff;
  border: 1px solid #000;
  margin: 1em 0;
  padding: 0.75em;
  width: 100%;
  border-radius: 0.3em;
  box-shadow: 2px 2px 0 rgb(0 0 0 / 70%);
  width: 100%;

  font-family: 'W95FA';
  animation: fade-in 0.5s;
}
.dialogue--speaker img {
  width: 72px;
  display: block;
  margin: 0 auto;
  image-rendering: auto;
}
.dialogue--speaker {
  position: absolute;
  left: 0;
  bottom: 0;
  transform: translate(0%, 100%);
  background: #222;
  border-radius: 0.3em;
  padding: 0.05em;
  max-width: 82px;
  box-shadow: 2px 2px 0 rgb(0 0 0 / 70%);
}
.dialogue--speaker-name {
  font-size: 0.7em;
  text-transform: uppercase;
  font-family: 'Inter', sans-serif;
  font-weight: 600;
  letter-spacing: 0.01em;
  margin-bottom: 0.5em;
}

.dialogue--text{
  font-size: 1.1rem;
  letter-spacing: 0.01em;
  line-height: 130%;
}

.dialogue--text img {
  height: 20px;
  vertical-align: middle;
  image-rendering: auto;
  margin-bottom: 2px;
}

.dialogue--choices {
  text-align: right;
  flex: 1;
  margin-left: 5em;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
}

.dialogue--choice {
  background: #fff;
  text-align: right;
  display: inline-block;
  padding: 0.5em;
  margin-left: 0.5em;
  margin-bottom: 0.2em;
  cursor: pointer;
  border: 1px solid #000;
  border-radius: 0.3em;
  user-select: none;
  box-shadow: 2px 2px 0 rgb(0 0 0 / 70%);
  max-width: calc(100% - 94px);

  font-size:1.1em;
  font-family: 'W95FA';
}
.dialogue--choice:hover {
  background: var(--colour-pink);
}

.dialogue--effects .effect--text{
  font-family: 'Inter';
  padding: 1em 1em 0 1em;
  margin: 0.5em 0;
  border-top: 1px dotted black;
  image-rendering: auto;
}
</style>
