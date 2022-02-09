<template>
<Card class="npc" background="#724680">
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
    <p v-if="relationshipName == 'Ally'" class="active" v-html="effectsHtml"></p>
    <p v-else class="inactive" v-tip="{text: `Improve your relationship with ${name} to activate this ability.`, icon: 'relationship'}" v-html="effectsHtml"></p>
  </template>
  <template v-slot:top-back>
    <img
      :src="`/assets/characters/${npc.name}.png`"
      onerror="this.src='/assets/placeholders/character.png';" />
    <p class="card-desc npc-desc">{{description}}</p>
  </template>
  <template v-slot:bot-back>
    <div class="likes-dislikes">
      <div>
        <h3>Likes</h3>
        <p>{{likes}}</p>
      </div>
      <div>
        <h3>Dislikes</h3>
        <p>{{dislikes}}</p>
      </div>
    </div>
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
    effectsHtml() {
      return display.fillIcons(this.effects);
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
.npc p.inactive {
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

.likes-dislikes {
  display: flex;
  width: 100%;
}
.likes-dislikes > div {
  flex: 1;
  padding: 0.5em;
  border-right: 1px solid rgba(255,255,255,0.5);
  border-bottom: 1px solid rgba(255,255,255,0.5);
  border-top: 1px solid rgba(0,0,0,0.4);
  border-left: 1px solid rgba(0,0,0,0.4);
  border-radius: 0.5em;
  margin: 0 0.5em;
}

.npc-desc {
  font-style: italic;
  padding: 0 1em;
  font-size: 0.75em;
  z-index: 1;
}

.likes-dislikes h3 {
  font-family: 'Inter', sans-serif;
  text-transform: uppercase;
  font-size: 0.55em;
  text-align: center;
  margin-top: 0.5em;
}
.likes-dislikes p {
  font-family: 'W95FA';
  font-size: 0.75em;
  text-transform: uppercase;
  text-align: center;
}

.card-top-back > img {
  height: 72px;
  margin: 4em 0 0;
}
</style>
