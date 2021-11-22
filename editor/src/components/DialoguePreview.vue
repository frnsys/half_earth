<template>
<div class="overlay" @click="tryClose" ref="overlay">
  <Dialogue :dialogue="dialogue" @done="$emit('close')"/>
</div>
</template>

<script>
import Dialogue from './Dialogue.vue';

export default {
  props: ['dialogue'],
  components: {
    Dialogue,
  },
  methods: {
    tryClose(ev) {
      if (ev.target == this.$refs.overlay) {
        this.$emit('close');
      }
    },
  }
};
</script>

<style>
.overlay {
  position: fixed;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  z-index: 100;
  background: rgba(0,0,0,0.9);
  display: flex;
  align-items: center;
}
.image-editor {
  background: #18181A;
  border-radius: 0.2em;
  max-width: 800px;
  margin: 0 auto;
  border: 1px solid #fff;
}
#palette {
  display: none;
}
.palette {
  padding: 1em;
  display: flex;
  flex-wrap: wrap;
}
.palette input {
  width: 48px;
	padding: 0;
  border: 1px solid #D0D0D7;
  vertical-align: middle;
  border-radius: 2px 0 0 2px;
}
.palette button {
  margin-right: 0.2em;
}
.color-select {
  display: flex;
  margin-right: 0.2em;
}
.color-delete {
	background: #D0D0D7;
	border-radius: 0 0.2em 0.2em 0;
	padding: 0.1em;
	font-size: 0.8em;
	line-height: 1.5;
}
.image-previews {
  display: flex;
}
.image-previews > div {
  width: 50%;
  min-height: 240px;
  position: relative;
}
.image-previews .image-preview {
  height: 100%;
}
.dither-button {
	width: 100%;
	border-radius: 0 0 0.2em 0.2em;
	border: 1px solid #18181A;
  padding: 0.5em;
}
.color-delete {
  cursor: pointer;
}
.saved-palette {
  cursor: pointer;
  padding: 0 2px;
  display: inline-block;
  margin-right: 0.25em;
}
.saved-palette:hover {
  background: #E5CF2B;
}
.swatch {
  width: 12px;
  height: 12px;
  display: inline-block;
}
.image-preview-label {
  position: absolute;
  top: 0.5em;
  left: 50%;
  transform: translate(-50%, 0);
  background: rgba(0,0,0,0.7);
  color: #fff;
  padding: 0 0.2em;
  border-radius: 0.2em;
  font-size: 0.7em;
}
.image-preview-save,
.image-preview-upload {
  font-size: 0.7em;
  position: absolute;
  bottom: 0.5em;
  right: 0.5em;
  padding: 0 0.1em;
}
.image-preview-upload {
  right: auto;
  left: 0.5em;
}
.image-upload-input {
  display: none;
}
</style>

