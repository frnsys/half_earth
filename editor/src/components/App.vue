<template>
<nav>
  <div class="tab" :class="{selected: type == 'Policy'}" @click="() => type = 'Policy'">Policies</div>
  <div class="tab" :class="{selected: type == 'Event'}" @click="() => type = 'Event'">Events</div>
</nav>
<template v-if="type == 'Policy'">
  <Policy v-for="p in itemsOfType('Policy')" :policy="p" />
  <button class="new-button" @click="() => addNew('Policy')">Add Policy</button>
</template>
<template v-else-if="type == 'Event'">
  <Event v-for="e in itemsOfType('Event')" :event="e" />
  <button class="new-button" @click="() => addNew('Event')">Add Event</button>
</template>
</template>

<script>
import api from '../api';
import uuid from '../uuid';
import state from '../state';
import Event from './Event.vue';
import Policy from './Policy.vue';

export default {
  data() {
    return {
      type: 'Event',
      state
    }
  },
  components: {
    Event,
    Policy,
  },
  methods: {
    itemsOfType(type) {
      return Object.values(this.state.items).filter((i) => i._type == type).sort((a, b) => a._created < b._created);
    },
    addNew(type) {
      api.update({
        id: uuid(),
        _created: Date.now(),
        _type: type,
      });
      scroll(0,0);
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
</style>
