<template>
<div class="plan-change-select planning--page" :class="{scrolling}">
  <HelpTip text="↑ Swipe this card up and hold to add it to your plan ↑" x="50%" y="150px" :center="true" />
  <HelpTip text="⟵ Swipe sideways to see other projects ⟶ " x="50%" y="250px" :center="true" />

  <div class="planning--page-tabs">
   <div class="planning-sub-tab" @click="type = 'Research'" :class="{selected: type == 'Research'}">
      <img :src="icons.research" />
      <div>Research</div>
    </div>
   <div class="planning-sub-tab" @click="type = 'Initiative'" :class="{selected: type == 'Initiative'}">
      <img :src="icons.initiative" />
      <div>Infrastructure</div>
    </div>
   <div class="planning-sub-tab" @click="type = 'Policy'" :class="{selected: type == 'Policy'}">
      <img :src="icons.policy" />
      <div>Policies</div>
    </div>
    <div @click="$emit('close')">Back</div>
  </div>
  <div class="card-scan-target" ref="target"></div>

  <div class="card-withdraw-target" ref="withdrawTarget">
    Withdraw
    <div class="withdraw-bar" ref="withdrawProgress"></div>
  </div>

  <Cards @dragStart="onDragStart" @dragEnd="onDragEnd" :enabled="scrollable">
    <Draggable @drag="onDragVertical"
      @dragStop="onDragVerticalStop"
      v-for="i in projectOrder"
      :minY="yMin"
      :maxY="yMax"
      :draggable="focusedProject == projects[i].id"
      :id="projects[i].id"
      :key="projects[i].id">
      <ProjectCard
        :project="projects[i]"
        @change="$emit('change')" />
    </Draggable>
  </Cards>
  <footer>
    <div class="pips">
      <div class="scan-progress" ref="scanProgress"></div>
      <template v-if="type == 'Policy'">
        {{availablePoints}}<img class="pip" :src="icons.political_capital">
      </template>
      <template v-else>
        <template v-if="availablePoints > 0">
          {{availablePoints}}<img class="pip" :src="icons[icon]">
        </template>
        <template v-else>
          {{nextPointCost}}<img class="pip" :src="icons.political_capital"> ⮕ <img class="pip" :src="icons[icon]">
        </template>
      </template>
    </div>
  </footer>
</div>
</template>

<script>
import state from '/src/state';
import HelpTip from 'components/Help.vue';
import Cards from 'components/cards/Cards.vue';
import ProjectCard from 'components/cards/ProjectCard.vue';
import ScannerMixin from 'components/phases/ScannerMixin';

export default {
  mixins: [ScannerMixin],
  components: {
    Cards,
    ProjectCard,
    HelpTip,
  },
  data() {
    return {
      state,
      scrolling: false,
      scrollable: true,
      focusedProject: 0,
      type: 'Research',
      projectOrder: [],
    };
  },
  watch: {
    type(type) {
      this.scrolling = false;
      this.scrollable = true;
      // Kind of hacky, but
      // figure out what the focused card is
      this.$nextTick(() => {
        for (let el of document.querySelectorAll('.draggable')) {
          let rect = el.getBoundingClientRect();
          if (rect.x > 0) {
            this.focusedProject = el.id;
            break;
          }
        }
      });
    }
  },
  computed: {
    project() {
      if (this.focusedProject !== null) {
        return state.gameState.projects[this.focusedProject];
      }
    },
    projectOrder() {
      let projects = state.gameState.projects
        .filter((p) => p.kind == this.type && !p.locked);

      let idxs = projects.map((p, i) => i);
      idxs.sort((a, b) => projects[a].name.toLowerCase().localeCompare(projects[b].name.toLowerCase()))
      return idxs;
    },
    projects() {
      return state.gameState.projects.filter((p) => p.kind == this.type && !p.locked);
    },
  },
  methods: {
    onDragStart() {
      this.scrolling = true;
    },
    onDragEnd(card) {
      if (card) {
        this.focusedProject = parseInt(card.child.id);
      }
      this.scrolling = false;
    },
    onDragVertical(component) {
      this.scrollable = false;
      this.checkDrag(component);
    },
    onDragVerticalStop() {
      this.scrollable = true;
      this.stopDrag();
    },
  }
}
</script>

<style>
.plan-change-select.scrolling .card {
  pointer-events: none;
}
</style>
