<template>
<nav>
  <div class="tab" :class="{selected: type == 'Policy'}" @click="() => type = 'Policy'">Policies</div>
  <div class="tab" :class="{selected: type == 'Event'}" @click="() => type = 'Event'">Events</div>
</nav>
<div class="items">
  <template v-if="type == 'Policy'">
    <Policy v-for="p in itemsOfType" :policy="p" />
  </template>
  <template v-else-if="type == 'Event'">
    <Event v-for="e in itemsOfType" :event="e" />
  </template>
</div>
<div class="sidebar">
  <button @click="() => addNew(type)">Add {{type}}</button>
  <h6>Filter by {{type == 'Event' ? 'story arc' : 'policy type'}}</h6>
  <ul class="filters">
    <li :class="{selected: filter == null}" @click="() => filter = null">All</li>
    <li v-for="f in filters" :class="{selected: filter == f}" @click="() => filter = f">{{f}}</li>
  </ul>
</div>

<ul id="toc" v-if="tocOpen">
  <li v-for="i in tableOfContents">
    <a :href="`#${i.id}`" :class="{invalid: i.invalid}"><span class="question-icon" v-if="i.questions">?</span>{{i.label || '(empty)'}}</a>
  </li>
</ul>
<div class="toc-toggle" @click="() => tocOpen = !tocOpen">{{tocOpen ? 'Hide' : 'Show'}} TOC</div>

<datalist id="arcs">
  <option v-for="arc in storyArcs">{{arc}}</option>
</datalist>
</template>

<script>
import api from '../api';
import uuid from '../uuid';
import util from '../util';
import state from '../state';
import consts from '../consts';
import Event from './Event.vue';
import Policy from './Policy.vue';

export default {
  data() {
    return {
      type: 'Event',
      tocOpen: true,
      filter: null,
      state
    }
  },
  components: {
    Event,
    Policy,
  },
  methods: {
    addNew(type) {
      api.update({
        id: uuid(),
        _created: Date.now(),
        _type: type,
      });
      scroll(0,0);
    }
  },
  watch: {
    type(_) {
      this.filter = null;
    }
  },
  computed: {
    itemsOfType() {
      return Object.values(this.state.items)
        .filter((i) => i._type == this.type)
        .filter((i) => {
          return this.filter == null || (this.type == 'Event' ? i.arc : i.type) == this.filter;
        })
        .sort((a, b) => a._created < b._created);
    },
    storyArcs() {
      let arcs = Object.values(this.state.items)
        .filter((i) => i._type == 'Event' && i.arc).map((e) => e.arc);
      return [...new Set(arcs)];
    },
    filters() {
      return this.type == 'Event' ? this.storyArcs : consts.POLICY_TYPE;
    },
    tableOfContents() {
      let key;
      let required;
      let questions;
      switch (this.type) {
        case 'Event':
          key = 'body';
          required = ['body', 'area', 'conditions', 'effects'];
          questions = ['body', 'conditions', 'effects', 'variations', 'responses', 'notes'];
          break;
        case 'Policy':
          key = 'name';
          required = ['name', 'type', 'description', 'requirements', 'effects'];
          questions = ['name', 'description', 'requirements', 'effects', 'notes'];
          break;
      }
      return this.itemsOfType.map((i) => ({
        id: i.id,
        label: i[key],
        invalid: required.some((k) => {
          let val = i[k];
          return !(val && val.length > 0);
        }),
        questions: questions.some((k) => {
          let val = i[k];
          return val && val.includes('?');
        })
      }));
    }
  }
}
</script>

<style>
* {
  box-sizing: border-box;
}

html, body {
  font-family: 'Arial', sans-serif;
}

main {
  max-width: 720px;
  margin: 0 auto;
}

