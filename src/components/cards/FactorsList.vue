<template>
<div class="factors--users">
  <div class="factors--total">
    <div>Total:</div>
    <div>{{factors.total}}<span class="type-total" v-if="consts.maxValues[factors.type]">/{{consts.maxValues[factors.type]}}</span><img :src="icons[factors.icon]" /></div>
  </div>
  <div class="factors--user" v-for="user in relevantFactors" :class="{highlight: factors.current && user.name == factors.current.name}">
    <div>
      <div>{{user.name}}</div>
    </div>
    <div>
      <template v-if="user.type == 'Region'">
        <IntensityIcon
          resource="wealth" :intensity="user.intensity" />
        <div class="factors--usage">{{user.displayAmount}}<img :src="icons[factors.icon]"></div>
      </template>
      <template v-else-if="user.type !== 'Project' && user.type !== 'Event'">
        <IntensityIcon
          :resource="factors.icon" :intensity="user.intensity" />
        <div class="factors--usage"><template v-if="user.displayProduced !== null">{{user.displayProduced}}<img :src="icons[user.output]"><span class="factor-relation">{{factors.type == 'emissions' ? 'makes' : 'uses'}}</span></template>{{user.displayAmount}}<img :src="icons[factors.icon]"></div>
      </template>
      <template v-else>
        <div class="factors--usage factors--usage-solo">{{user.displayAmount || user.amount || 0}}<img :src="icons[factors.icon]"></div>
      </template>
    </div>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import IntensityIcon from './IntensityIcon.vue';

export default {
  props: ['factors'],
  components: {
    IntensityIcon,
  },
  computed: {
    relation() {
      if (this.factors.type == 'emissions') {
        return 'makes';
      } else if (this.factors.type == 'biodiversity') {
        return 'causes';
      } else {
        return 'uses';
      }
    },
    relevantFactors() {
      return state.factors[this.factors.type].filter((user) => user.displayProduced !== 0);
    },
  }
}
</script>

<style>
.factors--users {
  padding-top: 1em;
}
.factors--users > div {
  margin: 0.5em 0;
  padding: 0 0.25em;
}
.factors--users img {
  height: 14px;
  width: 14px;
  vertical-align: middle;
  image-rendering: auto;
}
.factors--user > div, .factors--total {
  display: flex;
  justify-content: space-between;
}
.factors--user.highlight {
  background: #f5f9c7;
  color: #111;
  border-radius: 0.2em;
}
.factors--user .card-icon {
  display: flex;
}
.factors--user .intensity-pips {
  display: flex;
  align-items: center;
  margin-top: 0;
  margin-left: 2px;
  height: 14px;
}
.factors--usage {
  font-size: 12px;
}

.factors--usage img{
  image-rendering: auto;
}

.factors--total {
  border: 1px solid #fff;
  position: absolute;
  top: 0.5em;
  left: 1.5em;
  right: 1.5em;
  background: #222222;
  border-radius: 0.2em;
  padding: 0.1em 0.25em !important;
}

/* hacky, but so exhausted at this point */
.factors--usage-solo {
  width: 100%;
  text-align: right;
}

.type-total {
  color: rgba(255,255,255,0.6);
}

.factor-relation {
  margin: 0 3px;
  font-size: 0.8em;
  color: #a5a5a5;
}
</style>
