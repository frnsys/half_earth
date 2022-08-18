<template>
<div class="plan-change-select planning--page">
  <HelpTip :text="scanTip" x="50%" y="150px" :center="true" />
  <HelpTip :text="scrollTip" x="50%" y="250px" :center="true" />

  <div class="planning--page-tabs">
   <div class="planning-sub-tab" @click="type = 'Research'" :class="{selected: type == 'Research'}">
      <img :src="icons.research" />
      <div>{{t('Research')}}</div>
    </div>
   <div class="planning-sub-tab" @click="type = 'Initiative'" :class="{selected: type == 'Initiative'}">
      <img :src="icons.initiative" />
      <div>{{t('Infrastructure')}}</div>
    </div>
   <div class="planning-sub-tab" @click="type = 'Policy'" :class="{selected: type == 'Policy'}">
      <img :src="icons.policy" />
      <div>{{t('Policies')}}</div>
    </div>
    <div @click="$emit('close')" :class="{disabled: backDisabled, highlight: backHighlighted}">{{t('Back')}}</div>
  </div>

  <AddScanner ref="addScanner" :project="project" v-if="project" />
  <RemoveScanner ref="removeScanner" :project="project" v-if="project" />

  <Cards @focused="onFocus" @scrollStart="onScrollStarted" @scrollEnd="onScrollEnd" :disabled="!allowScroll">
    <Draggable
      @drag="onDragStarted"
      @dragStop="onDragStop"
      v-for="i in projectOrder"
      :yBounds="yBounds"
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
    <Points :kind="type" />
  </footer>
</div>
</template>

<script>
import t from '/src/i18n';
import debug from '/src/debug';
import state from '/src/state';
import HelpTip from 'components/Help.vue';
import ProjectCard from 'components/cards/ProjectCard.vue';
import tutorial from '/src/tutorial';

import CardsMixin from 'components/phases/CardsMixin';
import Points from 'components/scanner/project/Points.vue';
import AddScanner from 'components/scanner/project/AddScanner.vue';
import RemoveScanner from 'components/scanner/project/RemoveScanner.vue';

import PROJECT_LOCKERS from '/assets/content/project_lockers.json';

const scanTip = t('↑ Swipe this card up and hold to add it to your plan ↑');
const scrollTip = t('⟵ Swipe sideways to see other projects ⟶ ');

export default {
  mixins: [CardsMixin],
  components: {
    Points,
    ProjectCard,
    HelpTip,
    AddScanner,
    RemoveScanner,
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
      this.updateFocused();

      // Emit for events
      let page = type;
      if (type == 'Initiative') {
        page = 'Initiatives';
      } else if (type == 'Policy') {
        page = 'Policies';
      }
      this.$emit('page', page);
    }
  },
  computed: {
    scanTip() {
      return scanTip;
    },
    scrollTip() {
      return scrollTip;
    },
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
          if (this.projects.length === 0) {
            return null;
          } else {
            return this.projects[0];
          }
        } else {
          return proj;
        }
      } else {
        // Default for loading
        return null;
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

            // Filter out projects that are mutually exclusive
            // with active projects
            && (PROJECT_LOCKERS[p.id] === undefined ||
              (state.gameState.projects[PROJECT_LOCKERS[p.id]].status !== 'Building' &&
              state.gameState.projects[PROJECT_LOCKERS[p.id]].status !== 'Active' &&
              state.gameState.projects[PROJECT_LOCKERS[p.id]].status !== 'Finished'))

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
    onFocus(idx) {
      this.onFocused(idx);
      if (this.project) {
        if (!state.viewed.includes(this.project.ref_id)) {
          state.viewed.push(this.project.ref_id);
        }
      }
    },
    onScrollStarted() {
      state.help[scrollTip] = true;
      this.onScrollStart();
    },
    onDragStarted(rect) {
      state.help[scanTip] = true;
      this.onDrag(rect);
    },
    yBounds() {
      return [
        this.$refs.addScanner.botY - 10,
        this.$refs.removeScanner.topY + 10 - this.cardHeight,
      ];
    },
    items(idx) {
      return this.projectOrder[idx];
    },
  }
}
</script>