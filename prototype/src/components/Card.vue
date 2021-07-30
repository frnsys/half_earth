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
  width: 160px;
  height: 200px;
  cursor: pointer;
  position: relative;
  display: flex;
  flex-direction: column;
  justify-content: space-around;
}
.card:hover {
  background: #f0f0f0;
}
.card.selected {
  border: 1px solid #000;
  box-shadow: 3px 3px 0 rgba(0,0,0,0.5);
}
.card.selected::before {
  content: '✔️';
  position: absolute;
  top: -0.5em;
  left: -0.5em;
  background: #fff;
}

.details {
  margin-top: 1em;
  text-align: center;
}

.flip {
  position: absolute;
  right: 0.5em;
  top: 0;
  opacity: 0;
  z-index: 1;
}
.card:hover .flip {
  opacity: 0.5;
}
.card:hover .flip:hover {
  opacity: 1;
}

.meta {
  left: 0;
  right: 0;
  position: absolute;
}
.meta-top {
  top: -0.7em;
}
.meta-bot {
  bottom: -0.7em;
}
.meta > div {
	background: #fff;
	margin: 0 auto;
	display: inline-block;
	border: 1px dashed black;
	padding: 0.05em 0.1em;
}
</style>
