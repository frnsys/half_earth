<template>
<div>
  <label>
    Upgrades
    <button @click="addUpgrade">+ Upgrade</button>
  </label>
  <div class="upgrades">
    <div class="field-group" v-for="(upgrade, i) in localData" :key="upgrade.id">
      <div class="upgrade--meta">
        <label>
          Upgrade {{i}}
        </label>
        <div>
          <label>
            Cost/Years
            <Tip>Political capital cost for policies, otherwise years to completion. If the parent cost is dynamic, this is the multiplier for the output demand.</Tip>
          </label>
          <input type="number" v-model="upgrade.cost" />
        </div>
        <div class="subitem-actions">
          <button @click="() => deleteUpgrade(upgrade)">X</button>
          <button v-if="i > 0" @click="() => moveUpgrade(i, i-1)">ᐱ</button>
          <button v-if="i < localData.length - 2" @click="() => moveUpgrade(i, i+1)">ᐯ</button>
        </div>
      </div>
      <Effects :effects="upgrade.effects" @update="saveEffects(i, $event)" />
    </div>
  </div>
</div>
</template>

<script>
import uuid from '../../uuid';
import Tip from '../Tip.vue';
import Effects from './Effects.vue';

export default {
  props: ['id', 'upgrades'],
  components: {
    Tip, Effects,
  },
  data() {
    return {
      localData: this.upgrades || []
    };
  },
  methods: {
    update() {
      this.$emit('update', this.localData);
    },
    addUpgrade() {
      this.localData.push({
        id: uuid(),
        effects: [],
        cost: 0,
      });
      this.update();
    },
    // https://stackoverflow.com/a/6470794
    moveUpgrade(fromIdx, toIdx) {
      let item = this.localData[fromIdx];
      this.localData.splice(fromIdx, 1);
      this.localData.splice(toIdx, 0, item);
      this.update();
    },
    deleteUpgrade(upgrade) {
      this.localData = this.localData.filter((e) => e != upgrade);
      this.update();
    },
    saveEffects(i, effects) {
      this.localData[i].effects = effects;
      this.update();
    },
  }
}
</script>

<style>
.upgrades {
  display: flex;
  flex-wrap: wrap;
}
.upgrades .field-group {
  flex-basis: 50%;
}
.upgrade--meta {
  display: flex;
}
.upgrade--meta label {
  flex: 1;
  margin-right: 0.5em;
}
.upgrades .subitem-actions {
  display: flex;
}
.upgrades .effects {
  border: none;
  padding: 0;
}
</style>
