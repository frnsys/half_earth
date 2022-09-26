<template>
<Card class="npc" background="#724680">
  <template v-slot:header>
    <div>{{t('Parliament')}}</div>
    <div v-tip="{
      icon: 'relationship',
      text: t(`Your relationship with {name}. Increase it by implementing projects they like. At 5 hearts or more they will join your coalition.`, {name: t(name)}),
    }">
      <template v-for="i in consts.maxRelationship" >
        <img :src="icons.relationship" v-if="i <= npc.relationship" />
        <img :src="icons.relationship_empty" v-else />
      </template>
    </div>
  </template>
  <template v-slot:figure>
    <img
      :src="`/assets/characters/${npc.name}.webp`"
      @error="fallbackPortrait" />
  </template>
  <template v-slot:name>
    <div class="npc-tag"><img :src="icons[relationshipName.toLowerCase()]">{{t(relationshipName)}}</div>
    {{t(name)}}
  </template>
  <template v-slot:body>
    <p v-if="isAlly" class="npc-effect active" v-html="effectsHtml"></p>
    <p v-else class="npc-effect inactive" v-tip="{text: t(`Improve your relationship with {name} to activate this ability.`, {name: t(name)}), icon: 'relationship'}" v-html="effectsHtml"></p>
  </template>
  <template v-slot:top-back>
    <img
      :src="`/assets/characters/${npc.name}.webp`"
      @error="fallbackPortrait" />
    <p class="card-desc npc-desc">{{t(description)}}</p>
  </template>
  <template v-slot:bot-back>
    <div class="likes-dislikes">
      <div>
        <h3>{{t('Likes')}}</h3>
        <p>{{t(likes)}}</p>
      </div>
      <div>
        <h3>{{t('Dislikes')}}</h3>
        <p>{{t(dislikes)}}</p>
      </div>
    </div>
  </template>
</Card>

</template>

<script>
import t from '/src/i18n';
import state from '/src/state';
import Card from './Card.vue';
import display from '/src/display/display';
import NPCS from '/assets/content/npcs.json';
import icons from '/src/components/icons';

export default {
  props: ['npc'],
  components: {
    Card,
  },
  data() {
    return {
      state,
      ...NPCS[this.npc.id],
    };
  },
  computed: {
    isAlly() {
      return this.relationshipName == 'Ally';
    },
    effectsHtml() {
      return `${this.isAlly ? '' : `<img src="${icons.locks}" />`}${display.fillIcons(t(this.effects))}`;
    },
    relationshipName() {
      return display.relationshipName(this.npc.relationship);
    }
  },
  methods:{
    fallbackPortrait(e){
      e.target.src ='/assets/characters/' + this.npc.name + '.png'
    },
  }
}
</script>

<style>
.npc figure {
  text-align: center;
  position: relative;
  display: flex;
  flex-direction: column;
  justify-content: center;
  background-color: rgba(0,0,0,0.1);
}
.npc figure img {
  width: 140px;
  image-rendering: auto;
  margin: 0 auto;
}
.npc p.inactive {
  opacity: 0.5;
  text-align: center;
}
.npc p.active {
  opacity: 1;
  text-align: center;
}

.npc-tag {
  color: #000;
  background: #fff;
  border-radius: 1rem;
  border: 1px solid #000;
  text-align: center;
  font-family: 'W95FA', monospace;
  line-height: 1.2;
  display: flex;
  font-size: 0.9rem;
  padding: 0.2rem 0.25rem 0;

  position: absolute;
  top: -2.1rem;
  left: 50%;
  transform: translate(-50%, 50%);
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
  margin: 2em 0 0;
  image-rendering: auto;

}
</style>
