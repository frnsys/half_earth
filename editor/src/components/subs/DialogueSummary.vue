<template>
<div class="dialogue-summary" v-if="dialogue">
  <div v-if="!dialogue.lines || dialogue.lines.length === 0">MISSING DIALOGUE</div>
  <div v-else v-for="line in dialogue.lines">[{{line.speaker || "MISSING SPEAKER"}}]: "{{line.text || "MISSING TEXT"}}"</div>
  <ChoicesSummary :choices="dialogue.choices" />
</div>
</template>

<script>
import ChoicesSummary from './ChoicesSummary.vue';

export default {
  props: ['dialogue'],
  components: {
    ChoicesSummary
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
