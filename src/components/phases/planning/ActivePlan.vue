<template>
<div class="planning--page active-plan">
  <div class="planning--page-tabs">
    <div @click="$emit('close')">Back</div>
  </div>
  <div class="plan--changes">
    <div class="plan--change">
      <div class="plan--add-change minicard" @click="$emit('add')">
        <div>
          <img :src="icons.add">
          <div class="plan--action">Add</div>
        </div>
      </div>
    </div>
    <div class="plan--change" v-for="project in activeProjects">
      <MiniProject :project="project" />
      <div class="plan--change-name">{{ project.name }}</div>
    </div>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import MiniProject from 'components/cards/mini/MiniProject.vue';

export default {
  components: {
    MiniProject,
  },
  data() {
    return {
    };
  },
  computed: {
    activeProjects() {
      return state.gameState.projects.filter((p) => p.status == 'Active' || p.status == 'Finished' || p.status == 'Building');
    },
  },
  methods: {
  }
}
</script>

<style>
.active-plan {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  z-index: 2;
  background: url('/assets/backgrounds/plan.png');
  background-size: cover;
  background-repeat: no-repeat;
  background-position: center center;
  image-rendering: pixelated;
}

.planning--page.active-plan {
  padding-top: calc(4em + 10px);
}

.plan--change-name {
  color: rgba(0,0,0,0.8);
  text-transform: uppercase;
  font-size: 0.6em;
  letter-spacing: 0.01em;
  font-weight: bold;
  font-family: 'Inter', sans-serif;
 
  margin: 1em 0 0.5em;
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}

.active-plan .planning--page-tabs {
  max-width: 100px;
  margin: -0.75em auto 0;
  width: 100%;
  position:relative;
  top:-33px;
}

.active-plan .planning--page-tabs div{
  border-bottom-left-radius: 0.3em !important;
}

.active-plan .planning--page-tabs:hover {
  border-bottom-left-radius: 0.3em !important;
}

.active-plan .plan--changes {
  height: auto;
  max-width: 530px;
  width: 100%;
  column-gap: 1.25em;

  justify-content: left;
}

@media only screen and (min-width: 481px) {
  .active-plan .plan--changes {
    max-width: 610px;
  }
}
</style>