label {
  font-size: 0.7em;
  display: flex;
  justify-content: space-between;
  margin-top: 0.3em;
  font-family: 'Arial', sans-serif;
}
input, textarea, select {
  width: 100%;
  font-family: 'Arial', sans-serif;
}
textarea {
  min-width: 100%;
  max-width: 100%;
  resize: none;
}
input, textarea, select {
  border: 1px solid #999;
  border-radius: 2px;
}
button {
  cursor: pointer;
  font-family: 'Arial', sans-serif;
}
fieldset {
  border: none;
  display: flex;
  padding: 0;
}
fieldset > div {
  flex: 1;
  margin-left: 0.5em;
  display: flex;
  flex-direction: column;
}
fieldset > div textarea {
  flex-grow: 1;
}
fieldset > div:first-child {
  margin-left: 0;
}
input.invalid, textarea.invalid, select.invalid {
  background: #ff00001c;
}
input.question, textarea.question, select.question {
  background: #ffefa5;
}
input.title, textarea.title {
  font-size: 1.5em;
  border: none;
  border-radius: 0;
  border-bottom: 1px solid #000;
  font-weight: bold;
}

.item {
  position: relative;
}
.indicators {
  position: absolute;
  left: calc(100% + 0.5em);
}
.indicator {
  font-size: 0.75em;
  padding: 0.25em;
  white-space: nowrap;
  border-radius: 0.25em;
  font-weight: bold;
  margin-bottom: 0.25em;
  display: inline-block;
}
.indicator--missing {
  color: #fff;
  background: #f54242;
  border: 1px solid #a22727;
}
.indicator--question {
  color: #000;
  background: #efcf40;
  border: 1px solid #927c18;
}

ul, li {
  margin: 0;
  padding: 0;
  list-style-type: none;
}

.items li {
  margin: 4em 0;
}

nav {
  display: flex;
  justify-content: space-around;
}
.tab {
  cursor: pointer;
}
.tab:hover, .tab.selected {
  border-bottom: 2px solid #000;
}

h6 {
  margin-bottom: 0;
  font-weight: normal;
  border-bottom: 1px solid #000;
}
.sidebar {
  position: fixed;
  right: 1em;
  top: 1em;
}
.filters li {
  margin: 0.25em 0;
  cursor: pointer;
  color: #aaa;
}
.filters li:hover {
  color: #000;
}
.filters li.selected {
  color: #000;
  text-decoration: underline;
}

.notes {
  margin-top: 0.5em;
  padding: 0 0.25em 0.25em 0.25em;
  background: #f0f0f0;
  border: 1px solid #ccc;
}
.notes label {
  cursor: pointer;
  user-select: none;
}
.notes-icon {
	color: #fff;
	background: #484848;
	border-radius: 10em;
	padding: 0 0.45em;
}

#toc {
  top: 0;
  left: 0;
  width: 260px;
  padding: 2em 0.5em 0.5em 0.5em;
  position: fixed;
  height: 100vh;
  overflow-y: scroll;
  background: #fff;
}
#toc li {
  margin: 0.25em 0;
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow-x: hidden;
  color: #aaa;
  border-bottom: 1px solid transparent;
}
#toc li a {
  color: #aaa;
  text-decoration: none;
}
#toc li a.invalid {
  color: #f54242;
}
#toc li a.question {
  color: #efcf40;
}
#toc li:hover {
  color: #000;
  border-bottom: 1px solid #000;
}
#toc li:hover a {
  color: #000;
}
.toc-toggle {
  position: fixed;
  left: 0.6em;
  top: 1em;
  font-size: 0.8em;
  cursor: pointer;
  z-index: 1;
  color: #aaa;
  background: #fff;
  border: 1px solid #aaa;
  padding: 0.1em;
  border-radius: 0.2em;
}
.toc-toggle:hover {
  color: #000;
  text-decoration: underline;
}
.question-icon {
  color: #000;
  background: #EFCF40;
  font-size: 0.8em;
  vertical-align: super;
	border-radius: 10em;
	padding: 0 0.3em;
}
</style>
