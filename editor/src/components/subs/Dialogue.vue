<template>
<div class="dia" ref="root" :style="style">
  <div v-for="line, i in dialogue" :key="line.id" :id="`line-${line.id}`">
    <div class="dia-line">
      <select class="dia-line-speaker" v-model="line.speaker">
        <option v-for="k in SPEAKERS" :value="k">{{k}}</option>
      </select>
      <input class="dia-line-text"
        type="text" placeholder="Dialogue line"
        v-model.lazy="line.text" @keyup.enter="addLine(line.id)" />
      <button v-if="i > 0" @click="deleteLine(line.id)">X</button>
      <template v-if="i == dialogue.length - 1 && !Array.isArray(line.next)">
        <button @click="createResponses(line.id)"><img src="/static/response.png"></button>
        <button @click="createBranches(line.id)"><img src="/static/branch.png"></button>
      </template>
    </div>
    <template v-if="Array.isArray(line.next)">
      <div class="dia-line-branches" :style="{borderBottomColor: branchColor}">
        <div v-for="b, j in line.next" :key="b.id" :style="{backgroundColor: branch==j ? branchColor : ''}" class="dia-line-branch" @click="branch = j">
          <Conditions :conditions="b.conditions">
            <template v-slot:title>
              <template v-if="line.decision">
                <textarea
                  class="dia-line-response-text"
                  placeholder="Response"
                  v-model.lazy="b.text" />
              </template>
              <template v-else>
                Branch {{j+1}}
              </template>
            </template>
            <template v-slot:actions>
              <button @click="deleteBranch(line.id, j)">X</button>
            </template>
          </Conditions>
          <Effects :effects="b.effects" v-if="line.decision" />
          <template v-if="line.decision">
            <label>
              Go To
              <input type="checkbox" v-model="b._goto" class="dia-line-goto" @change="goToChange(b)">
            </label>
            <div v-if="b._goto">
              <select v-model="b.line_id">
                <template v-for="l in Object.values(lines)">
                  <option :value="l.id" v-if="l.text !== ''">{{l.text}}</option>
                </template>
              </select>
            </div>
          </template>
        </div>
        <button v-if="line.next.length < 3" @click="addBranch(line.id)">+ {{line.decision ? 'Response' : 'Branch'}}</button>
      </div>
      <Dialogue v-if="line.next[branch] && !(line.decision && line.next[branch]._goto)" :root="line.next[branch].line_id" :lines="lines" :nested="true" :color="branchColor" />
      <div v-else-if="line.next[branch] && line.decision && line.next[branch]._goto">
        <div class="dia-summary-line">
          <div class="dia-summary-goto">Go To</div>
          <div class="dia-summary-speaker">{{lines[line.next[branch].line_id].speaker || "MISSING SPEAKER"}}</div>
          <div class="dia-summary-text">{{lines[line.next[branch].line_id].text || "MISSING TEXT"}}</div>
        </div>
      </div>
    </template>
  </div>
</div>
</template>

<script>
import uuid from '../../uuid';
import Effects from './Effects.vue';
import Conditions from './Conditions.vue';

export default {
  name: 'Dialogue',
  props: ['id', 'root', 'lines', 'nested', 'color'],
  components: {
    Effects,
    Conditions,
  },
  data() {
    return {
      // Selected branch
      branch: 0,
    }
  },
  computed: {
    dialogue() {
      let line = this.lines[this.root];
      let dialogue = [];
      while (line) {
        dialogue.push(line);
        if (Array.isArray(line.next)) {
          let next = line.next[this.branch];
          if (next) {
            let nextId = next.line_id;
            line = this.lines[nextId];
          }
          break; // Stop at the first branch
        } else {
          line = this.lines[line.next];
        }
      }
      return dialogue;
    },
    style() {
      return {
        'border-left-width': `${this.nested ? 4 : 0}px`,
        'border-left-color': this.color
      }
    },
    branchColor() {
      const colors = ['#43CC70', '#6096E8', '#E7CB5D'];
      return colors[this.branch];
    }
  },
  methods: {
    createLine() {
      let id = uuid();
      this.lines[id] = {
        id,
        speaker: 'Gossy',
        text: '',
        next: null,
      };
      return this.lines[id];
    },
    focusLine(id) {
      this.$refs.root
        .querySelector(`#line-${id} input`).focus();
    },
    addLine(id) {
      let line = this.createLine();
      line.next = this.lines[id].next;
      line.speaker = this.lines[id].speaker;
      line.decision = this.lines[id].decision;
      this.lines[id].decision = false;
      this.lines[id].next = line.id;
      this.$nextTick(() => this.focusLine(line.id));
    },
    deleteLine(id) {
      let prevNext = this.lines[id].next;
      Object.values(this.lines).forEach((line) => {
        if (line.next == id) {
          line.next = prevNext;
        } else if (Array.isArray(line.next)) {
          let idx = line.next.findIndex((n) => n.line_id == id);
          if (idx >= 0) {
            line.next[idx].line_id = prevNext;
          }
        }
      });
    },
    createResponses(id) {
      let line = this.createLine();
      this.lines[id].decision = true;
      this.lines[id].next = [{
        id: uuid(),
        line_id: line.id,
        conditions: [],
        effects: [],
        text: '',
      }];

    },
    createBranches(id) {
      let line = this.createLine();
      this.lines[id].next = [{
        id: uuid(),
        line_id: line.id,
        conditions: [],
      }];
    },
    addBranch(id) {
      let line = this.createLine();
      this.lines[id].next.push({
        id: uuid(),
        line_id: line.id,
        conditions: [],
        effects: [],
        text: ''
      });
    },
    deleteBranch(id, i) {
      if (confirm('Are you sure you want to delete branch?')) {
        this.lines[id].next.splice(i, 1);
        this.branch = 0;
        if (this.lines[id].next.length === 0) {
          this.lines[id].next = null;
          this.lines[id].decision = false;
        }
      }
    },
    goToChange(b) {
      // Restore the original line this branch pointed to
      if (b._goto) {
        b._original_line_id = b.line_id;
      } else {
        b.line_id = b._original_line_id;
      }
    }
  }
}
</script>

<style>
.dia {
  border-left-style: solid;
}
.dia select {
  background: #fff;
  border: none;
  border-bottom: 1px solid #aaa;
}
.dia-line {
  display: flex;
}
.dia-line-speaker {
  width: 10em;
}
.dia-line-text {
  flex: 1;
}
.dia button {
  background: #eee;
  padding: 0.25em;
  vertical-align: middle;
  display: flex;
  border: 1px solid #aaa;
  border-radius: 0.2em;
  margin-left: 1px;
  cursor: pointer;
  align-items: center;
}
.dia button:hover {
  background: #ddd;
}
.dia button img {
  height: 16px;
}

.dia-line-branches {
  display: flex;
  justify-content: flex-start;
  border-bottom: 4px solid transparent;
}
.dia-line-branch {
  width: 33.3%;
  padding: 2px;
  border-right: 1px solid #aaa;
}
.dia-line-branch:last-child {
  border-right: none;
}
.dia-line-branch:hover {
  background: #eee;
  cursor: pointer;
}

.dia-line-branch .effects {
  margin: 0.25em 0 0 0;
  padding: 0;
  border: none;
  background: none;
}

.dia-line-response-text {
  min-width: auto;
  flex: 1;
  height: 2em;
  font-size: 0.9em;
}

.dia-line-goto {
  width: 1.5em;
  margin-top: -0.0em;
}
</style>

