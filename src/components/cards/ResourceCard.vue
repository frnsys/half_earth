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
        </div>
        <div>
          <IntensityIcon
            :resource="icon" :intensity="user.intensity" />
          <div class="resource--usage">{{user.displayAmount}}<img :src="icons[icon]"><span class="arrow">⟶</span>{{user.displayProduced}}<img :src="icons[user.output]"></div>
        </div>
      </div>
      <template v-if="!inTop">
        <div class="resource--spacer">...</div>
        <div class="resource--user highlight">
          <div>
            <div>{{current.name}}</div>
            <div>{{currentData.displayAmount}}</div>
          </div>
          <div>
            <IntensityIcon
              :resource="icon" :intensity="currentData.intensity" />
            <div>{{currentData.displayProduced}}<img :src="icons[currentData.output]"></div>
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
import IntensityIcon from './IntensityIcon.vue';

export default {
  props: ['resource'],
  components: {
    Card,
    IntensityIcon,
  },
  data() {
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
    currentData() {
      return this.rankings.find((s) => s.name == this.current.name);
    },
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
.resource--user .card-icon {
  display: flex;
}
.resource--user img {
  height: 14px;
  width: 14px;
  vertical-align: middle;
}
.resource--user .intensity-pips {
  display: flex;
  align-items: center;
  margin-top: 0;
  margin-left: 2px;
  height: 14px;
}
.resource--usage {
  font-size: 14px;
}
.resource--spacer {
  text-align: center;
  margin-bottom: 1em;
  color: #aaa;
}

.arrow {
  color: #727987;
}
</style>
