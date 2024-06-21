<template>
  <figure class='clock--outer'>
    <div class='clock--inner'>
      <div ref="hour" class='hand hour' :style="{'transform' : `rotate(${hoursRotate}deg)`}"></div>
      <div ref="min" class='hand min' :style="{'transform' : `rotate(${minutesRotate}deg)`}"></div>
      <div ref="sec" class='hand sec' :style="{'transform' : `rotate(${secondsRotate}deg)`}"></div>
    </div>
  </figure>
</template>

<script>

// Built with assistance from ~Foolish Developer~
// https://dev.to/code_mystery/simple-analog-clock-using-html-css-javascript-2c6a

export default {
  name:'clock',
  data(){
    return{
      secondsRotate: 0,
      minutesRotate: 0,
      hoursRotate: 0
    }
  },
  methods:{
    tick(){
      const now = new Date();
      const seconds = now.getSeconds();
      this.secondsRotate = (seconds/60 * 360)+90;

      const minutes = now.getMinutes();
      this.minutesRotate = (minutes/60 * 360)+90;

      const hours = now.getHours();
      this.hoursRotate = ((hours / 12) * 360) + ((minutes/60)*30) + 90;
    }
  },
  mounted(){
    setInterval(this.tick, 1000);

    this.tick()
  }
}
</script>

<style scoped>
.clock--outer{
  background-image: url('/assets/clock.png');
  image-rendering: auto;
  background-size: cover;
}
.clock--inner{
  width: 100px;
  height: 100px;
  position: relative;
  padding: 1rem;
}

.hand{
  width: 50%;
  right: 50%;
  height: 2px;
  background-color: #f1f1f1;
  position: absolute;
  top: 50%;
  transform-origin: 100%;
  transform:rotate(90deg);
  transition-timing-function: cubic-bezier(0.1, 2.7, 0.58, 1); 
}

.hand.hour{
  width: 25%;
  z-index: 3;
}

.hand.min{
  width: 30%;
  z-index: 10;
}

.hand.sec{
  width: 35%;
  height: 1px;
  background-color: red;
  z-index: 15;
}


</style>