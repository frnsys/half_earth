<template>
<MiniCard>
  <template v-slot:body>
    <header>
      <img :src="icons.ally" v-if="relationshipName == 'Ally'">
    </header>
    <div class="mini-character">
      <img
        :src="`/assets/characters/${npc.name}.png`"
        onerror="this.src='/assets/placeholders/character.png';" />
    </div>
    <footer class="mini-relationship">
      <template v-for="i in consts.maxRelationship" >
        <img :src="icons.relationship" v-if="i <= npc.relationship" />
        <img :src="icons.relationship_empty" v-else />
      </template>
    </footer>

  </template>
  <template v-slot:expanded>
    <NPCCard :npc="npc" />
  </template>
</MiniCard>
</template>

<script>
import NPCCard from '../NPCCard.vue';
import MiniCard from './MiniCard.vue';
import display from '/src/display/display';

export default {
  props: ['npc'],
  components: {
    MiniCard,
    NPCCard,
  },
  computed: {
    relationshipName() {
      return display.relationshipName(this.npc.relationship);
    }
  }
}
</script>

<style>
.mini-character {
  width: 48px;
  margin: 0.5em auto;
}
.mini-relationship img {
  width: 10px !important;
}
</style>
