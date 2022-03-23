<template>
<div class="plan-change-select planning--page">
  <HelpTip text="↑ Swipe this card up and hold to add it to your plan ↑" x="50%" y="150px" :center="true" />
  <HelpTip text="⟵ Scroll sideways to see other projects ⟶ " x="50%" y="250px" :center="true" />

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
    <div @click="$emit('close')" :class="{disabled: backDisabled, highlight: backHighlighted}">Back</div>
  </div>
  <!-- <div class="card-scan-target" ref="target"></div> -->

  <div class="scanbar-wrapper"  ref="target">
    <div class="mini-scanbar">
        <div class="scanbar-base">
          <div class="scan-progress-bar" ref="scanProgress"></div>
        </div>
        <div class="scanbar-led scanbar-led-ok"></div>
        <div class="scanbar-led scanbar-led-bad"></div>
        <div class="card-scan-target"></div>
    </div>
  </div>

  <div class="card-withdraw-target" ref="withdrawTarget">
    {{ refundable ? 'Undo' : (canDowngrade ? 'Downgrade' : 'Withdraw') }}
    <div class="withdraw-bar" ref="withdrawProgress"></div>
  </div>

  <Cards @focused="onFocused" @scrollStart="onScrollStart" @scrollEnd="onScrollEnd" :disabled="!allowScroll">
    <Draggable
      ref="draggables"
      @drag="onDrag"
      @tryScroll="tryScroll"
      @dragStop="onDragStop"
      v-for="i in projectOrder"
      :minY="yMin"
      :maxY="yMax"
      :draggable="allowSwipe && focused == i"
      :id="projects[i].id"
      :key="projects[i].id">
      <ProjectCard
        :project="projects[i]"
        @change="$emit('change')" />
    </Draggable>
  </Cards>

  <CardFocusArea />

  <footer>
    <div class="pips">
      <div class="scan-progress" ></div>
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
import debug from '/src/debug';
import state from '/src/state';
import HelpTip from 'components/Help.vue';
import ProjectCard from 'components/cards/ProjectCard.vue';
import CardsMixin from 'components/phases/CardsMixin';
import ScannerMixin from 'components/phases/ScannerMixin';
import tutorial from '/src/tutorial';

export default {
  mixins: [ScannerMixin('Project'), CardsMixin],
  components: {
    ProjectCard,
    HelpTip,
  },
  data() {
    return {
      state,
      type: 'Research',
    };
  },
  mounted() {
    this.$emit('page', this.type);
  },
  watch: {
    type(type) {
      // Figure out what the focused card is
      this.updateFocused(() => {
        // Emit for events
        let page = type;
        if (type == 'Initiative') {
          page = 'Initiatives';
        } else if (type == 'Policy') {
          page = 'Policies';
        }
        this.$emit('page', page);
      });
    }
  },
  computed: {
    backDisabled() {
      return state.tutorial < tutorial.PROJECTS_BACK;
    },
    backHighlighted() {
      return state.tutorial == tutorial.PROJECTS_BACK;
    },
    project() {
      if (this.focused !== null) {
        let proj =  this.projects[this.focused];
        if (proj === undefined) {
          return this.projects[0];
        } else {
          return proj;
        }
      } else {
        // Default for loading
        return state.gameState.projects[0];
      }
    },
    projectOrder() {
      let idxs = this.projects.map((p, i) => i);
      idxs.sort((a, b) => this.projects[a].name.toLowerCase().localeCompare(this.projects[b].name.toLowerCase()))
      return idxs;
    },
    projects() {
      return state.gameState.projects
        .filter((p) => {
          return p.kind == this.type
            && (!p.locked || debug.showAllProjects)
            // Filter out finished projects
            && p.status !== 'Finished'

            // Filter out finished policies
            // but only ones added before
            // this planning session
            && (p.status !== 'Active' || p.id in state.planChanges)
        });
    },
  },
  methods: {
    items(idx) {
      return this.projectOrder[idx];
    },
  }
}
</script>

<style scoped>

.scanbar-wrapper{
  width: 100%;
  position: absolute;
  height:60px;
  top:-20px;
  z-index: 1;
}
.mini-scanbar {
  height: 60px;
  position: relative;
  /* top: 0; */
  margin:0 auto;
}


</style>