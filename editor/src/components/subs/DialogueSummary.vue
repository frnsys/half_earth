<template>
<div class="dialogue-summary">
  <div v-if="!localData.lines || localData.lines.length === 0">MISSING DIALOGUE</div>
  <div v-else v-for="line in localData.lines">[{{line.speaker || "MISSING SPEAKER"}}]: "{{line.text || "MISSING TEXT"}}"</div>
  <ChoicesSummary :choices="localData.choices" />
</div>
</template>

<script>
import ChoicesSummary from './ChoicesSummary.vue';

export default {
  props: ['dialogue'],
  components: {
    ChoicesSummary
  },
  data() {
    return {
      localData: this.dialogue || {}
    };
  },
  methods: {
    flags(dialogue) {
      let invalid = dialogue.lines === undefined || dialogue.lines.length === 0;
      return {invalid};
    },
  }
}
</script>

<style>
.dialogue-summary {
	background: rgba(0,0,0,0.1);
	border: 1px solid #888;
	padding: 0.25em;
	border-radius: 0.2em;
}
</style>
