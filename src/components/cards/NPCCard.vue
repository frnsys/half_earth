<template>
<Card class="npc" background="#724680" :noBack="true">
  <template v-slot:header>
    <div>Parliament</div>
    <div v-tip="{text: `Your relationship with ${name}.`, icon: 'relationship'}">
      <template v-for="i in consts.maxRelationship" >
        <img :src="icons.relationship" v-if="i <= npc.relationship" />
        <img :src="icons.relationship_empty" v-else />
      </template>
    </div>
  </template>
  <template v-slot:figure>
    <img
      :src="`/assets/characters/${npc.name}.png`"
      onerror="this.src='/assets/placeholders/character.png';" />
    <div class="card-tack-cb npc-tag"><img :src="icons[relationshipName.toLowerCase()]">{{relationshipName}}</div>
  </template>
  <template v-slot:name>
    {{name}}
  </template>
  <template v-slot:body>
    <p v-if="relationshipName == 'Ally'" class="active" v-html="html"></p>
    <p v-else v-tip="{text: `Improve your relationship with ${name} to activate this ability.`, icon: 'relationship'}" v-html="html"></p>
  </template>
  <template v-slot:back>
  </template>
</Card>

</template>

<script>
import Card from './Card.vue';
import display from '/src/display/display';
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
      return display.relationshipName(this.npc.relationship);
    }
  }
}
</script>

<style>
.npc figure {
  text-align: center;
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

.npc-tag {
  color: #000;
  background: #fff;
  border-radius: 1em;
  border: 1px solid #000;
  text-align: center;
  font-family: 'W95FA', monospace;
  line-height: 1.2;
  display: flex;
  font-size: 0.9em;
  padding: 0.2em 0.25em 0;
}
.npc-tag img {
  width: 12px !important;
  margin-right: 2px;
  margin-top: -4px;
}
</style>
