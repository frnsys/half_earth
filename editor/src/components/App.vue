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
      type: 'Policy',
      state
    }
  },
  components: {
    Event,
    Policy,
  },
  methods: {
    itemsOfType(type) {
      return Object.values(this.state.items).filter((i) => i._type == type);
    },
    addNew(type) {
      api.update({
        id: uuid(),
        _type: type,
      });
    }
  }
}
</script>

<style>
* {
  box-sizing: border-box;
}

main {
  max-width: 480px;
  margin: 0 auto;
}
label {
  font-size: 0.8em;
  display: flex;
  justify-content: space-between;
  margin-top: 0.3em;
}

input, textarea, select, button {
  width: 100%;
  max-width: 100%;
}

textarea {
  min-height: 80px;
}

ul, li {
  margin: 0;
  padding: 0;
  list-style-type: none;
}

li {
  margin: 2em 0;
  background: #f0f0f0;
  border: 1px solid #aaa;
  padding: 0.5em 1em 1em 1em;
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
  margin: 3em 0 0 0;
}
</style>
