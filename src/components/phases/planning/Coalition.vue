<template>
<div class="planning--page">
  <div class="minicard-grid">
    <div class="minicard-grid-item" v-for="npc in npcs">
      <MiniNPC :npc="npc" />
      <div class="minicard-grid-item-label">{{npc.name}}</div>
    </div>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import MiniNPC from 'components/cards/MiniNPC.vue';

export default {
  components: {
    MiniNPC
  },
  data() {
    return {
      npcs: state.gameState.npcs.filter((npc) => !npc.locked),
    }
  },
}
</script>
