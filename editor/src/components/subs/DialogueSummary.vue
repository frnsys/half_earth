<template>
<div class="dialogue-summary">
  <div>[{{localData.speaker || "MISSING SPEAKER"}}]: "{{localData.text || "MISSING TEXT"}}"</div>
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
      let invalid = dialogue.text === undefined || dialogue.text === '';
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
