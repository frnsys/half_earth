<template>
<div class="card">
  <slot></slot>
</div>
</template>

<style>
.card {
  padding: 1em;
  border: 1px dashed #000;
  text-align: center;
  max-width: 280px;
  cursor: pointer;
}
.card:hover {
  background: #f0f0f0;
}
</style>
