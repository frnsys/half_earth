<template>
<h1>Editor</h1>
<label>Config Name:</label>
<input type="text" v-model="name" />
<textarea v-model="config"></textarea>

<div>
  <button @click="save">Save</button>
</div>

<hr />
<div>
  <label>Load config:</label>
  <select v-model="selected">
    <option v-for="name in configs">{{name}}</option>
  </select>
  <button @click="load">Load</button>
</div>
</template>

<script>
import state from '../src/state';

const dbHost = 'http://localhost:8000'

export default {
  data() {
    return {
      name: '',
      selected: 'default',
      config: JSON.stringify(state, null, 4),
      configs: ['default']
    };
  },
  mounted() {
    fetch(`${dbHost}/list`)
      .then((resp) => resp.json()).then((json) => {
        this.configs = ['default'].concat(json.configs);
      });
  },
  methods: {
    load() {
      if (this.selected === 'default') {
        this.name = '';
        this.config = JSON.stringify(state, null, 4);
      } else {
        fetch(`${dbHost}/load/${this.selected}`)
          .then((resp) => resp.json()).then((json) => {
            this.name = this.selected;
            this.config = JSON.stringify(json.config, null, 4);
          });
      }

    },
    save() {
      if (this.name.length === 0) {
        alert('You need to specify a name for the config.');
        return;

      } else if (this.configs.includes(this.name)) {
        if (!confirm('This will overwrite an existing config with the same name.')) {
          return;
        }
      }

      let config;
      try {
        config = JSON.parse(this.config);
      } catch (e) {
        alert('Failed to parse JSON, check that the syntax is correct.');
        return;
      }

      fetch(`${dbHost}/save`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name: this.name,
          config: config
        }),
      });
    },
  }
}
</script>

<style>
body {
  font-family: monospace;
}

button {
  padding: 0.5em 1em;
  border: 1px solid #000;
  cursor: pointer;
  background: #fff;
}
button:hover {
  background: #000;
  color: #fff;
}

textarea {
  display: block;
  width: 640px;
  height: 400px;
}

hr {
  height: 1px;
  border: none;
  margin: 2em 0;
  background: #000;
}
</style>
