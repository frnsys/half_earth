<template>
<nav>
  <div class="tab" :class="{selected: type == 'Policy'}" @click="() => type = 'Policy'">Policies</div>
  <div class="tab" :class="{selected: type == 'Event'}" @click="() => type = 'Event'">Events</div>
</nav>
<template v-if="type == 'Policy'">
  <Policy v-for="p in itemsOfType" :policy="p" />
  <button class="new-button" @click="() => addNew('Policy')">Add Policy</button>
</template>
<template v-else-if="type == 'Event'">
  <Event v-for="e in itemsOfType" :event="e" />
  <button class="new-button" @click="() => addNew('Event')">Add Event</button>
</template>

<ul id="toc" v-if="tocOpen">
  <li v-for="i in tableOfContents"><a :href="`#${i.id}`">{{i.label || '(empty)'}}</a></li>
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
import Event from './Event.vue';
import Policy from './Policy.vue';

export default {
  data() {
    return {
      type: 'Event',
      tocOpen: true,
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
  computed: {
    itemsOfType() {
      return Object.values(this.state.items).filter((i) => i._type == this.type).sort((a, b) => a._created < b._created);
    },
    storyArcs() {
      let arcs = Object.values(this.state.items).filter((i) => i._type == 'Event' && i.arc).map((e) => e.arc);
      return [...new Set(arcs)];
    },
    tableOfContents() {
      let key;
      switch (this.type) {
        case 'Event':
          key = 'body';
          break;
        case 'Policy':
          key = 'name';
          break;
      }
      return this.itemsOfType.map((i) => ({
        id: i.id,
        label: i[key],
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

ul, li {
  margin: 0;
  padding: 0;
  list-style-type: none;
}

li {
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

.new-button {
  position: fixed;
  right: 1em;
  top: 1em;
}

.notes {
  margin-top: 0.5em;
  padding: 0 0.25em 0.25em 0.25em;
  background: #f0f0f0;
  border: 1px solid #ccc;
}
.notes label {
  cursor: pointer;
  text-decoration: underline;
  user-select: none;
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
}
.toc-toggle:hover {
  color: #000;
  text-decoration: underline;
}
</style>