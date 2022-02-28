<template>
<Calibration v-if="calibrationOpen" @close="calibrationOpen = false" />
<!-- <div class="calibration-open" v-else @click="calibrationOpen = true">Calibration</div> -->
<nav>
  <div class="tab" :class="{selected: type == 'World'}" @click="() => type = 'World'">Worlds</div>
  <div class="tab" :class="{selected: type == 'Region'}" @click="() => type = 'Region'">Regions</div>
  <div class="tab" :class="{selected: type == 'Industry'}" @click="() => type = 'Industry'">Industries</div>
  <div class="tab" :class="{selected: type == 'Process'}" @click="() => type = 'Process'">Processes</div>
  <div class="tab" :class="{selected: type == 'Project'}" @click="() => type = 'Project'">Projects</div>
  <div class="tab" :class="{selected: type == 'Event'}" @click="() => type = 'Event'">Events</div>
  <div class="tab" :class="{selected: type == 'NPC'}" @click="() => type = 'NPC'">NPCs</div>
  <div class="tab" :class="{selected: type == 'Variable'}" @click="() => type = 'Variable'">Vars</div>
  <div class="tab" :class="{selected: type == 'Const'}" @click="() => type = 'Const'">Consts</div>
</nav>
<div class="items" :class="type">
  <template v-if="type == 'Process'">
    <Process v-for="p in itemsOfCurrentType" :item="p" :key="p.id" />
  </template>
  <template v-if="type == 'Project'">
    <Project v-for="p in itemsOfCurrentType" :item="p" :key="p.id" />
  </template>
  <template v-if="type == 'Region'">
    <Region v-for="r in itemsOfCurrentType" :item="r" :key="r.id" />
  </template>
  <template v-else-if="type == 'Event'">
    <Event v-for="e in itemsOfCurrentType" :item="e" :key="e.id" />
  </template>
  <template v-else-if="type == 'NPC'">
    <NPC v-for="e in itemsOfCurrentType" :item="e" :key="e.id" />
  </template>
  <template v-else-if="type == 'World'">
    <World v-for="e in itemsOfCurrentType" :item="e" :key="e.id" />
  </template>
  <template v-else-if="type == 'Const'">
    <Const v-for="f in itemsOfCurrentType" :item="f" :key="f.id" />
  </template>
  <template v-else-if="type == 'Variable'">
    <Variable v-for="v in itemsOfCurrentType" :item="v" :key="v.id" />
  </template>
  <template v-else-if="type == 'Industry'">
    <Industry v-for="v in itemsOfCurrentType" :item="v" :key="v.id" />
  </template>
</div>
<div class="sidebar">
  <button @click="() => addNew(type)">Add {{type}}</button>
  <div v-for="f, i in filters">
    <div class="filters--header">
      <div>Filter by {{f.name}}</div>
    </div>
    <ul class="filters">
      <li :class="{selected: filter[i] == null}" @click="() => filter[i] = null">All</li>
      <li v-for="v in f.values" :class="{selected: filter[i] == v}" @click="() => filter[i] = v">{{v}}</li>
    </ul>
  </div>
</div>

<ul id="toc" v-if="tocOpen">
  <li v-for="i, item in tableOfContents" :key="i">
    <a :href="`#${i.id}`" :class="{invalid: i.invalid}"><span class="question-icon" v-if="i.questions">?</span>{{i.label || '(empty)'}}</a>
  </li>
</ul>
<div class="toc-meta">
  <div class="toggle" @click="() => tocOpen = !tocOpen">{{tocOpen ? 'Hide' : 'Show'}} TOC</div>
  <div class="count">{{itemsOfCurrentType.length}} items</div>
</div>

<Graph :items="state.items" />

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
import Event from './items/Event.vue';
import Region from './items/Region.vue';
import Project from './items/Project.vue';
import Process from './items/Process.vue';
import World from './items/World.vue';
import Variable from './items/Variable.vue';
import Industry from './items/Industry.vue';
import Const from './items/Const.vue';
import NPC from './items/NPC.vue';
import validate from '../validate';
import Graph from './Graph.vue';
import Calibration from './Calibration.vue';

