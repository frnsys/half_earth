<template>
<div class="card-focus-area wrapper">
  <div class="inner">
    <small class="helper top">▲</small>
    <small class="helper bottom">▼</small>
  </div>
</div>
</template>

<script>
export default {
  name:'card-focus-area'
}
</script>

<style scoped>
.card-focus-area.wrapper{
  position: fixed;
  z-index: -99;
  display: flex;
  top:0;
  bottom: 0;
  left:0;
  right: 0;
  align-items: center;
  justify-content: center;
  pointer-events: none;
}
.inner{
  background-color: rgba(0,0,0,0.1);
  /* background-image: url('/assets/backgrounds/screen-door.png'); */
  height: 520px;
  width: 350px;
  pointer-events: none;
  border-radius: 1em;

  box-shadow: inset 1px 1px 0px rgb(0 0 0 / 50%);
  border-right: 1px solid rgba(255,255,255,0.5);
  border-bottom: 1px solid rgba(255,255,255,0.5);

  position: relative;
}

.helper{
  text-transform: uppercase;
  letter-spacing: 0.01em;
  font-family: 'Inter', sans-serif;
  font-size: 1em;
  text-align: center;
  width: 100%;
  position: absolute;
  opacity: 0.5;
}

.helper.top{
  top:0.75em;
}

.helper.bottom{
  bottom:0.75em;
}

</style>