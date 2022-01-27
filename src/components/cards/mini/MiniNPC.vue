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
    <div class="ally-tag" v-if="relationshipName == 'Ally'">
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

.ally-tag {
  color: #000;
  background: #fff;
  border-radius: 1em;
  border: 1px solid #000;
  text-align: center;
  font-family: 'VT323', monospace;
  position: absolute;
  padding: 0 0.25em;
  left: 50%;
  transform: translate(-50%, 50%);
  width: 60px;
  bottom: 0;
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
</style>