export default {
  updated() {
    // Kind of hacky way to jump to the hash/id
    // after loading in the data, but only
    // after the first load
    if (!this.loaded) {
      this.loaded = true;
      if (window.location.hash) {
        const el = document.getElementById(window.location.hash.substr(1));
        el && el.scrollIntoView();
      }
    }
  },
  data() {
    const urlParams = new URLSearchParams(window.location.search);
    const type = urlParams.get('type') || 'Event';
    return {
      type,
      loaded: false,
      tocOpen: true,
      filter: [],
      state,
      calibrationOpen: false,
    }
  },
  components: {
    NPC,
    Event,
    World,
    Region,
    Project,
    Process,
    Variable,
    Const,
    Industry,
    Graph,
    Calibration
  },
  methods: {
    addNew(type) {
      let spec = validate[type];
      let data = {
        id: uuid(),
        _created: Date.now(),
        _type: type,
      };
      let validation = {
        invalid: spec.validate.filter((k) => {
          return !spec.validateKey(data, k);
        }),
        questions: []
      };
      data._validation = validation;

      api.update(data);
      scroll(0,0);
    },
  },
  watch: {
    type(_) {
      this.filter = this.filters.map(() => null);
    }
  },
  computed: {
    itemsOfCurrentType() {
      return Object.values(this.state.itemsByType[this.type] || {})
      .filter((item) => {
          return this.filters.every((f, i) => {
            let val = this.filter[i];
            return val == null
            || item[f.key] == val
            || val == '(none)' && (item[f.key] === undefined || item[f.key] === '');
          });
        })
        .sort((a, b) => a._created < b._created ? 1 : -1);
    },
    storyArcs() {
      let allArcs = Object.values(this.state.items)
        .filter((i) => i._type == 'Event' && i.arc).map((e) => e.arc);
      let arcs = [...new Set(allArcs)];
      arcs.sort((a, b) => a.localeCompare(b));
      return arcs;
    },
    filters() {
      switch (this.type) {
        case 'Event':
          return [{
            key: 'type',
            name: 'type',
            values: consts.EVENT_TYPES,
          }, {
            key: 'arc',
            name: 'story arc',
            values: ['(none)'].concat(this.storyArcs),
          }]
        case 'Project':
          return [{
            key: 'type',
            name: 'project type',
            values: ['Initiative', 'Research', 'Policy']
          }, {
            key: 'group',
            name: 'project group',
            values: ['(none)'].concat(consts.PROJECT_GROUPS)
          }]
        case 'Process':
          return [{
            key: 'output',
            name: 'output type',
            values: Object.keys(consts.OUTPUTS)
          }]
        default:
          return [];
      }
    },
    tableOfContents() {
      let spec = validate[this.type];
      let toc = this.itemsOfCurrentType.map((i) => {
        return {
          id: i.id,
          label: i[spec.key],
          invalid: i._validation.invalid.length > 0,
          questions: i._validation.questions.length > 0,
        }
      });
      toc.sort((a, b) => (a.label || '').localeCompare((b.label || '')));
      return toc;
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
  justify-content: space-between;
}
fieldset > div textarea {
  flex-grow: 1;
}
fieldset > div:first-child {
  margin-left: 0;
}
fieldset.big-group > div {
  justify-content: start;
}
.field-group {
  padding: 0.1em 0.5em 0.5em 0.5em;
  background: #f5f5f5;
  border: 1px solid #aaa;
  margin: 0.5em 0;
  border-radius: 0.2em;
}
.field-group label {
	background: #ddd;
	padding: 0.1em 0.0 0.1em 0.25em;
  align-items: center;
}
.field-group h3 {
  margin: 0.4em 0 0 0;
  font-size: 0.7em;
}
.radio {
  display: flex;
  margin-top: 0.5em;
}
.radio > div {
  display: flex;
  align-items: center;
  margin-left: 0.25em;
}
.radio label {
  background: none;
  margin: 0;
}
.radio > div input {
  margin: 0 0.5em;
}
fieldset .checkbox {
  flex: 0.2;
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
  display: flex;
  margin: 0.25em 0;
  justify-content: space-between;
}
.kind-quantities::after {
  content: "";
  flex: auto;
}
.kind-quantities > div {
  width: 90px;
  border: 1px solid #aaa;
  margin-right: 0.5em;
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
	background: #f8f8f8;
	padding: 0.25em 0.5em;
  border: 1px solid #aaa;
  border-radius: 0.2em;
}


ul, li {
  margin: 0;
  padding: 0;
  list-style-type: none;
}

.items .item {
  margin: 2em 0;
}

nav {
  display: flex;
  justify-content: space-around;
  border-bottom: 2px solid #000;
  padding-bottom: 0.5em;
  position: sticky;
  top: 0;
  z-index: 10;
  background: #fff;
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
.filters {
  max-height: 200px;
  overflow-y: scroll;
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

.missing-defined {
  background: #F54242;
  border: 1px solid #613232;
  color: #fff;
  display: inline-block;
  border-radius: 0.2em;
  text-align: center;
  font-size: 0.8em;
  margin: 0.25em 0;
  padding: 0 0.25em;
}

.summary-pill {
  display: inline-flex !important;
  font-size: 0.7em;
  border: 1px solid #000;
  border-radius: 0.2em;
  margin: 0.1em 0.5em 0.1em 0;
  line-height: 1;
  background: #ececec;
  vertical-align: middle;
}
.summary-pill a {
  text-decoration: none;
  color: #000;
}
.summary-pill .has-url:hover {
  background: yellow;
}
.summary-pill > div {
  padding: 0.2em 0.3em;
  border-left: 1px solid #000;
}
.summary-pill > div:first-child {
  display: flex;
  align-items: center;
  border-left: none;
}
.summary-pill.invalid > div:first-child {
  background: #F54242;
}
.summary-pill.invalid {
  background: #F54242;
  color: #fff;
  font-weight: bold;
}
.summary-pill.invalid::before {
  content: '!';
  padding: 0 0.5em;
  font-weight: bold;
  color: #fff;
  line-height: 1.3;
}
.kind-summaries .summary-pill > div:first-child {
  background: #e9d06d;
}
.kinds-summary-label {
  display: inline-block;
  margin-right: 0.5em;
}

.item-meta {
  position: absolute;
  top: 0;
  left: 0;
  transform: translate(0, -50%);
  display: flex;
  width: 100%;
}
.meta-pill {
	font-size: 0.7em;
	border: 1px solid #000;
	padding: 0.1em 0.2em;
	border-radius: 0.2em;
  background: #d282ff;
  margin-right: 0.5em;
  line-height: 1;
}
.meta-pill.invalid {
  background: #F54242 !important;
}
.meta-pill.split-pill {
  display: flex;
  padding: 0;
  background: #dfdfdf;
}
.meta-pill.split-pill > div {
  padding: 0.1em 0.2em;
}
.meta-pill.split-pill > div:first-child {
  border-right: 1px solid #000;
  background: #B2EAEC;
}

.item-summary {
  padding: 0.5em;
  position: relative;
}
.item-summary-details {
  display: flex;
  margin: 1em 0;
  border-top: 1px solid #aaa;
  padding-top: 1em;
}
.item-summary-title {
  font-size: 1.2em;
  font-weight: bold;
  margin: 0.5em 0;
}
.item-summary-flavor {
  margin: 1em 0;
}
.item-summary-notes {
  font-size: 0.7em;
  margin: 1em 0 0 0;
}
.item-summary-notes a {
  color: #3639ff;
}
.item-summary-image {
  max-width: 260px;
  margin: 0 1em 0 0;
}
.item-summary-title.invalid,
.item-summary-desc.invalid {
  color: #F54242;
  text-align: center;
}
.item-missing.invalid {
  color: #F54242;
  font-weight: bold;
  text-align: center;
}
.edit-toggle {
  position: absolute;
  top: 0;
  right: 0;
  z-index: 1;
  transform: translate(0, -50%);
  font-size: 1.2em;
}

p.invalid {
  color: #F54242;
}

h5 {
  margin: 0;
  font-weight: normal;
}

.additional-actions {
  margin: 0.5em 0;
  text-align: right;
}

.outputs-summary, .resources-summary, .byproducts-summary {
  display: inline-block;
}

.image-attribution {
  font-size: 0.7em;
  font-style: italic;
  color: #555;
}

.calibration-open {
  position: fixed;
  left: 1em;
  bottom: 1em;
  text-decortion: underline;
  z-index: 20;
  background: rgba(0,0,0,0.7);
  color: #fff;
  border-radius: 0.2em;
  font-size: 0.9em;
  padding: 0.1em 0.2em;
  cursor: pointer;
}
.calibration-open:hover {
  background: #000;
}

@media only screen and (max-width: 480px) {
  nav {
    flex-wrap: wrap;
    position: static;
  }
  #toc, .toc-meta, .graph, .calibration-open,
  .filters, .filters--header {
    display: none;
  }
  .sidebar {
    top: auto;
    width: auto;
    bottom: 1em;
  }
  .sidebar button {
    background: #111;
    color: #fff;
    border-radius: 0.2em;
    border: 1px solid black;
    font-size: 1.1em;
  }
}
</style>
