<template>
<Card class="npc">
  <template v-slot:header>
    <div>{{name}}</div>
    <div v-tip="{text: `Your relationship with ${name}.`, icon: 'relationship'}">
      <template v-for="i in consts.maxRelationship" >
        <img :src="icons.relationship" v-if="i <= relationship" />
        <img :src="icons.relationship_empty" v-else />
      </template>
    </div>
  </template>
  <template v-slot:figure>
    <img
      :src="`/assets/characters/${npc.name}.png`"
      onerror="this.src='/assets/placeholders/character.png';" />
    <div class="npc-relationship">{{relationshipName}}</div>
  </template>
  <template v-slot:body>
    <p v-if="relationshipName == 'Ally'" class="active" v-html="html"></p>
    <p v-else v-tip="{text: `Improve your relationship with ${name} to activate this ability.`, icon: 'relationship'}" v-html="html"></p>
  </template>
  <template v-slot:back>
  </template>
  <template v-slot:footer>
    <div>GOSPLANT</div>
  </template>
</Card>

</template>

<script>
import Card from './Card.vue';
import display from 'lib/display';
import NPCS from '/assets/content/npcs.json';

export default {
  props: ['npc'],
  components: {
    Card,
  },
  data() {
    return {
      ...NPCS[this.npc.id],
    };
  },
  computed: {
    html() {
      return display.fillIcons(this.description);
    },
    relationshipName() {
      return display.relationshipName(this.relationship);
    }
  }
}
</script>

<style>
.npc figure {
  text-align: center;
  background: #fcf5e0;
  border-radius: 0.3em;
  position: relative;
  padding: 2em 1em;
}
.npc figure img {
  width: 120px;
}
.npc p {
  opacity: 0.25;
}
.npc p.active {
  opacity: 1;
}
.npc-relationship {
  color: #fff;
  background: #222;
  border: 1px solid #fff;
  font-family: 'Andada Pro';
  text-transform: uppercase;
  font-size: 0.8em;
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translate(-50%, 50%);
  padding: 0.1em 0.2em;
}
</style>
