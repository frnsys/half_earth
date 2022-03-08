<template>
<MiniCard>
  <template v-slot:body>
    <div class="mini-character">
      <img
        :src="`/assets/characters/${npc.name}.png`"
        onerror="this.src='/assets/placeholders/character.png';" />
    </div>
    <div class="mini-npc-name">{{npc.name}}</div>
    <div class="mini-npc-seats">
      <div :style="{background: factionColor}" v-for="i in factionSeats" />
    </div>
    <div class="mini-npc-tag npc-tag" v-if="relationshipName == 'Ally'">
      <img :src="icons.ally">Ally
    </div>
  </template>
  <template v-slot:expanded>
    <NPCCard :npc="npc" />
  </template>
</MiniCard>
</template>

<script>
import state from '/src/state';
import consts from '/src/consts';
import NPCCard from '../NPCCard.vue';
import MiniCard from './MiniCard.vue';
import display from '/src/display/display';
import NPCS from '/assets/content/npcs.json';
import textFit from 'textfit';

const totalSeats = consts.parliamentSeats.reduce((acc,s) => acc + s, 0);

export default {
  props: ['npc'],
  components: {
    MiniCard,
    NPCCard,
  },
  mounted() {
    this.fitText();
  },
  activated() {
    this.fitText();
  },
  computed: {
    relationshipName() {
      return display.relationshipName(this.npc.relationship);
    },
    factionSeats() {
      return Math.floor(this.npc.seats * totalSeats) + (state.extraSeats[this.npc.name] || 0);
    },
    factionColor(npc) {
      return NPCS[this.npc.id].color;
    }
  },
  methods: {
    fitText() {
      let npcName = this.$el.parentNode.querySelector('.mini-npc-name');
      if (npcName) {
        textFit(npcName, {
          alignHoriz: true,
          alignVert: true,
          multiLine: true,
          minFontSize: 11,
          maxFontSize: 13,
        });
      }
    }
  }
}
</script>

<style>
.mini-character {
  width: 62px;
  position: absolute;
  left: 50%;
  top: 0;
  transform: translate(-50%, -35%);
}

.mini-npc-name {
  font-size: 0.85em;
  margin-bottom: 0.2rem;
  height: 24px;
  width: 100%;
}

.mini-npc-seats {
  text-align: center;
  padding: 0 0.4em;
}
.mini-npc-seats > div {
  height: 8px;
  width: 8px;
  display: inline-block;
  margin: 0 1px;
}

.mini-npc-tag {
  top: auto;
  position: absolute;
  left: 50%;
  transform: translate(-50%, 50%);
  bottom: 0;
}
</style>
