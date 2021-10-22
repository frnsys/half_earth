<template>
<div class="condition">
  <fieldset>
    <div>
      <label>Condition Type</label>
      <select v-model="localData.type" @change="update" :class="flags('type')">
        <option v-for="type in Object.keys(CONDITIONS)" :value="type">{{type}}</option>
      </select>
    </div>
    <div v-if="spec.choices">
      <label>Subtype</label>
      <select v-model="localData.subtype" @change="update" :class="flags('subtype')">
        <option v-for="choice in spec.choices" :value="choice">{{choice}}</option>
      </select>
    </div>
    <div v-if="spec.entity">
      <label>Value</label>
      <select :id="`${localData.id}-entity-select`" v-model="localData.entity" @change="update" :class="flags('entity')">
        <option v-for="choice in itemsOfType(spec.entity)" :value="choice.id">{{choice.name}}</option>
      </select>
    </div>
    <template v-if="spec.compare">
      <div>
        <label>Comparator</label>
        <select v-model="localData.comparator" @change="update" :class="flags('comparator')">
          <option v-for="comparator in COMPARATORS" :value="comparator">{{comparator}}</option>
        </select>
      </div>
      <div>
        <label>Value</label>
        <input type="number" v-model.lazy="localData.value" @blur="update" :class="flags('value')"/>
      </div>
    </template>
    <template v-if="spec.flag">
      <div>
        <label>Flag</label>
        <input type="text" v-model.lazy="localData.value" @blur="update" :class="flags('value')"/>
      </div>
    </template>
  </fieldset>
</div>
</template>

<script>
import state from '../../state';
import consts from '../../consts';
import SlimSelect from 'slim-select';

export default {
  props: ['condition'],
  data() {
    return {
      localData: this.condition
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
      return this.localData.type ? consts.CONDITIONS[this.localData.type] : {};
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
  }
}
</script>

<style>
@import '../../../node_modules/slim-select/dist/slimselect.min.css';
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
