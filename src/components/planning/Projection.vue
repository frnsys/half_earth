<template>
<div class="chart"></div>
</template>

<script>
import Chart from 'lib/chart';

export default {
  props: {
    startYear: Number,
    endYear: Number,
    pastValues: Number,
    currentTargetValue: Number,
    finalTargetValue: Number
  },
  mounted() {
    this.chart = new Chart(this.$el, {
      ranges: this.ranges
    });

    window.addEventListener('resize', () => {
      this.chart.setSize();
      this.renderChart();
    });

    this.renderChart();
  },
  updated() {
    this.chart.ranges = this.ranges;
    this.renderChart();
  },
  computed: {
    ranges() {
      let values = this.pastValues.concat([
        this.currentTargetValue,
        this.finalTargetValue
      ]);
      // Pad out the ranges a bit
      return {
        x: [this.startYear - 2, this.endYear + 2],
        y: [0.9 * Math.min(...values), Math.max(...values) * 1.1]
      };
    }
  },
  methods: {
    renderChart() {
      let currentYear = this.startYear + this.pastValues.length;
      let currentValue = this.pastValues[this.pastValues.length - 1];
      let targetChange = this.currentTargetValue - currentValue;

      let pastPoints = this.pastValues.map((v, i) => ({
        x: this.startYear + i,
        y: v
      }));

      // Project out the current planned change until the end of the game
      // Divide target change by 5 b/c this change will occur over 5 years
      let years = this.endYear - this.startYear + 1;
      let projPoints = [...new Array(years-pastPoints.length)].map((_, i) => ({
        x: this.startYear + pastPoints.length + i,
        y: currentValue + (targetChange/5 * i)
      }));

      this.chart.reset();

      // Draw ticks
      [...new Array(years)].forEach((_, i) => {
        this.chart.drawXTick(
          this.startYear + i,
          i % 5 == 0 ? 10 : 5, // tick size
          i % 5 == 0 ? this.startYear + i : '');
      });

      this.chart.drawHLine(this.finalTargetValue, '#ff0000', {
        text: 'Pre-industrial Level',
        color: '#000000',
      });
      this.chart.drawLine(pastPoints, '#000000');
      this.chart.drawLine(projPoints, '#cccccc');
      this.chart.drawPoint({
        y: currentValue,
        x: currentYear
      }, '#000000', 3);
    }
  }
}
</script>

<style>
.chart {
  width: 100%;
  height: 200px;
  background: #f0f0f0;
}
</style>
