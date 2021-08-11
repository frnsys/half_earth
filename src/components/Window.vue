<template>
  <div class="window">
    <div class="window--title">{{title}}</div>
    <div class="window--body">
      <slot></slot>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    title: String,
  },
};
</script>


<style>
.window {
  background: #fff;
  margin: 2em auto;
  max-width: 640px;
  border-radius: 0.25em;
}
.window--title {
  text-align: center;
  border-radius: 0.25em 0.25em 0 0;
  font-size: 0.8em;
  background: #eee;
  line-height: 1;
  padding: 0.75em 0.5em 0.5em 0.5em;;
}
.window--body {
  padding: 0.5em 1em;
}
</style>
