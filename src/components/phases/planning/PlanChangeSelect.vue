<template>
<div class="plan-change-select">
  <header>
    <div>{{page ? page : ''}}</div>
    <div v-if="page !== null" @click="page = null">Back</div>
    <div v-else @click="$emit('close')">Close</div>
  </header>
  <Research v-if="page == PAGES.RESEARCH" />
  <Policies v-else-if="page == PAGES.POLICIES" />
  <Initiatives v-else-if="page == PAGES.INITIATIVES" />
  <div class="planning--page" v-else>
    <div class="planning--menu">
      <button @click="selectPage(PAGES.RESEARCH)">
        <img :src="icons.research" />
        <div>Research</div>
      </button>
      <button @click="selectPage(PAGES.POLICIES)">
        <img :src="icons.policy" />
        <div>Policies</div>
      </button>
      <button @click="selectPage(PAGES.INITIATIVES)">
        <img :src="icons.initiative" />
        <div>Initiatives</div>
      </button>
    </div>
  </div>
</div>
</template>

<script>
import Research from './Research.vue';
import Policies from './Policies.vue';
import Initiatives from './Initiatives.vue';

const PAGES = {
  RESEARCH: 'Research',
  POLICIES: 'Policies',
  INITIATIVES: 'Initiatives',
};

export default {
  components: {
    Research,
    Policies,
    Initiatives,
  },
  created() {
    this.PAGES = PAGES;
  },
  data() {
    return {
      page: null
    }
  },
  methods: {
    selectPage(p) {
      this.page = p;
      this.$emit('page', p);
    }
  }
}
</script>

<style>
.plan-change-select {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  z-index: 2;
  background: #ff6b56;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding-top: 1em; /* Space for the hud */
}
.plan-change-select > header {
  color: #fff;
  text-align: right;
  padding: 0.5em;
  display: flex;
  justify-content: space-between;
}
.plan-change-select > header > div:first-child {
  text-decoration: underline;
}
</style>
