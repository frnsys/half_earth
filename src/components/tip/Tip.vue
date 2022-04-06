<template>
<transition name="opacityfade">
<div class="tip-wrapper" v-if="show" :class="{overlay: card}" ref="overlay" @click="dismiss">
  <div class="tip" ref="tip">
    <div class="tip--icon" v-if="icon">
      <img :src="icons[icon]">
      <img :src="icons[subicon]" v-if="subicon" class="tip--subicon">
      <img :src="icons[supicon]" v-if="supicon" class="tip--supicon">
    </div>
    <div class="tip--body" v-if="text" v-html="text"></div>
  </div>
  <transition appear name="appear-bounceup">
  <div class="tip--card">
    <ProcessCard v-if="card && card.type == 'Process'" :process="card.data" />
    <ProjectCard v-if="card && card.type == 'Project'" :project="card.data" />
    <IndustryCard v-if="card && card.type == 'Industry'" :industry="card.data" />
    <FactorsCard v-if="card && card.type == 'Factors'" :factors="card.data" />
    <RegionCard v-if="card && card.type == 'Region'" :region="card.data" />
    <NPCCard v-if="card && card.type == 'NPC'" :npc="card.data" />
    <Cards v-if="card && card.type == 'Processes'">
      <ProcessCard v-for="p in card.data" :process="p" />
    </Cards>
  </div>
  </transition>
</div>
</transition>
</template>

<script>
import Cards from 'components/cards/Cards.vue';
import NPCCard from 'components/cards/NPCCard.vue';
import ProcessCard from 'components/cards/ProcessCard.vue';
import ProjectCard from 'components/cards/ProjectCard.vue';
import IndustryCard from 'components/cards/IndustryCard.vue';
import FactorsCard from 'components/cards/FactorsCard.vue';
import RegionCard from 'components/cards/RegionCard.vue';

export default {
  components: {
    Cards,
    NPCCard,
    ProcessCard,
    ProjectCard,
    IndustryCard,
    FactorsCard,
    RegionCard,
  },
  data() {
    return {
      show: false,
      icon: null,
      text: null,
      card: null,
      subicon: null,
      supicon: null,
    }
  },
  created() {
    document.body.addEventListener('click', (ev) => {
      if (this.card === undefined) {
        this.show = false;
      }
    });

    // Probably very hacky
    window.tip = this;
  },
  methods: {
    dismiss(ev) {
      if (ev.target == this.$refs.overlay || this.$refs.tip.contains(ev.target)) {
        this.show = false;
      }
    }
  }
}
</script>

<style>
.tip-wrapper {
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  pointer-events: none;
  z-index: 101;
}
.tip-wrapper.overlay {
  background: rgba(0,0,0,0.8);
  pointer-events: auto;
}
.tip {
  background: #222;
  color: #fff;
  padding: 0.5em;
  border-radius: 0.3em;
  margin: 1em auto;
  border: 1px solid #707070;
  border-right: 2px solid #000;
  border-bottom: 2px solid #000;
  display: flex;
  pointer-events: auto;
  box-shadow: 0 1px 2px rgb(0 0 0 / 70%);
  font-family: 'Inter', sans-serif;
  max-width: 700px;
  box-shadow: 0 0 9px rgba(0,0,0,0.8);
}
.tip--icon {
  min-width: 36px;
  max-width: 36px;
  position: relative;
}
.tip--subicon {
  position: absolute;
  width: 16px;
  right: -4px;
}
.tip--supicon {
  position: absolute;
  width: 16px;
  right: -4px;
  top: -4px;
}
.tip--body {
  padding: 0 0.5em;
  align-self: center;
  font-size: 0.9em;
}
.tip--body strong {
  /* color: #ffee5d; */
  color: #43cc70;
}
.tip--body img {
  width: 16px;
  vertical-align: middle;
}
.tip--card {
  pointer-events: none;
}
.tip--card .card {
  pointer-events: auto;
}
.tip--card .card.process {
  margin: 0 auto !important;
  padding-right: 0 !important;
}

*[data-tip]:hover {
  opacity: 0.8;
  cursor: pointer;
}
</style>
