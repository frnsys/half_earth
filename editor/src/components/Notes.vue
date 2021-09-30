<template>
  <div class="notes">
    <label @click="() => expandNotes = !expandNotes">
      <span>
        <span class="notes-icon question" v-if="hasQuestion">?</span>
        <span class="notes-icon" v-if="notes && notes.length > 0">!</span> Notes, References, &amp; Discussion</span>
      <div class="notes--toggle">Toggle</div>
    </label>
    <textarea v-if="expandNotes" v-model="notes"
      placeholder="Write any notes or discussion for others"
      @blur="$emit('blur', notes)" :class="{question: hasQuestion}"/>
  </div>
</template>

<script>
export default {
  props: ['notes'],
  data() {
    return {
      expandNotes: false,
    };
  },
  computed: {
    hasQuestion() {
      return this.notes && this.notes.includes('?');
    }
  }
};
</script>

<style>
.notes {
  margin-top: 0.5em;
  padding: 0 0.25em 0.25em 0.25em;
  background: #f0f0f0;
  border: 1px solid #ccc;
}
.notes label {
  cursor: pointer;
  user-select: none;
}
.notes-icon {
	color: #fff;
	background: #484848;
	border-radius: 10em;
	padding: 0 0.45em;
}
.notes-icon.question {
  color: #000;
  background: #EFCF40;
  margin-right: 0.25em;
  padding: 0 0.3em;
}
</style>
