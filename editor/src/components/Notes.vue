<template>
  <div class="notes">
    <label>
      <span>
        <span class="notes-icon question" v-if="hasQuestion">?</span>
        <span class="notes-icon" v-if="notes && notes.length > 0">!</span> Notes, References, &amp; Discussion</span>
    </label>
    <textarea v-model="notes"
      placeholder="Write any notes or discussion for others"
      @blur="$emit('blur', notes)" :class="{question: hasQuestion}"/>
  </div>
</template>

<script>
export default {
  props: ['notes'],
  computed: {
    hasQuestion() {
      return this.notes && this.notes.includes('? ');
    }
  }
};
</script>

<style>
.notes {
  margin-top: 0.5em;
}
.notes-icon {
	color: #fff;
  background: #5942ff;
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
