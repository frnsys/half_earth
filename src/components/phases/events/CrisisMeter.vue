<template>
<div class="crisis-meter">
  <div class="crisis-meter-label">Crisis</div>
  <div class="crisis-meter-bar">
    <div class="crisis-meter-bar-current" :style="{width: `${state.crisis.max}%`}">
      <div class="crisis-meter-bar-cover" :style="{width: `${(1 - state.crisis.points/state.crisis.max) * 100}%`}"></div>
    </div>
  </div>
  <div class="crisis-meter-intensity">2X</div>
</div>
</template>

<script>
import state from '/src/state';

export default {
  data() {
    return {
      state,
    }
  },
}
</script>

<style>
.crisis-meter {
  display: flex;
  position: absolute;
  bottom: 6px;
  left: 50%;
  transform: translate(-50%);
}
.crisis-meter-label {
  color: #555555;
  text-transform: uppercase;
  font-weight: bold;
  margin-right: 0.5em;
  line-height: 1;
}
.crisis-meter-bar {
  background: rgba(0,0,0,0.15);
  height: 14px;
  width: 200px;
  position: relative;
  overflow: hidden;
  border-radius: 0.2em;
}
.crisis-meter-bar-current {
  height: 100%;
  position: relative;
  background: -moz-linear-gradient(left,  #43cc70 0%, #e2af04 48%, #f22004 100%); /* FF3.6-15 */
  background: -webkit-linear-gradient(left,  #43cc70 0%,#e2af04 48%,#f22004 100%); /* Chrome10-25,Safari5.1-6 */
  background: linear-gradient(to right,  #43cc70 0%,#e2af04 48%,#f22004 100%); /* W3C, IE10+, FF16+, Chrome26+, Opera12+, Safari7+ */
}
.crisis-meter-bar-cover {
  height: 100%;
  width: 10%;
  position: absolute;
  right: -1px;
  background: #878787;
}
.crisis-meter-intensity {
  font-weight: bold;
  line-height: 1;
  margin-left: 0.25em;
  color: #EB3941;
}
</style>
