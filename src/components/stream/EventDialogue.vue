<template>
<div class="event-dialogue">
  <Globe />
  <div class="event-dialogue--speech">
    <div class="event-dialogue--meta">
      <img src="/assets/characters/test.svg" />
      <div class="event-dialogue--type">Migrant Crisis</div>
    </div>
    <p>Urban gardening is the latest trend. With the plummeting use of personal vehicles, cities have reclaimed space for community uses.</p>
  </div>
  <div class="event-dialogue--responses">
    <div class="event-dialogue--response">
      <p>This is a thing you can do.</p>
      <div class="event-dialogue--response--effects">☹️⚡️</div>
    </div>
    <div class="event-dialogue--response">
      <p>This is another thing you can do.</p>
      <div class="event-dialogue--response--effects">☹️⚡️</div>
    </div>
    <div class="event-dialogue--response locked">
      🔒
    </div>
  </div>
</div>
</template>

<script>
import Globe from '../Globe.vue';

export default {
  components: {
    Globe
  }
}
</script>

<style>
.event-dialogue--speech {
  background: #fff;
  padding: 1em;
  border-radius: 1em;
  max-width: 360px;
  margin: 1em auto;
  box-shadow: 0px 4px 10px rgba(0,0,0,0.3);
}
.event-dialogue--speech p {
  margin-bottom: 0;
}
.event-dialogue--meta {
  display: flex;
  justify-content: space-between;
}
.event-dialogue--type {
  font-weight: 500;
}

.event-dialogue--responses {
  text-align: right;
}
.event-dialogue--response {
  background: #fff;
  padding: 1em;
  border-radius: 1em;
  max-width: 360px;
  margin: 0.5em 0;
  display: inline-block;
  text-align: left;
  box-shadow: 0px 4px 10px rgba(0,0,0,0.3);
  cursor: pointer;
}
.event-dialogue--response:hover {
  box-shadow: 0px 4px 10px rgba(0,0,0,0.5);
}
.event-dialogue--response.locked {
  opacity: 0.75;
  text-align: center;
  width: 240px;
}
.event-dialogue--response p {
  margin-top: 0;
}

.event-dialogue {
  position: relative;
  min-height: 100vh;
  padding: 0 1em;
}
.event-dialogue #globe {
  position: absolute;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  z-index: -1;
}
</style>
