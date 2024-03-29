<template>
<div class="effect">
  <fieldset>
    <div>
      <label>Type</label>
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
    <div class="effect-params">
      <div v-if="spec.params" v-for="name in Object.keys(spec.params)">
        <template v-if="spec.params[name] == Number">
          <label>{{name}}</label>
          <input type="number" :value="localData.params[name]" @blur="(ev) => updateParam(name, ev.target.value)" :class="paramFlags(name)"/>
        </template>
        <template v-if="spec.params[name] == String">
          <label>{{name}}</label>
          <input type="text" :value="localData.params[name]" @blur="(ev) => updateParam(name, ev.target.value)" :class="paramFlags(name)"/>
        </template>
        <template v-if="spec.params[name] == Boolean">
          <label>{{name}}</label>
          <input type="checkbox" :value="localData.params[name]" @blur="(ev) => updateParam(name, ev.target.checked)" :class="paramFlags(name)"/>
        </template>
      </div>
    </div>
    <div class="effect-hide">
      <label>👁️</label>
      <input type="checkbox" v-model="localData.hidden" :true-value="false" :false-value="true" @change="update" />
    </div>
  </fieldset>
  <div class="effect-desc">{{desc}}</div>
</div>
</template>

<script>
import state from '../../state';
import consts from '../../consts';
import SlimSelect from 'slim-select';

export default {
  props: ['effect'],
  data() {
    if (this.effect.hidden === undefined) {
      this.effect.hidden = false;
    }
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
    },
    desc() {
      let spec = this.spec;
      if (typeof spec.desc === 'string' || spec.desc instanceof String) {
        return spec.desc;
      } else {
        return spec.desc[this.localData.subtype];
      }
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
    updateHidden(ev) {
      this.localData.hidden = !ev.target.checked;
      this.update();
    },
    updateParam(name, val) {
      if (!this.localData.params) {
        this.localData.params = {};
      }
      this.localData.params[name] = val;
      this.update();
    },
    itemsOfType(type) {
      if (type == 'IconEvent') {
        return Object.values(state.itemsByType['Event']).filter((i) => i.type == 'Icon');
      } else {
        return Object.values(state.itemsByType[type]);
      }
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
        invalid: this.localData.params[name] === undefined || this.localData.params[name] === ''
      };
    }
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

.effect-params {
  flex-direction: row;
}
.effect-params > div {
  flex: 1;
  margin-right: 0.5em;
}
.effect-params > div:last-child {
  margin-right: 0;
}

.effect-desc {
  font-size: 0.75em;
  margin: 0.25em 0 0.5em 0;
  color: #555;
  text-align: center;
}

.effect-hide {
  max-width: 16px;
}
.effect-hide input[type=checkbox] {
  margin: 3px 0;
}
</style>
