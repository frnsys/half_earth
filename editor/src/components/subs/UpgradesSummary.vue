<template>
<ul class="upgrades-summary" v-if="upgrades">
  <li v-for="(upgrade, i) in upgrades" :key="upgrade.id">
    <div class="upgrade-level">{{i}}</div>
    <EffectsSummary :effects="upgrade.effects" />
  </li>
</ul>
</template>

<script>
import EffectsSummary from './EffectsSummary.vue';

export default {
  props: ['upgrades'],
  components: {
    EffectsSummary,
  },
}
</script>

<style>
.upgrades-summary li {
  display: flex;
  padding: 0.25em;
  border: 1px solid #aaa;
}
.upgrade-level {
    padding: 0.25em 0.5em;
    background: #111;
    margin-right: 0.25em;
    color: #fff;
    line-height: 1;
}
</style>
