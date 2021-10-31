<template>
<div class="planning--page">
  <header>
    <img class="back" @click="$emit('close')" src="/assets/icons/back.svg">
  </header>
  <div class="planning--coalition">
    <div class="npc" v-for="npc in npcs">
      <div class="npc--portrait">
        <img
          :src="`/assets/characters/${npc.name}.png`"
          onerror="this.src='/assets/placeholders/character.png';" />
        <div class="npc--relationship">{{relationshipName(npc.relationship)}}</div>
      </div>
      <div>
        <h3>{{npc.name}}</h3>
        <p v-html="npc.html"></p>
      </div>
    </div>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import NPCS from '/assets/content/npcs.json';

export default {
  data() {
    return {
      state
    }
  },
  computed: {
    npcs() {
      return state.gameState.npcs.map((npc) => {
        let data = NPCS[npc.id];
        let text = data.description;
        let icons = [...text.matchAll(/\[([a-z_]+)\]/g)];
        for (const match of icons) {
          text = text.replaceAll(match[0], `<img src="/assets/icons/pips/${match[1]}.png">`);
        }
        return {
          name: data.name,
          html: text,
          relationship: npc.relationship,
        }
      });
    }
  },
  methods: {
    relationshipName(val) {
      if (val >= 80) {
        return 'Ally';
      } else if (val <= -80) {
        return 'Nemesis';
      } else {
        return 'Neutral';
      }
    }
  }
}
</script>

<style>
.npc {
  display: flex;
  padding: 1em;
}

.npc--portrait {
  margin-right: 1em;
  display: flex;
  justify-content: space-around;
  width: 112px;
  text-align: center;
  flex-direction: column;
}
.npc--portrait img {
  width: 72px;
}

.npc p img {
  width: 22px;
  height: 22px;
  vertical-align: middle;
}

.npc h3 {
  font-weight: normal;
  font-family: 'Andada Pro';
}

.npc--relationship {
  font-size: 0.8em;
}
</style>
