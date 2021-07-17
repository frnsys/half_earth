<template>
    <div id="hud">
        <div id="time">
            <div id="year">2038</div>
            <div id="progress">●●●<span id="current-year">●</span><span id="remaining-years">●</span></div>
        </div>
        <div id="indicators">♚ ♛ ♜ ♝ ♞ </div>
        <div id="menu">Menu</div>
    </div>
    <!-- <Hand :cards="cards" /> -->
    <Globe />
</template>

<script>
import Hand from './Hand.vue'
import Globe from './Globe.vue'
export default {
    components: {
        Hand,
        Globe
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
#hud {
    text-align: center;
    color: #fff;
    background: #222;
    padding: 0.25em 0.5em;
    line-height: 1;
    font-size: 1.5em;
    display: flex;
    justify-content: space-between;
}

#time {
    font-size: 0.8em;
}
#progress {
    color: #484848;
    font-size: 0.4em;
}
#current-year {
    color: #f7f78b;
}
#remaining-years {
    color: #646161;
}
</style>
