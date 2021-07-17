<template>
    <Hand :cards="cards" />
</template>

<script>
import Hand from './Hand.vue'
export default {
    components: {
        Hand,
    },
    data() {
        return {
            cards: [{
                id: 0
            }, {
                id: 1
            }, {
                id: 2
            }, {
                id: 3
            }]
        }
    }
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
