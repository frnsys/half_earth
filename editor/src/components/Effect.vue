<template>
<div>
  <fieldset>
    <div>
      <label>Effect Type</label>
      <select v-model="localData.type" @change="update" :class="flags('type')">
        <option v-for="type in Object.keys(EFFECTS)" :value="type">{{type}}</option>
      </select>
    </div>
    <div v-if="spec.choices">
      <label>Subtype</label>
      <select v-model="localData.subtype" @change="update" :class="flags('subtype')">
        <option v-for="choice in spec.choices" :value="choice">{{choice}}</option>
      </select>
    </div>
    <div v-if="spec.entity">
      <label>Target</label>
      <select :id="`${localData.id}-entity-select`" v-model="localData.entity" @change="update" :class="flags('entity')">
        <option v-for="choice in itemsOfType(spec.entity)" :value="choice.id">{{choice.name}}</option>
      </select>
    </div>
    <div>
      <div v-if="spec.params" v-for="name in Object.keys(spec.params)">
        <template v-if="spec.params[name] == Number">
          <label>{{name}}</label>
          <input type="number" :value="localData.params[name]" @change="(ev) => updateParam(name, ev.target.value)" :class="paramFlags(name)"/>
        </template>
      </div>
    </div>
  </fieldset>
</div>
</template>

<script>
import state from '../state';
import consts from '../consts';
import SlimSelect from 'slim-select';

export default {
  props: ['effect'],
  data() {
    return {
      localData: this.effect
    };
  },
  mounted() {
    this.setupSelect();
  },
  updated() {
    this.setupSelect();
  },
  computed: {
    spec() {
      return this.localData.type ? consts.EFFECTS[this.localData.type] : {};
    }
  },
  methods: {
    setupSelect() {
      if (this.select) this.select.destroy();
      let sel = `${this.localData.id}-entity-select`;
      let el = document.getElementById(sel);
      if (el) {
        this.select = new SlimSelect({
          select: el
        });
      }
    },
    update() {
      this.$emit('update', this.localData);
    },
    updateParam(name, val) {
      if (!this.localData.params) {
        this.localData.params = {};
      }
      this.localData.params[name] = val;
      this.update();
    },
    itemsOfType(type) {
      return Object.values(state.items)
        .filter((i) => i._type == type)
        .sort((a, b) => a._created < b._created ? 1 : -1);
    },
    flags(key) {
      let invalid = false;
      let val = this.localData[key];
      switch (key) {
        case 'subtype':
          invalid = val === undefined || !this.spec.choices.includes(val);
          break;
        default:
          invalid = val === undefined
      }
      return {
        invalid
      };
    },
    paramFlags(name) {
      return {
        invalid: this.localData.params[name] === undefined || this.localData.params[name] == ''
      };
    }
  }
}
</script>

<style>
@import '../../node_modules/slim-select/dist/slimselect.min.css';
.ss-main .ss-single-selected {
  height: 20px;
  font-size: 14px;
  padding: 2px;
  border-radius: 2px;
  border: 1px solid #999;
  color: #000;
}

.ss-main.invalid .ss-single-selected {
  background: #F6DADA;
}
</style>
