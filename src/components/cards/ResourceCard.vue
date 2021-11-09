<template>
<Card class="resource">
  <template v-slot:header>
    <div>{{name}}</div>
  </template>
  <template v-slot:body>
    <div class="resource--users">
      <div class="resource--user" v-for="user in top" :class="{highlight: user.name == current.name}">
        <div>
          <div>{{user.name}}</div>
          <div>{{format(user.amount)}}</div>
        </div>
        <div>
          <IntensityIcon
            :resource="icon" :intensity="user.intensity" />
          <div>{{display.output(user.produced, consts.outputs.keys[user.output])}}{{consts.icons[consts.outputs.keys[user.output]]}}</div>
        </div>
      </div>
      <template v-if="!inTop">
        <div class="resource--spacer">...</div>
        <div class="resource--user highlight">
          <div>
            <div>{{current.name}}</div>
            <div>{{format(currentAmount)}}</div>
          </div>
          <div>
            <IntensityIcon
              :resource="icon" :intensity="currentIntensity" />
            <div>{{display.output(currentProduced, consts.outputs.keys[current.output])}}{{consts.icons[consts.outputs.keys[current.output]]}}</div>
          </div>
        </div>
      </template>
    </div>
  </template>
  <template v-slot:back>
  </template>
  <template v-slot:footer>
  </template>
</Card>

</template>

<script>
import Card from './Card.vue';
import display from 'lib/display';
import IntensityIcon from './IntensityIcon.vue';

export default {
  props: ['resource'],
  components: {
    Card,
    IntensityIcon,
  },
  created() {
    this.display = display;
  },
  data() {
    console.log(this.resource)
    return {
      ...this.resource,
    };
  },
  computed: {
    top() {
      return this.rankings.slice(0, 5);
    },
    inTop() {
      return this.top.some((s) => s.name == this.current.name);
    },
    currentAmount() {
      return this.rankings.find((s) => s.name == this.current.name).amount;
    },
    currentIntensity() {
      return this.rankings.find((s) => s.name == this.current.name).intensity;
    },
    currentProduced() {
      return this.rankings.find((s) => s.name == this.current.name).produced;
    }
  }
}
</script>

<style>
.resource--user {
  margin: 0.5em 0;
  padding: 0 0.25em;
}
.resource--user > div {
  display: flex;
  justify-content: space-between;
}
.resource--user.highlight {
  background: #f5f9c7;
  color: #111;
  border-radius: 0.2em;
}
.resource--user .intensity-icon {
  display: flex;
}
.resource--user .intensity-icon img {
  height: 18px;
  width: 18px;
}
.resource--user .intensity-pips {
  display: flex;
  align-items: center;
  margin-top: 0;
  margin-left: 2px;
}
.resource--spacer {
  text-align: center;
  margin-bottom: 1em;
  color: #aaa;
}
</style>
