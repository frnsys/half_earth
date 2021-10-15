<template>
  <Setting
      background="assets/settings/bgs/redwood_forest.webp"
      audio="assets/settings/audio/463903__burghrecords__birds-in-spring-scotland.webm" />
  <Dialogue :dialogue="dialogue" />
</template>

<script>
import Setting from './Setting.vue';
import Dialogue from '../Dialogue.vue';

export default {
  props: ['dialogue'],
  components: {
    Setting,
    Dialogue,
  }
}
</script>
