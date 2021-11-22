<template>
<div class="dia-summary" :style="style">
  <template v-if="!nested && (!lines || Object.keys(lines).length === 0)">
    MISSING DIALOGUE
  </template>
  <template v-else>
    <div v-for="line in dialogue" :key="line.id">
      <div class="dia-summary-line">
        <div class="dia-summary-speaker">{{line.speaker || "MISSING SPEAKER"}}</div>
        <div class="dia-summary-text">{{line.text || "MISSING TEXT"}}</div>
      </div>
      <template v-if="Array.isArray(line.next)">
        <div class="dia-summary-branches" :style="{borderBottomColor: branchColor}">
          <div v-for="b, j in line.next" :key="b.id" :style="{backgroundColor: branch==j ? branchColor : ''}" class="dia-summary-branch" @click="branch = j">
            {{line.decision ? b.text : `Branch ${j+1}`}}
            <ConditionsSummary :conditions="b.conditions" />
            <EffectsSummary :effects="b.effects" v-if="line.decision" />
          </div>
        </div>
        <DialogueSummary :root="line.next[branch].line_id" :lines="lines" :nested="true" :color="branchColor"
          v-if="(!line.decision || (line.decision && lines[line.next[branch].line_id].text !== '')) && !(line.decision && line.next[branch]._goto)" />
        <div v-else-if="line.decision && line.next[branch]._goto">
          <div class="dia-summary-line">
            <div class="dia-summary-goto">Go To</div>
            <div class="dia-summary-speaker">{{lines[line.next[branch].line_id].speaker || "MISSING SPEAKER"}}</div>
            <div class="dia-summary-text">{{lines[line.next[branch].line_id].text || "MISSING TEXT"}}</div>
          </div>
        </div>
      </template>
    </div>
  </template>
</div>
</template>

<script>
import EffectsSummary from './EffectsSummary.vue';
import ConditionsSummary from './ConditionsSummary.vue';

export default {
  name: 'DialogueSummary',
  props: ['root', 'lines', 'nested', 'color'],
  components: {
    EffectsSummary,
    ConditionsSummary,
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
          let next = line.next[this.branch].line_id;
          line = this.lines[next];
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
  }
}
</script>

<style>
.dia-summary {
	background: #fff;
  font-size: 12px;
  border-left-style: solid;
}

.dia-summary-line {
  display: flex;
  font-size: 12px;
}
.dia-summary-speaker {
  padding: 0.25em;
  background: #faeaa2;
}
.dia-summary-text {
  padding: 0.25em;
}
.dia-summary-goto {
  padding: 0.25em;
  background: #202020;
  color: #fff;
}

.dia-summary-branches {
  display: flex;
  border-bottom: 4px solid transparent;
}
.dia-summary-branch {
  width: 33.3%;
  padding: 2px;
  border-top: 1px solid #aaa;
  border-right: 1px solid #aaa;
}
.dia-summary-branch:first-child {
  border-left: 1px solid #aaa;
}
.dia-summary-branch:hover {
  background: #eee;
  cursor: pointer;
}
</style>
