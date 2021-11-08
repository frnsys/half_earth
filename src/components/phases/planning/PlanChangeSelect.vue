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
  <Processes v-else-if="page == PAGES.PROCESSES" />
  <div class="plan-change-select--menu" v-else>
    <button @click="page = PAGES.RESEARCH">Research</button>
    <button @click="page = PAGES.POLICIES">Policies</button>
    <button @click="page = PAGES.INITIATIVES">Initiatives</button>
    <button @click="page = PAGES.PROCESSES">Processes</button>
  </div>
</div>
</template>

<script>
import Research from './Research.vue';
import Policies from './Policies.vue';
import Processes from './Processes.vue';
import Initiatives from './Initiatives.vue';

const PAGES = {
  RESEARCH: 'Research',
  POLICIES: 'Policies',
  INITIATIVES: 'Initiatives',
  PROCESSES: 'Processes',
};

// TODO events

export default {
  components: {
    Research,
    Policies,
    Initiatives,
    Processes,
  },
  created() {
    this.PAGES = PAGES;
  },
  data() {
    return {
      page: null
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
.plan-change-select--menu {
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  padding: 1em;
  flex: 1;
}
</style>
