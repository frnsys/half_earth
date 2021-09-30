<template>
<nav>
  <div class="tab" :class="{selected: type == 'Process'}" @click="() => type = 'Process'">Processes</div>
  <div class="tab" :class="{selected: type == 'Project'}" @click="() => type = 'Project'">Projects</div>
  <div class="tab" :class="{selected: type == 'Region'}" @click="() => type = 'Region'">Regions</div>
  <div class="tab" :class="{selected: type == 'Event'}" @click="() => type = 'Event'">Events</div>
  <div class="tab" :class="{selected: type == 'Earth'}" @click="() => type = 'Earth'">Earths</div>
</nav>
<div class="items">
  <template v-if="type == 'Process'">
    <Process v-for="p in itemsOfType" :item="p" />
  </template>
  <template v-if="type == 'Project'">
    <Project v-for="p in itemsOfType" :item="p" />
  </template>
  <template v-if="type == 'Region'">
    <Region v-for="r in itemsOfType" :item="r" />
  </template>
  <template v-else-if="type == 'Event'">
    <Event v-for="e in itemsOfType" :item="e" />
  </template>
  <template v-if="type == 'Earth'">
    <Earth v-for="e in itemsOfType" :item="e" />
  </template>
</div>
<div class="sidebar">
  <button @click="() => addNew(type)">Add {{type}}</button>
  <div class="filters--header">
    <div>Filter by {{filterType}}</div>
    <div class="toggle" @click="() => filtersOpen = !filtersOpen">{{filtersOpen ? 'Hide' : 'Show'}}</div>
  </div>
  <ul class="filters" v-if="filtersOpen">
    <li :class="{selected: filter == null}" @click="() => filter = null">All</li>
    <li v-for="f in filters" :class="{selected: filter == f}" @click="() => filter = f">{{f}}</li>
  </ul>
</div>

<ul id="toc" v-if="tocOpen">
  <li v-for="i in tableOfContents">
    <a :href="`#${i.id}`" :class="{invalid: i.invalid}"><span class="question-icon" v-if="i.questions">?</span>{{i.label || '(empty)'}}</a>
  </li>
</ul>
<div class="toc-meta">
  <div class="toggle" @click="() => tocOpen = !tocOpen">{{tocOpen ? 'Hide' : 'Show'}} TOC</div>
  <div class="count">{{itemsOfType.length}} items</div>
</div>

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
import Region from './Region.vue';
import Project from './Project.vue';
import Process from './Process.vue';
import Earth from './Earth.vue';

export default {
  data() {
    return {
      type: 'Project',
      tocOpen: true,
      filtersOpen: true,
      filter: null,
      state
    }
  },
  components: {
    Event,
    Earth,
    Region,
    Project,
    Process,
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
          return this.filter == null || i[this.filterKey] == this.filter;
        })
        .sort((a, b) => a._created < b._created ? 1 : -1);
    },
    storyArcs() {
      let arcs = Object.values(this.state.items)
        .filter((i) => i._type == 'Event' && i.arc).map((e) => e.arc);
      return [...new Set(arcs)];
    },
    filterKey() {
      switch (this.type) {
        case 'Event':
          return 'arc';
        case 'Project':
          return 'type';
        case 'Process':
          return 'output';
        default:
          return '';
      }
    },
    filterType() {
      switch (this.type) {
        case 'Event':
          return 'story arc';
        case 'Project':
          return 'project type';
        case 'Process':
          return 'output type';
        default:
          return '';
      }
    },
    filters() {
      switch (this.type) {
        case 'Event':
          return this.storyArcs;
        case 'Project':
          return ['Project', 'Research', 'Policy'];
        case 'Process':
          return Object.keys(consts.OUTPUTS);
        default:
          return [];
      }
    },
    tableOfContents() {
      let key;
      let required;
      let questions;
      switch (this.type) {
        case 'Event':
          key = 'name';
          required = ['name', 'description'];
          questions = ['name', 'description', 'notes'];
          break;
        case 'Project':
          key = 'name';
          required = ['name', 'description'];
          questions = ['name', 'description', 'notes'];
          break;
        case 'Process':
          key = 'name';
          required = ['name', 'description'];
          questions = ['name', 'description', 'notes'];
          break;
        case 'Region':
          key = 'name';
          required = ['name', 'description', 'countries', 'satiety', 'safety', 'health', 'outlook'];
          questions = ['name', 'description', 'countries', 'notes'];
          break;
        case 'Earth':
          key = 'year';
          required =['year', 'emissions', 'atmospheric_ghg', 'biodiversity', 'temperature', 'ozone_damage'];
          questions = ['year'];
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
  margin-top: 0.6em;
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
.field-group {
  padding: 0.1em 0.5em 0.5em 0.5em;
  background: #f5f5f5;
  border: 1px solid #aaa;
  margin: 0.5em 0;
}
.field-group label {
	background: #ddd;
	padding: 0.1em 0.0 0.1em 0.25em;
}
.field-group h3 {
  margin: 0.4em 0 0 0;
  font-size: 1em;
}
.checkbox {
	display: inline-block;
	background: #eee;
	padding: 0.1em 0.25em 0.2em 0.1em;
	border: 1px solid #aaa;
  margin-right: 1em;
  margin-bottom: 0.5em;
}
.checkbox > input, .checkbox > label {
  width: auto;
  display: inline;
}
.checkbox .tip {
  font-size: 0.75em;
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
.units {
  color: #777;
  font-size: 0.9em;
  margin-top: 1px;
  margin-left: 5px;
}
.kind-quantities {
  display: grid;
  margin: 0.25em 0;
  grid-gap: 5px;
  justify-content: space-between;
  grid-template-columns: repeat(7, 90px);
}
.kind-quantities::after {
  content: "";
  flex: auto;
}
.kind-quantities > div {
  width: 90px;
  border: 1px solid #aaa;
}
.kind-quantities > div input {
  border: 0;
  border-radius: 0;
}
.kind-quantities > div label {
  background: #eee;
  margin-top: 0;
  padding: 1px;
  border-bottom: 1px solid #ccc;
  text-decoration: none;
}

/* Hide number input arrows */
/* Chrome, Safari, Edge, Opera */
input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

/* Firefox */
input[type=number] {
  -moz-appearance: textfield;
}

.item {
  position: relative;
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
  border-bottom: 2px solid #000;
  padding-bottom: 0.5em;
}
.tab {
  cursor: pointer;
  padding: 0.25em 0.5em;
}
.tab:hover, .tab.selected {
  background: #000;
  color: #fff;
  border-radius: 0.2em;
}

.sidebar {
  position: fixed;
  right: 1em;
  top: 1em;
  width: 160px;
}
.filters--header {
  display: flex;
  justify-content: space-between;
  border-bottom: 1px solid #000;
  font-size: 0.75em;
  margin-top: 1em;
}
.filters--header .toggle {
  font-size: 0.7em;
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
.toc-meta {
  position: fixed;
  left: 0.6em;
  top: 1em;
  z-index: 1;
  font-size: 0.8em;
  display: flex;
}
.question-icon {
  color: #000;
  background: #EFCF40;
  font-size: 0.8em;
  vertical-align: super;
	border-radius: 10em;
	padding: 0 0.3em;
}

.toggle {
  cursor: pointer;
  color: #aaa;
  background: #fff;
  border: 1px solid #aaa;
  padding: 0.1em;
  border-radius: 0.2em;
}
.toggle:hover {
  color: #000;
  border: 1px solid #000;
}
.count {
  margin-left: 0.5em;
  line-height: 1.5;
}
</style>
