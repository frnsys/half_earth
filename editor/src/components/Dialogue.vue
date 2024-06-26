<template>
<div class="dialogue" @click="advance" v-if="current !== null">
  <div class="dialogue--speech">
    <div class="dialogue--speaker">
      <div class="dialogue--speaker-name">{{line.speaker}}</div>
    </div>
    <div class="dialogue--body">
      <div class="dialogue--text" ref="text"></div>
    </div>
  </div>
  <div class="dialogue--choices">
    <template v-if="revealed">
      <div v-if="isLastLine" class="dialogue--choice" @click="end">
        (End)
      </div>
      <template v-else-if="line.decision">
        <h2>Choice</h2>
        <template v-for="branch in line.next" :key="`choice-${branch.id}`">
          <div class="dialogue--choice" @click="(ev) => selectChoice(ev, branch)">
            {{branch.text}}
          </div>
        </template>
      </template>
      <!-- choose from branches -->
      <template v-else-if="Array.isArray(line.next)">
        <h2>Branch</h2>
        <template v-for="branch in line.next" :key="`branch-${branch.id}`">
          <div class="dialogue--choice" @click="(ev) => selectChoice(ev, branch)">
            BRANCH: "{{dialogue.lines[branch.line_id].text}}"
          </div>
        </template>
      </template>
    </template>
  </div>
</div>
</template>

<script>
// import display from 'lib/display';

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
  // return display.fillIcons(display.fillVars(text, context));
  // Don't want to port over all the icons and what not
  return text;
}

export default {
  props: ['dialogue'],
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
  },
  watch: {
    dialogue(dialogue) {
      if (dialogue !== null) {
        this.revealed = false;
        this.current = dialogue.root;
        this.play();
      }
    }
  },
  computed: {
    line() {
      let line = this.dialogue.lines[this.current];
      let text = this.dialogue.context ? parseText(line.text, this.dialogue.context) : line.text;
      if (!text) text = '[MISSING TEXT]';
      return {
        ...line,
        text
      };
    },
    isLastLine() {
      return this.line.next == null;
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
    selectChoice(ev, branch) {
      ev.stopImmediatePropagation();

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
      } else {
        this.skipReveal();
      }
    },
    nextLine() {
      if (Array.isArray(this.line.next)) {
        let branch = this.line.next.find((b) => {
          // TODO choose branches somewhere else
          return game.evalBranchConditions(this.eventId, this.regionId, b.id);
        });
        this.current = branch.line_id;
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
  margin-left: 0.5em;
  cursor: pointer;
  border: 1px solid #000;
  border-radius: 0.3em;
  user-select: none;
  box-shadow: 2px 2px 6px rgb(0 0 0 / 70%);
}
.dialogue--choice:hover {
  background: #FF6B56;
}

.dialogue--choices h2 {
  margin: 0 0 0.25em 0;
  border-bottom: 1px solid #000;
}
</style>
