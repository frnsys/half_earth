<template>
<div class="tip-wrapper" v-if="show" :class="{overlay: card}">
  <div class="tip">
    <div class="tip--icon" v-if="icon">
      <img :src="assets.icons[icon]">
    </div>
    <div class="tip--body" v-if="text" v-html="text"></div>
  </div>
  <ProcessCard v-if="card && card.type == 'Process'" :process="card.data" />
  <ProjectCard v-if="card && card.type == 'Project'" :project="card.data" />
</div>
</template>

<script>
import ProcessCard from 'components/cards/ProcessCard.vue';
import ProjectCard from 'components/cards/ProjectCard.vue';

export default {
  components: {
    ProcessCard,
    ProjectCard
  },
  data() {
    return {
      show: false,
      icon: null,
      text: null,
      card: null
    }
  },
  created() {
    console.log('Created');
    document.body.addEventListener('click', () => {
      this.show = false;
    });

    // Probably very hacky
    window.tip = this;
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
}
.tip {
  background: #222;
  color: #fff;
  padding: 0.5em;
  border-radius: 0.3em;
  margin: 1em;
  border: 1px solid #7B7B7B;
  display: flex;
  align-items: center;
  pointer-events: auto;
}
.tip--icon {
  min-width: 36px;
  max-width: 36px;
}
.tip--body {
  padding: 0 0.5em;
}
.tip--body img {
  width: 16px;
  vertical-align: middle;
}
</style>
