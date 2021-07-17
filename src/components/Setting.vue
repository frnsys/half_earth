<template>
    <h1 ref="button" style="color:#fff;text-align:center;" @click="start()">click</h1>
    <div class="setting">
        <div class="background" ref="background" :style="{backgroundImage:`url('${background}')`}"></div>
    </div>
    <div class="title" ref="title">The 12th Planning Conference</div>
    <div class="location" ref="location">The Klamath-Siskiyou Redwood Preserve</div>
    <div class="date" ref="date">June 28th, 2080</div>

    <div class="dialogue" ref="dialogue">
        <div>
            <div class="speech" ref="speech"></div>
            <img src="assets/characters/test.png" />
        </div>
    </div>
</template>

<script>
import {Howl, Howler} from 'howler';

const speechSound = new Howl({
    src: ['assets/speech.mp3'],
    loop: true,
    volume: 0.8
});

// <https://camwiegert.github.io/baffle/#demo> Use for gosplant speech?
function revealText(el, text, speed, cb) {
    speechSound.play();
    let revealed = '';
    const chars = text.split('');
    const interval = setInterval(() => {
        // Have to keep the revealed text
        // separate from innerText
        // b/c innerText will strip trailing spaces
        revealed += chars.shift();
        el.innerText = revealed;
        if (chars.length == 0) {
            clearInterval(interval);
            speechSound.stop();
            if (cb) cb();
        }
    }, 100/speed);
}

export default {
    props: {
        background: String,
        audio: String
    },
    data: () => {
        return {
            sound: null
        };
    },
    mounted() {
        // Requires user interaction to play
        this.sound = new Howl({
            src: [this.audio],
            loop: true,
            volume: 0,
        });
    },
    methods: {
        start() {
            this.sound.play();
            this.sound.fade(0, 1, 5000);
            this.$refs.background.style.opacity = 1;
            this.$refs.title.style.opacity = 1;
            // Hacky
            setTimeout(() => {
                this.$refs.location.style.opacity = 1;
                this.$refs.date.style.opacity = 1;
                setTimeout(() => {
                    this.$refs.location.style.opacity = 0;
                    this.$refs.date.style.opacity = 0;
                    this.$refs.title.style.opacity = 0;
                    setTimeout(() => {
                        this.$refs.location.style.display = 'none';
                        this.$refs.date.style.display = 'none';
                        this.$refs.title.style.display = 'none';
                        this.$refs.dialogue.style.display = 'flex';
                        revealText(this.$refs.speech, 'Hello--welcome to the conference!', 1., () => {
                            setTimeout(() => {
                                revealText(this.$refs.speech, 'I hope you have a good time.', 1.);
                            }, 5000);
                        });
                    }, 5000);
                }, 8000);
            }, 5000);

            this.$refs.button.style.display = 'none';
        }
    }
}
</script>

<style scoped>
.setting {
    position: fixed;
    left: 0;
    right: 0;
    top: 0;
    bottom: 0;
    z-index: -1;
    background: #000;
}
.background {
    width: 100%;
    height: 100%;
    background-size: cover;
    background-position: center center;
    background-repeat: no-repeat;
    opacity: 0;
    transition: 5s all;
}

/* test stuff */
.title, .location, .date {
    text-align: center;
    color: #fff;
    opacity: 0;
    transition: 5s all;
    text-shadow: 1px 1px 0px #573305;
    font-size: 2em;
}
.title {
    font-size: 3em;
    margin-top: 3em;
}
.dialogue {
    width: 320px;
    margin: 2em auto;
    text-align: center;
    display: none;
    align-items: center;
    justify-content: space-around;
    height: 100vh;
}
.dialogue img {
    max-width: 100px;
}
.speech {
    padding: 1em;
    background: #F4F3EA;
    border-radius: 0.2em;
    margin-bottom: 1em;
    position: relative;
}
.speech:after {
    content:'';
    position: absolute;
    top: 100%;
    left: 50%;
    margin-left: -12px;
    width: 0;
    height: 0;
    border-top: solid 12px #F4F3EA;
    border-left: solid 12px transparent;
    border-right: solid 12px transparent;
}
</style>
