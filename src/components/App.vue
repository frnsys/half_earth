<template>
    <!-- <Setting
        background="assets/settings/bgs/redwood_forest.png"
        audio="assets/settings/audio/463903__burghrecords__birds-in-spring-scotland.mp3" /> -->
    <Implementation />
</template>

<script>
import Setting from './Setting.vue'
import Implementation from './Implementation.vue'
export default {
    components: {
        Setting,
        Implementation
    },
}
</script>

<style>
@import url('https://fonts.googleapis.com/css2?family=Uchen&display=swap');
html, body {
    margin: 0;
    padding: 0;
    overflow: hidden;
    overscroll-behavior-y: contain;
    font-family: "Uchen", serif;
}
</style>
