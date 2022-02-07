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
import consts from '/src/consts';
import NPCCard from '../NPCCard.vue';
import MiniCard from './MiniCard.vue';
import display from '/src/display/display';
import NPCS from '/assets/content/npcs.json';

const totalSeats = consts.parliamentSeats.reduce((acc,s) => acc + s, 0);

export default {
  props: ['npc'],
  components: {
    MiniCard,
    NPCCard,
  },
  computed: {
    relationshipName() {
      return display.relationshipName(this.npc.relationship);
    },
    factionSeats() {
      return Math.floor(this.npc.seats * totalSeats);
    },
    factionColor(npc) {
      return NPCS[this.npc.id].color;
    }
  }
}
</script>

<style>
.mini-character {
  width: 48px;
  position: absolute;
  left: 50%;
  top: 0;
  transform: translate(-50%, -25%);
}

.mini-npc-name {
  font-size: 0.85em;
}

.mini-npc-seats {
  text-align: center;
  padding-bottom: 0.5em;
}
.mini-npc-seats > div {
  height: 8px;
  width: 8px;
  display: inline-block;
  margin: 0 1px;
}

.mini-npc-tag {
  position: absolute;
  left: 50%;
  transform: translate(-50%, 50%);
  bottom: 0;
  width: 60px;
}
</style>
